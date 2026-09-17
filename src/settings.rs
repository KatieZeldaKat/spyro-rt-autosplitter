//! Settings for the auto-splitter that can be modified in `LiveSplit`.

use crate::memory::{boss::Boss, collectible::Collectible, level::Level};
use asr::settings::gui::{Gui, Title};

const DRAGON_CATEGORY_REQUIREMENT: u8 = 80;
const EGG_CATEGORY_REQUIREMENT: u8 = 149;

/// Defines when the timer should split when exiting a [`Level`].
#[derive(Gui)]
pub enum LevelExit {
    /// Never
    ///
    /// Never split on level exit.
    Never,

    /// First Exit
    ///
    /// Split only the first time exiting the level.
    #[default]
    FirstExit,

    /// Always
    ///
    /// Always split on level exit.
    Always,
}

/// Defines when the timer should split when defeating a [`Boss`].
#[derive(Gui)]
pub enum BossDefeat {
    /// Never
    ///
    /// Never split on boss defeat.
    Never,

    /// First Defeat
    ///
    /// Split only the first time defeating the boss.
    #[default]
    FirstDefeat,

    /// Always
    ///
    /// Always split on boss defeat.
    Always,
}

/// Defines when the timer should split when earning a [`Collectible`].
#[derive(Gui)]
enum CollectibleEarned {
    /// Never
    ///
    /// Never split on collectible earned.
    #[default]
    Never,

    /// Category
    ///
    /// Split on based on the category's requirements:
    ///
    /// - Spyro the Dragon -> 80 Dragons
    /// - Spyro: Year of the Dragon -> 149 Eggs
    Category,

    /// Always
    ///
    /// Always split on collectible earned.
    Always,
}

/// Defines all possible settings for the auto-splitter.
#[derive(Gui)]
pub struct Settings {
    /// Spyro 1
    #[heading_level = 0]
    _title_s1: Title,

    /// Split on Dragon Rescued
    dragon_rescued: CollectibleEarned,

    /// Split on Level Exit
    #[heading_level = 1]
    _title_s1_exit: Title,

    /// Artisans
    artisans: LevelExit,

    /// Stone Hill
    stone_hill: LevelExit,

    /// Dark Hollow
    dark_hollow: LevelExit,

    /// Town Square
    town_square: LevelExit,

    /// Sunny Flight
    sunny_flight: LevelExit,

    /// Toasty
    toasty: LevelExit,

    /// Peace Keepers
    peace_keepers: LevelExit,

    /// Dry Canyon
    dry_canyon: LevelExit,

    /// Cliff Town
    cliff_town: LevelExit,

    /// Ice Cavern
    ice_cavern: LevelExit,

    /// Night Flight
    night_flight: LevelExit,

    /// Doctor Shemp
    doctor_shemp: LevelExit,

    /// Magic Crafters
    magic_crafters: LevelExit,

    /// Alpine Ridge
    alpine_ridge: LevelExit,

    /// High Caves
    high_caves: LevelExit,

    /// Wizard Peak
    wizard_peak: LevelExit,

    /// Crystal Flight
    crystal_flight: LevelExit,

    /// Blowhard
    blowhard: LevelExit,

    /// Beast Makers
    beast_makers: LevelExit,

    /// Terrace Village
    terrace_village: LevelExit,

    /// Misty Bog
    misty_bog: LevelExit,

    /// Tree Tops
    tree_tops: LevelExit,

    /// Wild Flight
    wild_flight: LevelExit,

    /// Metalhead
    metalhead: LevelExit,

    /// Dream Weavers
    dream_weavers: LevelExit,

    /// Dark Passage
    dark_passage: LevelExit,

    /// Lofty Castle
    lofty_castle: LevelExit,

    /// Haunted Towers
    haunted_towers: LevelExit,

    /// Icy Flight
    icy_flight: LevelExit,

    /// Jacques
    jacques: LevelExit,

    /// Gnorc Cove
    gnorc_cove: LevelExit,

    /// Twilight Harbor
    twilight_harbor: LevelExit,

    /// Gnasty Gnorc
    gnasty_gnorc: LevelExit,

    /// Gnasty's Loot
    gnastys_loot: LevelExit,

    /// Spyro 2
    #[heading_level = 0]
    _title_s2: Title,

    /// Split on Ripto Defeat
    ripto_defeated: BossDefeat,

    /// Split on Level Exit
    #[heading_level = 1]
    _title_s2_exit: Title,

    /// Glimmer
    glimmer: LevelExit,

    /// Idol Springs
    idol_springs: LevelExit,

    /// Colossus
    colossus: LevelExit,

    /// Hurricos
    hurricos: LevelExit,

    /// Sunny Beach
    sunny_beach: LevelExit,

    /// Aquaria Towers
    aquaria_towers: LevelExit,

    /// Crush's Dungeon
    crushs_dungeon: LevelExit,

    /// Ocean Speedway
    ocean_speedway: LevelExit,

    /// Crystal Glacier
    crystal_glacier: LevelExit,

    /// Skelos Badlands
    skelos_badlands: LevelExit,

    /// Zephyr
    zephyr: LevelExit,

    /// Breeze Harbor
    breeze_harbor: LevelExit,

    /// Scorch
    scorch: LevelExit,

    /// Fracture Hills
    fracture_hills: LevelExit,

    /// Magma Cone
    magma_cone: LevelExit,

    /// Shady Oasis
    shady_oasis: LevelExit,

    /// Gulp's Overlook
    gulps_overlook: LevelExit,

    /// Icy Speedway
    icy_speedway: LevelExit,

    /// Metro Speedway
    metro_speedway: LevelExit,

    /// Mystic Marsh
    mystic_marsh: LevelExit,

    /// Cloud Temples
    cloud_temples: LevelExit,

    /// Metropolis
    metropolis: LevelExit,

    /// Robotica Farms
    robotica_farms: LevelExit,

    /// Ripto's Arena
    riptos_arena: LevelExit,

    /// Canyon Speedway
    canyon_speedway: LevelExit,

    /// Dragon Shores
    dragon_shores: LevelExit,

    /// Spyro 3
    #[heading_level = 0]
    _title_s3: Title,

    /// Split on Egg Rescued
    egg_rescued: CollectibleEarned,

    /// Split on Sorceress Lair Defeat
    sorceress_lair_defeated: BossDefeat,

    /// Split on Sorceress SBR Defeat
    sorceress_sbr_defeated: BossDefeat,

    /// Split on Level Exit
    #[heading_level = 1]
    _title_s3_exit: Title,

    /// Sunny Villa
    sunny_villa: LevelExit,

    /// Cloud Spires
    cloud_spires: LevelExit,

    /// Molten Crater
    molten_crater: LevelExit,

    /// Seashell Shore
    seashell_shore: LevelExit,

    /// Sheila's Alp
    sheilas_alp: LevelExit,

    /// Mushroom Speedway
    mushroom_speedway: LevelExit,

    /// Buzz's Dungeon
    buzzs_dungeon: LevelExit,

    /// Crawdad Farm
    crawdad_farm: LevelExit,

    /// Icy Peak
    icy_peak: LevelExit,

    /// Enchanted Towers
    enchanted_towers: LevelExit,

    /// Spooky Swamp
    spooky_swamp: LevelExit,

    /// Bamboo Terrace
    bamboo_terrace: LevelExit,

    /// Sgt. Byrd's Base
    sgt_byrds_base: LevelExit,

    /// Country Speedway
    country_speedway: LevelExit,

    /// Spike's Arena
    spikes_arena: LevelExit,

    /// Spider Town
    spider_town: LevelExit,

    /// Lost Fleet
    lost_fleet: LevelExit,

    /// Frozen Altars
    frozen_altars: LevelExit,

    /// Fireworks Factory
    fireworks_factory: LevelExit,

    /// Charmed Ridge
    charmed_ridge: LevelExit,

    /// Bentley's Outpost
    bentleys_outpost: LevelExit,

    /// Honey Speedway
    honey_speedway: LevelExit,

    /// Scorch's Pit
    scorchs_pit: LevelExit,

    /// Starfish Reef
    starfish_reef: LevelExit,

    /// Crystal Islands
    crystal_islands: LevelExit,

    /// Desert Ruins
    desert_ruins: LevelExit,

    /// Haunted Tomb
    haunted_tomb: LevelExit,

    /// Dino Mines
    dino_mines: LevelExit,

    /// Agent 9's Lab
    agent_9s_lab: LevelExit,

    /// Harbor Speedway
    harbor_speedway: LevelExit,

    /// Sorceress's Lair
    sorceresss_lair: LevelExit,

    /// Bugbot Factory
    bugbot_factory: LevelExit,

    /// Super Bonus Round
    super_bonus_round: LevelExit,
}

impl Settings {
    /// Takes a [`Collectible`] and returns `true` if the timer should split, `false` otherwise.
    #[must_use]
    pub const fn get_split_on_collectible(&self, collectible: Collectible) -> bool {
        match collectible {
            Collectible::Dragon(dragon_count) => match self.dragon_rescued {
                CollectibleEarned::Never => false,
                CollectibleEarned::Category => dragon_count == DRAGON_CATEGORY_REQUIREMENT,
                CollectibleEarned::Always => true,
            },
            Collectible::Egg(egg_count) => match self.egg_rescued {
                CollectibleEarned::Never => false,
                CollectibleEarned::Category => egg_count == EGG_CATEGORY_REQUIREMENT,
                CollectibleEarned::Always => true,
            },
        }
    }

    /// Takes a [`Boss`] and returns its corresponding [`BossDefeat`] setting.
    #[must_use]
    pub const fn get_boss_defeat_setting(&self, boss: Boss) -> &BossDefeat {
        match boss {
            Boss::Ripto => &self.ripto_defeated,
            Boss::SorceressLair => &self.sorceress_lair_defeated,
            Boss::SorceressSbr => &self.sorceress_sbr_defeated,
        }
    }

    /// Takes a [`Level`] and returns its corresponding [`LevelExit`] setting.
    ///
    /// Note that not all [`Level`]s are included, and some are restricted to only split
    /// on valid transitions. See the source code of
    /// [`LevelTransition`](crate::memory::level::LevelTransition) for more info.
    #[expect(clippy::too_many_lines, reason = "Needs to include all Spyro levels.")]
    #[must_use]
    pub const fn get_level_exit_setting(&self, level: Level) -> &LevelExit {
        match level {
            Level::Artisans => &self.artisans,
            Level::StoneHill => &self.stone_hill,
            Level::DarkHollow => &self.dark_hollow,
            Level::TownSquare => &self.town_square,
            Level::SunnyFlight => &self.sunny_flight,
            Level::Toasty => &self.toasty,
            Level::PeaceKeepers => &self.peace_keepers,
            Level::DryCanyon => &self.dry_canyon,
            Level::CliffTown => &self.cliff_town,
            Level::IceCavern => &self.ice_cavern,
            Level::NightFlight => &self.night_flight,
            Level::DoctorShemp => &self.doctor_shemp,
            Level::MagicCrafters => &self.magic_crafters,
            Level::AlpineRidge => &self.alpine_ridge,
            Level::HighCaves => &self.high_caves,
            Level::WizardPeak => &self.wizard_peak,
            Level::CrystalFlight => &self.crystal_flight,
            Level::Blowhard => &self.blowhard,
            Level::BeastMakers => &self.beast_makers,
            Level::TerraceVillage => &self.terrace_village,
            Level::MistyBog => &self.misty_bog,
            Level::TreeTops => &self.tree_tops,
            Level::WildFlight => &self.wild_flight,
            Level::Metalhead => &self.metalhead,
            Level::DreamWeavers => &self.dream_weavers,
            Level::DarkPassage => &self.dark_passage,
            Level::LoftyCastle => &self.lofty_castle,
            Level::HauntedTowers => &self.haunted_towers,
            Level::IcyFlight => &self.icy_flight,
            Level::Jacques => &self.jacques,
            Level::GnorcCove => &self.gnorc_cove,
            Level::TwilightHarbor => &self.twilight_harbor,
            Level::GnastyGnorc => &self.gnasty_gnorc,
            Level::GnastysLoot => &self.gnastys_loot,
            Level::Glimmer => &self.glimmer,
            Level::IdolSprings => &self.idol_springs,
            Level::Colossus => &self.colossus,
            Level::Hurricos => &self.hurricos,
            Level::SunnyBeach => &self.sunny_beach,
            Level::AquariaTowers => &self.aquaria_towers,
            Level::CrushsDungeon => &self.crushs_dungeon,
            Level::OceanSpeedway => &self.ocean_speedway,
            Level::CrystalGlacier => &self.crystal_glacier,
            Level::SkelosBadlands => &self.skelos_badlands,
            Level::Zephyr => &self.zephyr,
            Level::BreezeHarbor => &self.breeze_harbor,
            Level::Scorch => &self.scorch,
            Level::FractureHills => &self.fracture_hills,
            Level::MagmaCone => &self.magma_cone,
            Level::ShadyOasis => &self.shady_oasis,
            Level::GulpsOverlook => &self.gulps_overlook,
            Level::IcySpeedway => &self.icy_speedway,
            Level::MetroSpeedway => &self.metro_speedway,
            Level::MysticMarsh => &self.mystic_marsh,
            Level::CloudTemples => &self.cloud_temples,
            Level::Metropolis => &self.metropolis,
            Level::RoboticaFarms => &self.robotica_farms,
            Level::RiptosArena => &self.riptos_arena,
            Level::CanyonSpeedway => &self.canyon_speedway,
            Level::DragonShores => &self.dragon_shores,
            Level::SunnyVilla => &self.sunny_villa,
            Level::CloudSpires => &self.cloud_spires,
            Level::MoltenCrater => &self.molten_crater,
            Level::SeashellShore => &self.seashell_shore,
            Level::SheilasAlp => &self.sheilas_alp,
            Level::MushroomSpeedway => &self.mushroom_speedway,
            Level::BuzzsDungeon => &self.buzzs_dungeon,
            Level::CrawdadFarm => &self.crawdad_farm,
            Level::IcyPeak => &self.icy_peak,
            Level::EnchantedTowers => &self.enchanted_towers,
            Level::SpookySwamp => &self.spooky_swamp,
            Level::BambooTerrace => &self.bamboo_terrace,
            Level::SgtByrdsBase => &self.sgt_byrds_base,
            Level::CountrySpeedway => &self.country_speedway,
            Level::SpikesArena => &self.spikes_arena,
            Level::SpiderTown => &self.spider_town,
            Level::LostFleet => &self.lost_fleet,
            Level::FrozenAltars => &self.frozen_altars,
            Level::FireworksFactory => &self.fireworks_factory,
            Level::CharmedRidge => &self.charmed_ridge,
            Level::BentleysOutpost => &self.bentleys_outpost,
            Level::HoneySpeedway => &self.honey_speedway,
            Level::ScorchsPit => &self.scorchs_pit,
            Level::StarfishReef => &self.starfish_reef,
            Level::CrystalIslands => &self.crystal_islands,
            Level::DesertRuins => &self.desert_ruins,
            Level::HauntedTomb => &self.haunted_tomb,
            Level::DinoMines => &self.dino_mines,
            Level::Agent9sLab => &self.agent_9s_lab,
            Level::HarborSpeedway => &self.harbor_speedway,
            Level::SorceresssLair => &self.sorceresss_lair,
            Level::BugbotFactory => &self.bugbot_factory,
            Level::SuperBonusRound => &self.super_bonus_round,
            Level::GnastysWorld
            | Level::SummerForest
            | Level::AutumnPlains
            | Level::WinterTundra
            | Level::SunriseSpring
            | Level::MiddayGardens
            | Level::EveningLake
            | Level::MidnightMountain => &LevelExit::Never,
        }
    }
}
