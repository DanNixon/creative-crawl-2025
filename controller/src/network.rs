use core::cell::RefCell;
use defmt::{info, warn};
use embassy_embedded_hal::shared_bus::asynch::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources, StaticConfigV4};
use embassy_net_wiznet::{chip::W5500, Device, Runner, State};
use embassy_rp::{
    clocks::RoscRng,
    gpio::{Input, Level, Output, Pull},
    peripherals::SPI0,
    spi::Spi,
};
use embassy_sync::{
    blocking_mutex::{raw::CriticalSectionRawMutex, CriticalSectionMutex},
    mutex::Mutex,
};
use embassy_time::{Duration, Instant};
use static_cell::StaticCell;

pub(super) async fn init(r: crate::EthernetResources, spawner: Spawner) -> Stack<'static> {
    let mut spi_config = embassy_rp::spi::Config::default();
    spi_config.frequency = 50_000_000;
    spi_config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
    spi_config.polarity = embassy_rp::spi::Polarity::IdleHigh;

    let spi = Spi::new(
        r.spi,
        r.clk,
        r.mosi,
        r.miso,
        r.tx_dma,
        r.rx_dma,
        spi_config.clone(),
    );

    static SPI: StaticCell<
        Mutex<CriticalSectionRawMutex, Spi<'static, SPI0, embassy_rp::spi::Async>>,
    > = StaticCell::new();
    let spi = SPI.init(Mutex::new(spi));

    let cs = Output::new(r.cs_pin, Level::High);
    let device = SpiDeviceWithConfig::new(spi, cs, spi_config);

    let w5500_int = Input::new(r.int_pin, Pull::Up);
    let w5500_reset = Output::new(r.rst_pin, Level::High);

    let mac_addr = [0x00, 0x08, 0xDC, 0x52, 0xD8, 0x75];

    static STATE: StaticCell<State<8, 8>> = StaticCell::new();
    let state = STATE.init(State::<8, 8>::new());

    let (device, runner) = embassy_net_wiznet::new(mac_addr, state, device, w5500_int, w5500_reset)
        .await
        .unwrap();

    spawner.must_spawn(ethernet_task(runner));

    let mut rng = RoscRng;

    static RESOURCES: StaticCell<StackResources<8>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        device,
        Config::dhcpv4(Default::default()),
        RESOURCES.init(StackResources::<8>::new()),
        rng.next_u64(),
    );
    spawner.must_spawn(net_task(runner));

    spawner.must_spawn(connection_monitor_task(stack));

    stack
}

type EthernetSpi = SpiDeviceWithConfig<
    'static,
    CriticalSectionRawMutex,
    Spi<'static, SPI0, embassy_rp::spi::Async>,
    Output<'static>,
>;

#[embassy_executor::task]
async fn ethernet_task(
    runner: Runner<'static, W5500, EthernetSpi, Input<'static>, Output<'static>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device<'static>>) -> ! {
    runner.run().await
}

pub(crate) static MONITORED_CONFIG_STATE: CriticalSectionMutex<RefCell<MonitoredConfigState>> =
    CriticalSectionMutex::new(RefCell::new(MonitoredConfigState {
        last_changed: None,
        dhcp4_config: None,
    }));

#[derive(Clone, Default)]
pub(crate) struct MonitoredConfigState {
    last_changed: Option<Instant>,
    dhcp4_config: Option<StaticConfigV4>,
}

impl MonitoredConfigState {
    pub(crate) fn age(&self) -> Duration {
        Instant::now() - self.last_changed.unwrap_or(Instant::MIN)
    }
}

#[embassy_executor::task]
async fn connection_monitor_task(stack: Stack<'static>) -> ! {
    loop {
        info!("Waiting for DHCP");
        stack.wait_config_up().await;
        info!("DHCP is now up");

        let config = stack.config_v4().unwrap();
        MONITORED_CONFIG_STATE.lock(|v| {
            let mut state = v.borrow_mut();
            state.last_changed.replace(Instant::now());
            state.dhcp4_config.replace(config);
        });

        stack.wait_config_down().await;
        warn!("Network down, reconnecting");
        MONITORED_CONFIG_STATE.lock(|v| {
            let mut state = v.borrow_mut();
            state.last_changed.replace(Instant::now());
            state.dhcp4_config.take();
        });
    }
}
