use crate::memory::{Boss, Game};
use asr::settings::gui::{Gui, Title};

/// The options to split for most settings.
#[derive(Gui, Clone, Copy)]
pub enum Split {
    /// Split only the first time the setting's event occurs (default).
    FirstTime,

    /// Split every time the setting's event occurs.
    EveryTime,

    /// Never split when the setting's event occurs.
    Never,
}

/// Specific to settings for collectable items.
/// Any splits will occur the first frame Spyro can move after collecting an item.
#[derive(Gui, Clone, Copy)]
pub enum CollectableSplit {
    /// Never split upon the item's collection (default).
    Never,

    /// Only split if it satisfies a [category extension](https://www.speedrun.com/spyrortce)
    /// for collectables.
    ///
    /// - Spyro the Dragon - 80 Dragons
    /// - Spyro 2: Ripto's Rage - 40 Orbs
    /// - Spyro: Year of the Dragon - 149 Eggs
    OnCategoryRequirement,

    /// Always split upon pickup of a collectable.
    EveryCollection,
}

/// Settings one can edit via GUI (where supported) or by editing a split XML file directly.
/// Any enabled level splits will occur upon exiting a level, not upon entry.
#[derive(Gui)]
pub struct Settings {
    /// General
    #[heading_level = 0]
    _title_general: Title,

    /// Reset timer on title screen
    #[default = false]
    pub reset_on_title: bool,

    /// Spyro 1
    #[heading_level = 0]
    _title_s1: Title,

    /// Split on Dragon Collected
    ///
    /// `OnCategoryRequirement` will split after Spyro has collected 80 dragons.
    s1_dragon_collected: CollectableSplit,

    /// Split on Exit
    #[heading_level = 1]
    _title_s1_exit: Title,

    /// Artisans
    s1_artisans: Split,

    /// Stone Hill
    s1_stone_hill: Split,

    /// Dark Hollow
    s1_dark_hollow: Split,

    /// Town Square
    s1_town_square: Split,

    /// Sunny Flight
    s1_sunny_flight: Split,

    /// Toasty
    s1_toasty: Split,

    /// Peace Keepers
    s1_peace_keepers: Split,

    /// Dry Canyon
    s1_dry_canyon: Split,

    /// Cliff Town
    s1_cliff_town: Split,

    /// Ice Cavern
    s1_ice_cavern: Split,

    /// Night Flight
    s1_night_flight: Split,

    /// Doctor Shemp
    s1_doctor_shemp: Split,

    /// Magic Crafters
    s1_magic_crafters: Split,

    /// Alpine Ridge
    s1_alpine_ridge: Split,

    /// High Caves
    s1_high_caves: Split,

    /// Wizard Peak
    s1_wizard_peak: Split,

    /// Crystal Flight
    s1_crystal_flight: Split,

    /// Blowhard
    s1_blowhard: Split,

    /// Beast Makers
    s1_beast_makers: Split,

    /// Terrace Village
    s1_terrace_village: Split,

    /// Misty Bog
    s1_misty_bog: Split,

    /// Tree Tops
    s1_tree_tops: Split,

    /// Wild Flight
    s1_wild_flight: Split,

    /// Metalhead
    s1_metalhead: Split,

    /// Dream Weavers
    s1_dream_weavers: Split,

    /// Dark Passage
    s1_dark_passage: Split,

    /// Lofty Castle
    s1_lofty_castle: Split,

    /// Haunted Towers
    s1_haunted_towers: Split,

    /// Icy Flight
    s1_icy_flight: Split,

    /// Jacques
    s1_jacques: Split,

    /// Gnorc Cove
    s1_gnorc_cove: Split,

    /// Twilight Harbor
    s1_twilight_harbor: Split,

    /// Gnasty Gnorc
    s1_gnasty_gnorc: Split,

    /// Gnasty's Loot
    s1_gnastys_loot: Split,

    /// Spyro 2
    #[heading_level = 0]
    _title_s2: Title,

    /// Split on Ripto Kill
    s2_ripto_kill: Split,

    /// Split on Exit
    #[heading_level = 1]
    _title_s2_exit: Title,

    /// Glimmer
    s2_glimmer: Split,

    /// Idol Springs
    s2_idol_springs: Split,

    /// Colossus
    s2_colossus: Split,

    /// Hurricos
    s2_hurricos: Split,

    /// Sunny Beach
    s2_sunny_beach: Split,

    /// Aquaria Towers
    s2_aquaria_towers: Split,

    /// Crush's Dungeon
    s2_crushs_dungeon: Split,

    /// Ocean Speedway
    s2_ocean_speedway: Split,

    /// Crystal Glacier
    s2_crystal_glacier: Split,

    /// Skelos Badlands
    s2_skelos_badlands: Split,

    /// Zephyr
    s2_zephyr: Split,

    /// Breeze Harbor
    s2_breeze_harbor: Split,

    /// Scorch
    s2_scorch: Split,

    /// Fracture Hills
    s2_fracture_hills: Split,

    /// Magma Cone
    s2_magma_cone: Split,

    /// Shady Oasis
    s2_shady_oasis: Split,

    /// Gulp's Overlook
    s2_gulps_overlook: Split,

    /// Icy Speedway
    s2_icy_speedway: Split,

    /// Metro Speedway
    s2_metro_speedway: Split,

    /// Mystic Marsh
    s2_mystic_marsh: Split,

    /// Cloud Temples
    s2_cloud_temples: Split,

    /// Metropolis
    s2_metropolis: Split,

    /// Robotica Farms
    s2_robotica_farms: Split,

    /// Ripto's Arena
    s2_riptos_arena: Split,

    /// Canyon Speedway
    s2_canyon_speedway: Split,

    /// Dragon Shores
    s2_dragon_shores: Split,

    /// Spyro 3
    #[heading_level = 0]
    _title_s3: Title,

    /// Split on Sorceress Lair Kill
    s3_sorceress_lair_kill: Split,

    /// Split on Sorceress SBR Kill
    s3_sorceress_sbr_kill: Split,

    /// Split on Egg Collected
    ///
    /// `OnCategoryRequirement` will split after Spyro has collected 149 eggs.
    s3_egg_collected: CollectableSplit,

    /// Split on Exit
    #[heading_level = 1]
    _title_s3_exit: Title,

    /// Sunny Villa
    s3_sunny_villa: Split,

    /// Cloud Spires
    s3_cloud_spires: Split,

    /// Molten Crater
    s3_molten_crater: Split,

    /// Seashell Shore
    s3_seashell_shore: Split,

    /// Sheila's Alp
    s3_sheilas_alp: Split,

    /// Mushroom Speedway
    s3_mushroom_speedway: Split,

    /// Buzz's Dungeon
    s3_buzzs_dungeon: Split,

    /// Crawdad Farm
    s3_crawdad_farms: Split,

    /// Icy Peak
    s3_icy_peak: Split,

    /// Enchanted Towers
    s3_enchanted_towers: Split,

    /// Spooky Swamp
    s3_spooky_swamp: Split,

    /// Bamboo Terrace
    s3_bamboo_terrace: Split,

    /// Sgt. Byrd's Base
    s3_sgt_byrds_base: Split,

    /// Country Speedway
    s3_country_speedway: Split,

    /// Spike's Arena
    s3_spikes_arena: Split,

    /// Spider Town
    s3_spider_town: Split,

    /// Lost Fleet
    s3_lost_fleet: Split,

    /// Frozen Altars
    s3_frozen_altars: Split,

    /// Fireworks Factory
    s3_fireworks_factory: Split,

    /// Charmed Ridge
    s3_charmed_ridge: Split,

    /// Bentleys Outpost
    s3_bentleys_outpost: Split,

    /// Honey Speedway
    s3_honey_speedway: Split,

    /// Scorchs Pit
    s3_scorchs_pit: Split,

    /// Starfish Reef
    s3_starfish_reef: Split,

    /// Crystal Islands
    s3_crystal_islands: Split,

    /// Desert Ruins
    s3_desert_ruins: Split,

    /// Haunted Tomb
    s3_haunted_tomb: Split,

    /// Dino Mines
    s3_dino_mines: Split,

    /// Agent 9's Lab
    s3_agent_9s_lab: Split,

    /// Harbor Speedway
    s3_harbor_speedway: Split,

    /// Sorceress's Lair
    s3_sorceresss_lair: Split,

    /// Bugbot Factory
    s3_bugbot_factory: Split,

    /// Super Bonus Round
    s3_super_bonus: Split,
}

impl Settings {
    /// The corresponding [`Split`] setting for a given map.
    pub fn split_on_map_exit(&self, map: &str) -> Split {
        match map {
            "/LS101_ArtisansHome/Maps/" => self.s1_artisans,
            "/LS102_StoneHill/Maps/" => self.s1_stone_hill,
            "/LS103_DarkHollow/Maps/" => self.s1_dark_hollow,
            "/LS104_Townsquare/Maps/" => self.s1_town_square,
            "/LS105_Sunnyflight/Maps/" => self.s1_sunny_flight,
            "/LS106_Toasty/Maps/" => self.s1_toasty,
            "/LS107_PeacekeeperHome/Maps/" => self.s1_peace_keepers,
            "/LS108_DryCanyon/Maps/" => self.s1_dry_canyon,
            "/LS109_CliffTown/Maps/" => self.s1_cliff_town,
            "/LS110_IceCavern/Maps/" => self.s1_ice_cavern,
            "/LS111_NightFlight/Maps/" => self.s1_night_flight,
            "/LS112_DrShemp/Maps/" => self.s1_doctor_shemp,
            "/LS113_MagicHome/Maps/" => self.s1_magic_crafters,
            "/LS114_AlpineRidge/Maps/" => self.s1_alpine_ridge,
            "/LS115_HighCaves/Maps/" => self.s1_high_caves,
            "/LS116_WizardPeak/Maps/" => self.s1_wizard_peak,
            "/LS117_CrystalFlight/Maps/" => self.s1_crystal_flight,
            "/LS118_Blowhard/Maps/" => self.s1_blowhard,
            "/LS119_BeastHome/Maps/" => self.s1_beast_makers,
            "/LS120_TerraceVillage/Maps/" => self.s1_terrace_village,
            "/LS121_MistyBog/Maps/" => self.s1_misty_bog,
            "/LS122_TreeTops/Maps/" => self.s1_tree_tops,
            "/LS123_WildFlight/Maps/" => self.s1_wild_flight,
            "/LS124_MetalHead/Maps/" => self.s1_metalhead,
            "/LS125_DreamWeaverHome/Maps/" => self.s1_dream_weavers,
            "/LS126_DarkPassage/Maps/" => self.s1_dark_passage,
            "/LS127_LoftyCastle/Maps/" => self.s1_lofty_castle,
            "/LS128_HauntedTowers/Maps/" => self.s1_haunted_towers,
            "/LS129_IcyFlight/Maps/" => self.s1_icy_flight,
            "/LS130_Jacques/Maps/" => self.s1_jacques,
            "/LS132_GnorcCove/Maps/" => self.s1_gnorc_cove,
            "/LS133_TwlightHarbour/Maps/" => self.s1_twilight_harbor,
            "/LS134_GnastyGnorc/Maps/" => self.s1_gnasty_gnorc,
            "/LS135_GnastyLoot/Maps/" => self.s1_gnastys_loot,
            "/LS202_Glimmer/Maps/" => self.s2_glimmer,
            "/LS203_IdolSprings/Maps/" => self.s2_idol_springs,
            "/LS204_Colossus/Maps/" => self.s2_colossus,
            "/LS205_Hurricos/Maps/" => self.s2_hurricos,
            "/LS206_SunnyBeach/Maps/" => self.s2_sunny_beach,
            "/LS207_AquariaTowers/Maps/" => self.s2_aquaria_towers,
            "/LS208_CrushsDungeon/Maps/" => self.s2_crushs_dungeon,
            "/LS209_OceanSpeedway/Maps/" => self.s2_ocean_speedway,
            "/LS211_CrystalGlacier/Maps/" => self.s2_crystal_glacier,
            "/LS212_SkelosBadlands/Maps/" => self.s2_skelos_badlands,
            "/LS213_Zephyr/Maps/" => self.s2_zephyr,
            "/LS214_BreezeHarbor/Maps/" => self.s2_breeze_harbor,
            "/LS215_Scorch/Maps/" => self.s2_scorch,
            "/LS216_FractureHills/Maps/" => self.s2_fracture_hills,
            "/LS217_MagmaCone/Maps/" => self.s2_magma_cone,
            "/LS218_ShadyOasis/Maps/" => self.s2_shady_oasis,
            "/LS219_GulpsOverlook/Maps/" => self.s2_gulps_overlook,
            "/LS220_IcySpeedway/Maps/" => self.s2_icy_speedway,
            "/LS221_MetroSpeedway/Maps/" => self.s2_metro_speedway,
            "/LS223_MysticMarsh/Maps/" => self.s2_mystic_marsh,
            "/LS224_CloudTemples/Maps/" => self.s2_cloud_temples,
            "/LS225_Metropolis/Maps/" => self.s2_metropolis,
            "/LS226_RoboticaFarms/Maps/" => self.s2_robotica_farms,
            "/LS227_RiptosArena/Maps/" => self.s2_riptos_arena,
            "/LS228_CanyonSpeedway/Maps/" => self.s2_canyon_speedway,
            "/LS229_DragonShores/Maps/" => self.s2_dragon_shores,
            "/LS302_SunnyVilla/Maps/" => self.s3_sunny_villa,
            "/LS303_CloudSpires/Maps/" => self.s3_cloud_spires,
            "/LS304_MoltenCrater/Maps/" => self.s3_molten_crater,
            "/LS305_SeashellShore/Maps/" => self.s3_seashell_shore,
            "/LS306_SheilasAlp/Maps/" => self.s3_sheilas_alp,
            "/LS307_MushroomSpeedway/Maps/" => self.s3_mushroom_speedway,
            "/LS308_BuzzsDungeon/Maps/" => self.s3_buzzs_dungeon,
            "/LS309_CrawdadFarm/Maps/" => self.s3_crawdad_farms,
            "/LS311_IcyPeak/Maps/" => self.s3_icy_peak,
            "/LS312_EnchantedTowers/Maps/" => self.s3_enchanted_towers,
            "/LS313_SpookySwamp/Maps/" => self.s3_spooky_swamp,
            "/LS314_BambooTerrace/Maps/" => self.s3_bamboo_terrace,
            "/LS315_SgtByrdBase/Maps/" => self.s3_sgt_byrds_base,
            "/LS316_CountrySpeedway/Maps/" => self.s3_country_speedway,
            "/LS317_SpikesArena/Maps/" => self.s3_spikes_arena,
            "/LS318_SpiderTown/Maps/" => self.s3_spider_town,
            "/LS320_LostFleet/Maps/" => self.s3_lost_fleet,
            "/LS321_FrozenAltars/Maps/" => self.s3_frozen_altars,
            "/LS322_FireworksFactory/Maps/" => self.s3_fireworks_factory,
            "/LS323_CharmedRidge/Maps/" => self.s3_charmed_ridge,
            "/LS324_BentleysOutpost/Maps/" => self.s3_bentleys_outpost,
            "/LS325_HoneySpeedway/Maps/" => self.s3_honey_speedway,
            "/LS326_ScorchsPit/Maps/" => self.s3_scorchs_pit,
            "/LS327_StarfishReef/Maps/" => self.s3_starfish_reef,
            "/LS329_CrystalIslands/Maps/" => self.s3_crystal_islands,
            "/LS330_DesertRuins/Maps/" => self.s3_desert_ruins,
            "/LS331_HauntedTomb/Maps/" => self.s3_haunted_tomb,
            "/LS332_DinoMines/Maps/" => self.s3_dino_mines,
            "/LS333_Agent9sLab/Maps/" => self.s3_agent_9s_lab,
            "/LS334_HarborSpeedway/Maps/" => self.s3_harbor_speedway,
            "/LS335_SorceressLair/Maps/" => self.s3_sorceresss_lair,
            "/LS336_BugbotFactory/Maps/" => self.s3_bugbot_factory,
            "/LS337_SuperBonusRound/Maps/" => self.s3_super_bonus,
            _ => Split::Never,
        }
    }

    /// The corresponding [`Split`] setting for a given [`Boss`].
    pub fn split_on_boss_kill(&self, boss: Boss) -> Split {
        match boss {
            Boss::Ripto(_) => self.s2_ripto_kill,
            Boss::SorceressLair(_) => self.s3_sorceress_lair_kill,
            Boss::SorceressSBR(_) => self.s3_sorceress_sbr_kill,
        }
    }

    /// The corresponding [`CollectableSplit`] setting for a given [`Game`].
    pub fn split_on_collectable_collected(&self, game: Game) -> CollectableSplit {
        match game {
            Game::Spyro1 => self.s1_dragon_collected,
            Game::Spyro2 => CollectableSplit::Never, // Orbs not yet supported
            Game::Spyro3 => self.s3_egg_collected,
        }
    }
}
