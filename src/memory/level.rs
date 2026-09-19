//! Levels (or maps) that load during gameplay.

use super::{Memory, PointerPaths};
#[cfg(debug_assertions)]
use asr::timer;
use asr::{Address, Process, string::ArrayWString, watcher::Watcher};

/// Extracts and caches information about what [`Level`]s Spyro has visited.
#[derive(Default)]
pub struct LevelReader {
    level: Watcher<Level>,
}

impl LevelReader {
    /// Updates the current [`Level`].
    /// This should only be called by [`Memory`].
    pub fn update(&mut self, process: &Process, address: Address, paths: &PointerPaths) {
        if let Some(level) = Self::read_level(process, address, paths) {
            self.level.update_infallible(level);
        }
    }

    /// The current [`Level`] Spyro is in, if any.
    #[must_use]
    pub fn current_level(&self) -> Option<Level> {
        Some(self.level.pair?.current)
    }

    /// Returns a [`LevelTransition`] if one has just occurred, [`None`] otherwise.
    ///
    /// Note that some level transitions are considered invalid. These transitions will
    /// never return a value and always return [`None`]. See the source code of [`LevelTransition`]
    /// for more details.
    #[must_use]
    pub fn level_changed(&self) -> Option<LevelTransition> {
        let pair = self.level.pair?;
        LevelTransition::try_new(pair.old, pair.current)
    }

    fn read_level(process: &Process, address: Address, paths: &PointerPaths) -> Option<Level> {
        let level_path = Memory::read::<ArrayWString<256>>(process, address, &paths.level)?;
        let level_path = String::from_utf16(&level_path).ok()?;

        #[cfg(debug_assertions)]
        timer::set_variable("level_path", &level_path);

        Level::from_memory(&level_path)
    }
}

/// Represents the current [`Level`] changing in memory.
pub struct LevelTransition {
    from: Level,
}

impl LevelTransition {
    /// The [`Level`] that was just exited from.
    #[must_use]
    pub const fn from(&self) -> Level {
        self.from
    }

    fn try_new(from: Level, to: Level) -> Option<Self> {
        Self::is_valid(from, to).then_some(Self { from })
    }

    /// Checks that the transition is a proper one given level storage and other shenanigans.
    fn is_valid(from: Level, to: Level) -> bool {
        match from {
            Level::Artisans => to == Level::PeaceKeepers,
            Level::PeaceKeepers => to == Level::MagicCrafters,
            Level::MagicCrafters => to == Level::BeastMakers,
            Level::BeastMakers => to == Level::DreamWeavers,
            Level::DreamWeavers => to == Level::GnastysWorld,
            Level::CrushsDungeon => to == Level::AutumnPlains,
            Level::GulpsOverlook => to == Level::WinterTundra,
            Level::RiptosArena => to == Level::DragonShores,
            _ => from != to,
        }
    }
}

/// The levels in the game Spyro can visit.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Artisans,
    StoneHill,
    DarkHollow,
    TownSquare,
    SunnyFlight,
    Toasty,
    PeaceKeepers,
    DryCanyon,
    CliffTown,
    IceCavern,
    NightFlight,
    DoctorShemp,
    MagicCrafters,
    AlpineRidge,
    HighCaves,
    WizardPeak,
    CrystalFlight,
    Blowhard,
    BeastMakers,
    TerraceVillage,
    MistyBog,
    TreeTops,
    WildFlight,
    Metalhead,
    DreamWeavers,
    DarkPassage,
    LoftyCastle,
    HauntedTowers,
    IcyFlight,
    Jacques,
    GnastysWorld,
    GnorcCove,
    TwilightHarbor,
    GnastyGnorc,
    GnastysLoot,

    SummerForest,
    Glimmer,
    IdolSprings,
    Colossus,
    Hurricos,
    SunnyBeach,
    AquariaTowers,
    CrushsDungeon,
    OceanSpeedway,
    AutumnPlains,
    CrystalGlacier,
    SkelosBadlands,
    Zephyr,
    BreezeHarbor,
    Scorch,
    FractureHills,
    MagmaCone,
    ShadyOasis,
    GulpsOverlook,
    IcySpeedway,
    MetroSpeedway,
    WinterTundra,
    MysticMarsh,
    CloudTemples,
    Metropolis,
    RoboticaFarms,
    RiptosArena,
    CanyonSpeedway,
    DragonShores,

    SunriseSpring,
    SunnyVilla,
    CloudSpires,
    MoltenCrater,
    SeashellShore,
    SheilasAlp,
    MushroomSpeedway,
    BuzzsDungeon,
    CrawdadFarm,
    MiddayGardens,
    IcyPeak,
    EnchantedTowers,
    SpookySwamp,
    BambooTerrace,
    SgtByrdsBase,
    CountrySpeedway,
    SpikesArena,
    SpiderTown,
    EveningLake,
    LostFleet,
    FrozenAltars,
    FireworksFactory,
    CharmedRidge,
    BentleysOutpost,
    HoneySpeedway,
    ScorchsPit,
    StarfishReef,
    MidnightMountain,
    CrystalIslands,
    DesertRuins,
    HauntedTomb,
    DinoMines,
    Agent9sLab,
    HarborSpeedway,
    SorceresssLair,
    BugbotFactory,
    SuperBonusRound,
}

impl Level {
    /// Parses the string representation found in memory into a [`Level`] that's easier to use.
    #[expect(clippy::too_many_lines, reason = "Needs to include all Spyro levels.")]
    #[must_use]
    pub fn from_memory(map_path: &str) -> Option<Self> {
        match map_path {
            "/LS101_ArtisansHome/Maps/" => Some(Self::Artisans),
            "/LS102_StoneHill/Maps/" => Some(Self::StoneHill),
            "/LS103_DarkHollow/Maps/" => Some(Self::DarkHollow),
            "/LS104_Townsquare/Maps/" => Some(Self::TownSquare),
            "/LS105_Sunnyflight/Maps/" => Some(Self::SunnyFlight),
            "/LS106_Toasty/Maps/" => Some(Self::Toasty),
            "/LS107_PeacekeeperHome/Maps/" => Some(Self::PeaceKeepers),
            "/LS108_DryCanyon/Maps/" => Some(Self::DryCanyon),
            "/LS109_CliffTown/Maps/" => Some(Self::CliffTown),
            "/LS110_IceCavern/Maps/" => Some(Self::IceCavern),
            "/LS111_NightFlight/Maps/" => Some(Self::NightFlight),
            "/LS112_DrShemp/Maps/" => Some(Self::DoctorShemp),
            "/LS113_MagicHome/Maps/" => Some(Self::MagicCrafters),
            "/LS114_AlpineRidge/Maps/" => Some(Self::AlpineRidge),
            "/LS115_HighCaves/Maps/" => Some(Self::HighCaves),
            "/LS116_WizardPeak/Maps/" => Some(Self::WizardPeak),
            "/LS117_CrystalFlight/Maps/" => Some(Self::CrystalFlight),
            "/LS118_Blowhard/Maps/" => Some(Self::Blowhard),
            "/LS119_BeastHome/Maps/" => Some(Self::BeastMakers),
            "/LS120_TerraceVillage/Maps/" => Some(Self::TerraceVillage),
            "/LS121_MistyBog/Maps/" => Some(Self::MistyBog),
            "/LS122_TreeTops/Maps/" => Some(Self::TreeTops),
            "/LS123_WildFlight/Maps/" => Some(Self::WildFlight),
            "/LS124_MetalHead/Maps/" => Some(Self::Metalhead),
            "/LS125_DreamWeaverHome/Maps/" => Some(Self::DreamWeavers),
            "/LS126_DarkPassage/Maps/" => Some(Self::DarkPassage),
            "/LS127_LoftyCastle/Maps/" => Some(Self::LoftyCastle),
            "/LS128_HauntedTowers/Maps/" => Some(Self::HauntedTowers),
            "/LS129_IcyFlight/Maps/" => Some(Self::IcyFlight),
            "/LS130_Jacques/Maps/" => Some(Self::Jacques),
            "/LS131_GnastyHome/Maps/" => Some(Self::GnastysWorld),
            "/LS132_GnorcCove/Maps/" => Some(Self::GnorcCove),
            "/LS133_TwlightHarbour/Maps/" => Some(Self::TwilightHarbor),
            "/LS134_GnastyGnorc/Maps/" => Some(Self::GnastyGnorc),
            "/LS135_GnastyLoot/Maps/" => Some(Self::GnastysLoot),

            "/LS201_SummerForest_Home/Maps/" => Some(Self::SummerForest),
            "/LS202_Glimmer/Maps/" => Some(Self::Glimmer),
            "/LS203_IdolSprings/Maps/" => Some(Self::IdolSprings),
            "/LS204_Colossus/Maps/" => Some(Self::Colossus),
            "/LS205_Hurricos/Maps/" => Some(Self::Hurricos),
            "/LS206_SunnyBeach/Maps/" => Some(Self::SunnyBeach),
            "/LS207_AquariaTowers/Maps/" => Some(Self::AquariaTowers),
            "/LS208_CrushsDungeon/Maps/" => Some(Self::CrushsDungeon),
            "/LS209_OceanSpeedway/Maps/" => Some(Self::OceanSpeedway),
            "/LS210_AutumnPlains_Home/Maps/" => Some(Self::AutumnPlains),
            "/LS211_CrystalGlacier/Maps/" => Some(Self::CrystalGlacier),
            "/LS212_SkelosBadlands/Maps/" => Some(Self::SkelosBadlands),
            "/LS213_Zephyr/Maps/" => Some(Self::Zephyr),
            "/LS214_BreezeHarbor/Maps/" => Some(Self::BreezeHarbor),
            "/LS215_Scorch/Maps/" => Some(Self::Scorch),
            "/LS216_FractureHills/Maps/" => Some(Self::FractureHills),
            "/LS217_MagmaCone/Maps/" => Some(Self::MagmaCone),
            "/LS218_ShadyOasis/Maps/" => Some(Self::ShadyOasis),
            "/LS219_GulpsOverlook/Maps/" => Some(Self::GulpsOverlook),
            "/LS220_IcySpeedway/Maps/" => Some(Self::IcySpeedway),
            "/LS221_MetroSpeedway/Maps/" => Some(Self::MetroSpeedway),
            "/LS222_WinterTundra_Home/Maps/" => Some(Self::WinterTundra),
            "/LS223_MysticMarsh/Maps/" => Some(Self::MysticMarsh),
            "/LS224_CloudTemples/Maps/" => Some(Self::CloudTemples),
            "/LS225_Metropolis/Maps/" => Some(Self::Metropolis),
            "/LS226_RoboticaFarms/Maps/" => Some(Self::RoboticaFarms),
            "/LS227_RiptosArena/Maps/" => Some(Self::RiptosArena),
            "/LS228_CanyonSpeedway/Maps/" => Some(Self::CanyonSpeedway),
            "/LS229_DragonShores/Maps/" => Some(Self::DragonShores),

            "/LS301_SunriseSpring_Home/Maps/" => Some(Self::SunriseSpring),
            "/LS302_SunnyVilla/Maps/" => Some(Self::SunnyVilla),
            "/LS303_CloudSpires/Maps/" => Some(Self::CloudSpires),
            "/LS304_MoltenCrater/Maps/" => Some(Self::MoltenCrater),
            "/LS305_SeashellShore/Maps/" => Some(Self::SeashellShore),
            "/LS306_SheilasAlp/Maps/" => Some(Self::SheilasAlp),
            "/LS307_MushroomSpeedway/Maps/" => Some(Self::MushroomSpeedway),
            "/LS308_BuzzsDungeon/Maps/" => Some(Self::BuzzsDungeon),
            "/LS309_CrawdadFarm/Maps/" => Some(Self::CrawdadFarm),
            "/LS310_MiddayGardens_Home/Maps/" => Some(Self::MiddayGardens),
            "/LS311_IcyPeak/Maps/" => Some(Self::IcyPeak),
            "/LS312_EnchantedTowers/Maps/" => Some(Self::EnchantedTowers),
            "/LS313_SpookySwamp/Maps/" => Some(Self::SpookySwamp),
            "/LS314_BambooTerrace/Maps/" => Some(Self::BambooTerrace),
            "/LS315_SgtByrdBase/Maps/" => Some(Self::SgtByrdsBase),
            "/LS316_CountrySpeedway/Maps/" => Some(Self::CountrySpeedway),
            "/LS317_SpikesArena/Maps/" => Some(Self::SpikesArena),
            "/LS318_SpiderTown/Maps/" => Some(Self::SpiderTown),
            "/LS319_EveningLake_Home/Maps/" => Some(Self::EveningLake),
            "/LS320_LostFleet/Maps/" => Some(Self::LostFleet),
            "/LS321_FrozenAltars/Maps/" => Some(Self::FrozenAltars),
            "/LS322_FireworksFactory/Maps/" => Some(Self::FireworksFactory),
            "/LS323_CharmedRidge/Maps/" => Some(Self::CharmedRidge),
            "/LS324_BentleysOutpost/Maps/" => Some(Self::BentleysOutpost),
            "/LS325_HoneySpeedway/Maps/" => Some(Self::HoneySpeedway),
            "/LS326_ScorchsPit/Maps/" => Some(Self::ScorchsPit),
            "/LS327_StarfishReef/Maps/" => Some(Self::StarfishReef),
            "/LS328_MidnightMountain_Home/Maps/" => Some(Self::MidnightMountain),
            "/LS329_CrystalIslands/Maps/" => Some(Self::CrystalIslands),
            "/LS330_DesertRuins/Maps/" => Some(Self::DesertRuins),
            "/LS331_HauntedTomb/Maps/" => Some(Self::HauntedTomb),
            "/LS332_DinoMines/Maps/" => Some(Self::DinoMines),
            "/LS333_Agent9sLab/Maps/" => Some(Self::Agent9sLab),
            "/LS334_HarborSpeedway/Maps/" => Some(Self::HarborSpeedway),
            "/LS335_SorceressLair/Maps/" => Some(Self::SorceresssLair),
            "/LS336_BugbotFactory/Maps/" => Some(Self::BugbotFactory),
            "/LS337_SuperBonusRound/Maps/" => Some(Self::SuperBonusRound),
            _ => None,
        }
    }
}
