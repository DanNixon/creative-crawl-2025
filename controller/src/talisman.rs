use crate::TalismanResources;
use defmt::{debug, info, Format};
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::Timer;

#[derive(Debug, Format, Clone)]
pub(crate) struct TalismanChosenEvent {
    pub(crate) talisman: Talisman,
}

#[derive(Debug, Format, Clone, Copy)]
pub(crate) enum Talisman {
    M,
    A,
    K,
    E,
    R,
}

impl Talisman {
    pub(super) fn verify() {
        let a = [7u8; Self::COUNT];
        assert_eq!(a[Self::R as usize], 7);
    }

    pub(crate) const COUNT: usize = 5;
}

pub(crate) static TALISMAN_CHOSEN: PubSubChannel<
    CriticalSectionRawMutex,
    TalismanChosenEvent,
    3,
    2,
    5,
> = PubSubChannel::new();

pub(super) fn init(r: TalismanResources, spawner: Spawner) {
    let m_input = Input::new(r.m_switch, Pull::Up);
    let a_input = Input::new(r.a_switch, Pull::Up);
    let k_input = Input::new(r.k_switch, Pull::Up);
    let e_input = Input::new(r.e_switch, Pull::Up);
    let r_input = Input::new(r.r_switch, Pull::Up);

    spawner.must_spawn(talisman_task(m_input, Talisman::M));
    spawner.must_spawn(talisman_task(a_input, Talisman::A));
    spawner.must_spawn(talisman_task(k_input, Talisman::K));
    spawner.must_spawn(talisman_task(e_input, Talisman::E));
    spawner.must_spawn(talisman_task(r_input, Talisman::R));
}

#[embassy_executor::task(pool_size = 5)]
async fn talisman_task(mut input: Input<'static>, talisman: Talisman) -> ! {
    let event_pub = TALISMAN_CHOSEN.publisher().unwrap();

    loop {
        input.wait_for_low().await;
        debug!("{} down", talisman);

        let event = TalismanChosenEvent {
            talisman: talisman.clone(),
        };
        info!("{}", event);
        event_pub.publish(event).await;

        Timer::after_millis(5).await;

        input.wait_for_high().await;
        debug!("{} up", talisman);

        // Wait longer before triggering again
        Timer::after_secs(2).await;
    }
}
