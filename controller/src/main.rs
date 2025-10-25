#![no_std]
#![no_main]

mod ami;
mod door;
mod lighting;
mod logic;
mod network;
mod status;
mod stories;
mod talisman;
mod telemetry;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{peripherals, Peri};
use panic_probe as _;

assign_resources::assign_resources! {
    ethernet: EthernetResources {
        miso: PIN_16,
        mosi: PIN_19,
        clk: PIN_18,
        spi: SPI0,
        tx_dma: DMA_CH0,
        rx_dma: DMA_CH1,
        cs_pin: PIN_17,
        int_pin: PIN_21,
        rst_pin: PIN_20,
    },
    door: DoorResources {
        switch: PIN_15,
    },
    talisman: TalismanResources {
        m_switch: PIN_14,
        a_switch: PIN_13,
        k_switch: PIN_12,
        e_switch: PIN_11,
        r_switch: PIN_10,
    },
    status: StatusResources {
        watchdog: WATCHDOG,
        led: PIN_25,
    },
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    talisman::Talisman::verify();
    stories::Story::verify();

    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);

    spawner.must_spawn(status::task(r.status));

    let net_stack = network::init(r.ethernet, spawner).await;

    spawner.must_spawn(telemetry::task(net_stack));

    spawner.must_spawn(door::task(r.door));
    talisman::init(r.talisman, spawner);

    spawner.must_spawn(logic::task(net_stack));
    spawner.must_spawn(lighting::task(net_stack));
}
