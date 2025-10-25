use crate::{
    lighting::APPLY_PRESET,
    stories::{Genre, Story},
    talisman::Talisman,
    telemetry::TelemetryString,
};
use core::fmt::Write;
use defmt::info;
use embassy_net::Stack;

#[embassy_executor::task]
pub(super) async fn task(net_stack: Stack<'static>) -> ! {
    let mut talisman_sub = crate::talisman::TALISMAN_CHOSEN.subscriber().unwrap();
    let telemetry_pub = crate::telemetry::TELEMETRY_QUEUE.publisher().unwrap();
    let lighting_preset_tx = APPLY_PRESET.sender();

    let mut talisman_select_counts = [0usize; Talisman::COUNT];
    let mut story_counts = [0usize; Story::COUNT];

    loop {
        let msg = talisman_sub.next_message_pure().await;

        let talisman = msg.talisman;
        talisman_select_counts[talisman as usize] += 1;

        // Determine the genre
        let genre = match talisman {
            Talisman::M => Genre::Gruesome,  // Embroidered
            Talisman::A => Genre::Dystopian, // Laser ply
            Talisman::K => Genre::Cyberpunk, // 3D printed
            Talisman::E => Genre::Maker,     // Hand crafted
            Talisman::R => Genre::Hacker,    // Lasered blue acrylic
        };
        info!("Selected genre {}", genre);

        // Set the lighting
        lighting_preset_tx.send(crate::lighting::Preset::Genre(genre));

        // Pick the story
        let story = crate::stories::pick_random_story(genre);
        story_counts[story as usize] += 1;
        info!("Selected story {}", story);

        // Place the call
        let filename = story.filename();
        crate::ami::place_call(net_stack, filename).await;

        let mut telem_str = TelemetryString::new();

        // Triggered talisman
        telem_str
            .write_fmt(format_args!(
                "talisman_trigger_count,talisman={:?} value={}\n",
                talisman, talisman_select_counts[talisman as usize],
            ))
            .unwrap();

        // Selected story
        telem_str
            .write_fmt(format_args!(
                "story_select_count,story={filename} value={}\n",
                story_counts[story as usize],
            ))
            .unwrap();

        telemetry_pub.publish(telem_str).await;
    }
}
