use crate::{lighting::APPLY_PRESET, telemetry::TelemetryString, DoorResources};
use defmt::info;
use embassy_rp::gpio::{Input, Pull};
use embassy_time::Timer;

#[embassy_executor::task]
pub(super) async fn task(r: DoorResources) -> ! {
    let mut input = Input::new(r.switch, Pull::Up);

    let lighting_preset_tx = APPLY_PRESET.sender();
    let telemetry_pub = crate::telemetry::TELEMETRY_QUEUE.publisher().unwrap();

    let telem_str_open: TelemetryString = "door value=\"open\"".try_into().unwrap();
    let telem_str_closed: TelemetryString = "door value=\"closed\"".try_into().unwrap();

    loop {
        input.wait_for_low().await;
        info!("Door opened");
        lighting_preset_tx.send(crate::lighting::Preset::DoorOpened);
        telemetry_pub.publish(telem_str_open.clone()).await;

        Timer::after_millis(5).await;

        input.wait_for_high().await;
        info!("Door closed");
        lighting_preset_tx.send(crate::lighting::Preset::DoorClosed);
        telemetry_pub.publish(telem_str_closed.clone()).await;

        Timer::after_millis(5).await;
    }
}
