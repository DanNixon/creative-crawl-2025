use defmt::{debug, Format};
use embassy_rp::clocks::RoscRng;

#[derive(Debug, Format, Clone, Copy)]
pub(crate) enum Story {
    AiGeneratedNightmare,
    Autopilot,
    BackdooredMind,
    BetterThanYou,
    BodyDebt,
    Companion,
    CorePatch,
    FirewallBreach,
    GhostInTheMainframe,
    InWarmBlood,
    Influence,
    InkOfTheAncients,
    LastLightDistrict,
    Mia2p0,
    MirrorChannel,
    NeuralBleed,
    NocturneExe,
    Numbered,
    OptimisationError,
    PatchNotes,
    PrintJobFromHell,
    PrintedThoughts,
    RaspberryPiPanopticon,
    RealityPatch,
    RootAccessToTheAfterlife,
    SkinwalkerFirmware,
    StitchesInTheDark,
    SubconsciousEchos,
    TheBlackoutWards,
    TheCncWhisperer,
    TheCodeThatShouldntExist,
    TheCursedCarvings,
    TheEchoChamber,
    TheEchoLottery,
    TheFeed,
    TheForgottenProtocol,
    TheGearhead,
    TheGrainKnows,
    TheHackspaceNeverClosed,
    TheHauntedLaserCutter,
    TheHumanCncProject,
    TheLaserEngraverRitual,
    TheLightsThatWatch,
    TheLuthiersApprentice,
    TheMakersEcho,
    TheNeonShrine,
    TheNetworkBenethTheSkin,
    TheOracleModel,
    TheRedstonePrison,
    TheReturnBin,
    TheRootKit,
    TheSimulationWakes,
    TheSmartWorkshop,
    TheTether,
    TheWeaversPattern,
    WhisperLoop,
    YourPasswordHasBeenCompromised,
}

impl Story {
    pub(super) fn verify() {
        let a = [7u8; Self::COUNT];
        assert_eq!(a[Self::YourPasswordHasBeenCompromised as usize], 7);
    }

    pub(crate) const COUNT: usize = 57;

    pub(crate) fn filename(&self) -> &'static str {
        match self {
            Story::AiGeneratedNightmare => "ai-generated-nightmare",
            Story::Autopilot => "autopilot",
            Story::BackdooredMind => "backdoored-mind",
            Story::BetterThanYou => "better-than-you",
            Story::BodyDebt => "body-debt",
            Story::Companion => "companion",
            Story::CorePatch => "core-patch",
            Story::FirewallBreach => "firewall-breach",
            Story::GhostInTheMainframe => "ghost-in-the-mainframe",
            Story::InWarmBlood => "in-warm-blood",
            Story::Influence => "influence",
            Story::InkOfTheAncients => "ink-of-the-ancients",
            Story::LastLightDistrict => "last-light-district",
            Story::Mia2p0 => "mia-2.0",
            Story::MirrorChannel => "mirror-channel",
            Story::NeuralBleed => "neural-bleed",
            Story::NocturneExe => "nocturne.exe",
            Story::Numbered => "numbered",
            Story::OptimisationError => "optimisation-error",
            Story::PatchNotes => "patch-notes",
            Story::PrintJobFromHell => "print-job-from-hell",
            Story::PrintedThoughts => "printed-thoughts",
            Story::RaspberryPiPanopticon => "raspberry-pi-panopticon",
            Story::RealityPatch => "reality-patch",
            Story::RootAccessToTheAfterlife => "root-access-to-the-afterlife",
            Story::SkinwalkerFirmware => "skinwalker-firmware",
            Story::StitchesInTheDark => "stitches-in-the-dark",
            Story::SubconsciousEchos => "subconscious-echos",
            Story::TheBlackoutWards => "the-blackout-wards",
            Story::TheCncWhisperer => "the-cnc-whsiperer",
            Story::TheCodeThatShouldntExist => "the-code-that-shouldnt-exist",
            Story::TheCursedCarvings => "the-cursed-carvings",
            Story::TheEchoChamber => "the-echo-chamber",
            Story::TheEchoLottery => "the-echo-lottery",
            Story::TheFeed => "the-feed",
            Story::TheForgottenProtocol => "the-forgotten-protocol",
            Story::TheGearhead => "the-gearhead",
            Story::TheGrainKnows => "the-grain-knows",
            Story::TheHackspaceNeverClosed => "the-hackspace-never-closed",
            Story::TheHauntedLaserCutter => "the-haunted-laser-cutter",
            Story::TheHumanCncProject => "the-human-cnc-project",
            Story::TheLaserEngraverRitual => "the-laser-engraver-ritual",
            Story::TheLightsThatWatch => "the-lights-that-watch",
            Story::TheLuthiersApprentice => "the-luthiers-apprentice",
            Story::TheMakersEcho => "the-makers-echo",
            Story::TheNeonShrine => "the-neon-shrine",
            Story::TheNetworkBenethTheSkin => "the-network-beneth-the-skin",
            Story::TheOracleModel => "the-oracle-model",
            Story::TheRedstonePrison => "the-redstone-prison",
            Story::TheReturnBin => "the-return-bin",
            Story::TheRootKit => "the-root-kit",
            Story::TheSimulationWakes => "the-simulation-wakes",
            Story::TheSmartWorkshop => "the-smart-workshop",
            Story::TheTether => "the-tether",
            Story::TheWeaversPattern => "the-weavers-pattern",
            Story::WhisperLoop => "whisper-loop",
            Story::YourPasswordHasBeenCompromised => "your-password-has-been-compromised",
        }
    }
}

pub(crate) const DYSTOPIAN: [Story; 17] = [
    Story::AiGeneratedNightmare,
    Story::Autopilot,
    Story::BetterThanYou,
    Story::Companion,
    Story::CorePatch,
    Story::Influence,
    Story::LastLightDistrict,
    Story::Mia2p0,
    Story::MirrorChannel,
    Story::OptimisationError,
    Story::TheBlackoutWards,
    Story::TheEchoChamber,
    Story::TheEchoLottery,
    Story::TheFeed,
    Story::TheOracleModel,
    Story::TheSimulationWakes,
    Story::TheSmartWorkshop,
];

pub(crate) const CYBERPUNK: [Story; 12] = [
    Story::BackdooredMind,
    Story::BodyDebt,
    Story::InWarmBlood,
    Story::NeuralBleed,
    Story::NocturneExe,
    Story::PatchNotes,
    Story::RootAccessToTheAfterlife,
    Story::SkinwalkerFirmware,
    Story::SubconsciousEchos,
    Story::TheGearhead,
    Story::TheNetworkBenethTheSkin,
    Story::TheSmartWorkshop,
];

pub(crate) const GRUESOME: [Story; 10] = [
    Story::BodyDebt,
    Story::InWarmBlood,
    Story::InkOfTheAncients,
    Story::NeuralBleed,
    Story::PrintJobFromHell,
    Story::TheCursedCarvings,
    Story::TheGearhead,
    Story::TheGrainKnows,
    Story::TheHumanCncProject,
    Story::TheHauntedLaserCutter,
];

pub(crate) const HACKER: [Story; 11] = [
    Story::FirewallBreach,
    Story::GhostInTheMainframe,
    Story::RealityPatch,
    Story::TheCodeThatShouldntExist,
    Story::TheCursedCarvings,
    Story::TheForgottenProtocol,
    Story::TheRootKit,
    Story::TheRedstonePrison,
    Story::TheTether,
    Story::WhisperLoop,
    Story::YourPasswordHasBeenCompromised,
];

pub(crate) const MAKER: [Story; 18] = [
    Story::InkOfTheAncients,
    Story::Numbered,
    Story::PrintedThoughts,
    Story::RaspberryPiPanopticon,
    Story::StitchesInTheDark,
    Story::TheCncWhisperer,
    Story::TheGrainKnows,
    Story::TheHackspaceNeverClosed,
    Story::TheHauntedLaserCutter,
    Story::TheHumanCncProject,
    Story::TheLaserEngraverRitual,
    Story::TheLightsThatWatch,
    Story::TheLuthiersApprentice,
    Story::TheMakersEcho,
    Story::TheNeonShrine,
    Story::TheReturnBin,
    Story::TheSmartWorkshop,
    Story::TheWeaversPattern,
];

#[derive(Debug, Format, Clone, Copy)]
pub(crate) enum Genre {
    Dystopian,
    Cyberpunk,
    Gruesome,
    Hacker,
    Maker,
}

pub(crate) fn pick_random_story(genre: Genre) -> Story {
    let story_count = match genre {
        Genre::Dystopian => DYSTOPIAN.len(),
        Genre::Cyberpunk => CYBERPUNK.len(),
        Genre::Gruesome => GRUESOME.len(),
        Genre::Hacker => HACKER.len(),
        Genre::Maker => MAKER.len(),
    };

    debug!("Picking a {} story (from {})", genre, story_count);

    let mut rng = RoscRng;

    let i = (rng.next_u64() % story_count as u64) as usize;
    debug!("i = {}", i);

    let story = match genre {
        Genre::Dystopian => DYSTOPIAN[i],
        Genre::Cyberpunk => CYBERPUNK[i],
        Genre::Gruesome => GRUESOME[i],
        Genre::Hacker => HACKER[i],
        Genre::Maker => MAKER[i],
    };

    debug!("Picked {}", story);
    story
}
