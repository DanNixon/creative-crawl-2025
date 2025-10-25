use crate::{network::MONITORED_CONFIG_STATE, telemetry::TelemetryString, StatusResources};
use core::fmt::Write;
use embassy_futures::select::{select, Either};
use embassy_rp::{
    gpio::{Level, Output},
    watchdog::Watchdog,
};
use embassy_time::{Duration, Instant, Ticker};

#[embassy_executor::task]
pub(super) async fn task(r: StatusResources) -> ! {
    let telemetry_pub = crate::telemetry::TELEMETRY_QUEUE.publisher().unwrap();

    let mut led = Output::new(r.led, Level::Low);

    let mut wdt = Watchdog::new(r.watchdog);
    wdt.start(Duration::from_secs(2));

    let mut heartbeat = Ticker::every(Duration::from_millis(500));
    let mut telem_tick = Ticker::every(Duration::from_secs(30));

    loop {
        match select(heartbeat.next(), telem_tick.next()).await {
            Either::First(_) => {
                led.toggle();
                wdt.feed();
            }
            Either::Second(_) => {
                let mut telem_str = TelemetryString::new();

                // Uptime
                telem_str
                    .write_fmt(format_args!(
                        "uptime,unit=ms value={}\n",
                        Instant::now().as_millis()
                    ))
                    .unwrap();

                // Connetion age
                let connection_age = MONITORED_CONFIG_STATE.lock(|state| {
                    let state = state.borrow();
                    state.age()
                });
                telem_str
                    .write_fmt(format_args!(
                        "network_config_age,unit=ms value={}\n",
                        connection_age.as_millis()
                    ))
                    .unwrap();

                telemetry_pub.publish(telem_str).await;
            }
        }
    }
}
