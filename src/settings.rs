use asr::settings::gui::{Gui, Title};

#[derive(Gui)]
pub struct Settings {
    /// General
    #[heading_level = 0]
    _title_general: Title,

    /// Reset timer on title screen
    #[default = false]
    pub reset_on_title: bool,

    /// Split if <15s in a level
    #[default = false]
    pub split_fast_exits: bool,

    /// Spyro 1 Split on Exit
    #[heading_level = 0]
    _title_s1: Title,

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

    /// Spyro 2 Split on Exit
    #[heading_level = 0]
    _title_s2: Title,

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

    /// Spyro 3 Split on Exit
    #[heading_level = 0]
    _title_s3: Title,

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

#[derive(Gui, Clone, Copy)]
pub enum Split {
    FirstTime,
    EveryTime,
    Never,
}

pub fn get_map_split_setting(map: &str, settings: &Settings) -> Split {
    match map {
        "/LS102_StoneHill/Maps/" => settings.s1_stone_hill,
        "/LS103_DarkHollow/Maps/" => settings.s1_dark_hollow,
        "/LS104_Townsquare/Maps/" => settings.s1_town_square,
        "/LS105_Sunnyflight/Maps/" => settings.s1_sunny_flight,
        "/LS106_Toasty/Maps/" => settings.s1_toasty,
        "/LS108_DryCanyon/Maps/" => settings.s1_dry_canyon,
        "/LS109_CliffTown/Maps/" => settings.s1_cliff_town,
        "/LS110_IceCavern/Maps/" => settings.s1_ice_cavern,
        "/LS111_NightFlight/Maps/" => settings.s1_night_flight,
        "/LS112_DrShemp/Maps/" => settings.s1_doctor_shemp,
        "/LS114_AlpineRidge/Maps/" => settings.s1_alpine_ridge,
        "/LS115_HighCaves/Maps/" => settings.s1_high_caves,
        "/LS116_WizardPeak/Maps/" => settings.s1_wizard_peak,
        "/LS117_CrystalFlight/Maps/" => settings.s1_crystal_flight,
        "/LS118_Blowhard/Maps/" => settings.s1_blowhard,
        "/LS120_TerraceVillage/Maps/" => settings.s1_terrace_village,
        "/LS121_MistyBog/Maps/" => settings.s1_misty_bog,
        "/LS122_TreeTops/Maps/" => settings.s1_tree_tops,
        "/LS123_WildFlight/Maps/" => settings.s1_wild_flight,
        "/LS124_MetalHead/Maps/" => settings.s1_metalhead,
        "/LS126_DarkPassage/Maps/" => settings.s1_dark_passage,
        "/LS127_LoftyCastle/Maps/" => settings.s1_lofty_castle,
        "/LS128_HauntedTowers/Maps/" => settings.s1_haunted_towers,
        "/LS129_IcyFlight/Maps/" => settings.s1_icy_flight,
        "/LS130_Jacques/Maps/" => settings.s1_jacques,
        "/LS132_GnorcCove/Maps/" => settings.s1_gnorc_cove,
        "/LS133_TwlightHarbour/Maps/" => settings.s1_twilight_harbor,
        "/LS134_GnastyGnorc/Maps/" => settings.s1_gnasty_gnorc,
        "/LS135_GnastyLoot/Maps/" => settings.s1_gnastys_loot,
        "/LS202_Glimmer/Maps/" => settings.s2_glimmer,
        "/LS203_IdolSprings/Maps/" => settings.s2_idol_springs,
        "/LS204_Colossus/Maps/" => settings.s2_colossus,
        "/LS205_Hurricos/Maps/" => settings.s2_hurricos,
        "/LS206_SunnyBeach/Maps/" => settings.s2_sunny_beach,
        "/LS207_AquariaTowers/Maps/" => settings.s2_aquaria_towers,
        "/LS208_CrushsDungeon/Maps/" => settings.s2_crushs_dungeon,
        "/LS209_OceanSpeedway/Maps/" => settings.s2_ocean_speedway,
        "/LS211_CrystalGlacier/Maps/" => settings.s2_crystal_glacier,
        "/LS212_SkelosBadlands/Maps/" => settings.s2_skelos_badlands,
        "/LS213_Zephyr/Maps/" => settings.s2_zephyr,
        "/LS214_BreezeHarbor/Maps/" => settings.s2_breeze_harbor,
        "/LS215_Scorch/Maps/" => settings.s2_scorch,
        "/LS216_FractureHills/Maps/" => settings.s2_fracture_hills,
        "/LS217_MagmaCone/Maps/" => settings.s2_magma_cone,
        "/LS218_ShadyOasis/Maps/" => settings.s2_shady_oasis,
        "/LS219_GulpsOverlook/Maps/" => settings.s2_gulps_overlook,
        "/LS220_IcySpeedway/Maps/" => settings.s2_icy_speedway,
        "/LS221_MetroSpeedway/Maps/" => settings.s2_metro_speedway,
        "/LS223_MysticMarsh/Maps/" => settings.s2_mystic_marsh,
        "/LS224_CloudTemples/Maps/" => settings.s2_cloud_temples,
        "/LS225_Metropolis/Maps/" => settings.s2_metropolis,
        "/LS226_RoboticaFarms/Maps/" => settings.s2_robotica_farms,
        "/LS227_RiptosArena/Maps/" => settings.s2_riptos_arena,
        "/LS228_CanyonSpeedway/Maps/" => settings.s2_canyon_speedway,
        "/LS229_DragonShores/Maps/" => settings.s2_dragon_shores,
        "/LS302_SunnyVilla/Maps/" => settings.s3_sunny_villa,
        "/LS303_CloudSpires/Maps/" => settings.s3_cloud_spires,
        "/LS304_MoltenCrater/Maps/" => settings.s3_molten_crater,
        "/LS305_SeashellShore/Maps/" => settings.s3_seashell_shore,
        "/LS306_SheilasAlp/Maps/" => settings.s3_sheilas_alp,
        "/LS307_MushroomSpeedway/Maps/" => settings.s3_mushroom_speedway,
        "/LS308_BuzzsDungeon/Maps/" => settings.s3_buzzs_dungeon,
        "/LS309_CrawdadFarm/Maps/" => settings.s3_crawdad_farms,
        "/LS311_IcyPeak/Maps/" => settings.s3_icy_peak,
        "/LS312_EnchantedTowers/Maps/" => settings.s3_enchanted_towers,
        "/LS313_SpookySwamp/Maps/" => settings.s3_spooky_swamp,
        "/LS314_BambooTerrace/Maps/" => settings.s3_bamboo_terrace,
        "/LS315_SgtByrdsBase/Maps/" => settings.s3_sgt_byrds_base,
        "/LS316_CountrySpeedway/Maps/" => settings.s3_country_speedway,
        "/LS317_SpikesArena/Maps/" => settings.s3_spikes_arena,
        "/LS318_SpiderTown/Maps/" => settings.s3_spider_town,
        "/LS320_LostFleet/Maps/" => settings.s3_lost_fleet,
        "/LS321_FrozenAltars/Maps/" => settings.s3_frozen_altars,
        "/LS322_FireworksFactory/Maps/" => settings.s3_fireworks_factory,
        "/LS323_CharmedRidge/Maps/" => settings.s3_charmed_ridge,
        "/LS324_BentleysOutpost/Maps/" => settings.s3_bentleys_outpost,
        "/LS325_HoneySpeedway/Maps/" => settings.s3_honey_speedway,
        "/LS326_ScorchsPit/Maps/" => settings.s3_scorchs_pit,
        "/LS327_StarfishReef/Maps/" => settings.s3_starfish_reef,
        "/LS329_CrystalIslands/Maps/" => settings.s3_crystal_islands,
        "/LS330_DesertRuins/Maps/" => settings.s3_desert_ruins,
        "/LS331_HauntedTomb/Maps/" => settings.s3_haunted_tomb,
        "/LS332_DinoMines/Maps/" => settings.s3_dino_mines,
        "/LS333_Agent9sLab/Maps/" => settings.s3_agent_9s_lab,
        "/LS334_HarborSpeedway/Maps/" => settings.s3_harbor_speedway,
        "/LS335_SorceresssLair/Maps/" => settings.s3_sorceresss_lair,
        "/LS336_BugbotFactory/Maps/" => settings.s3_bugbot_factory,
        "/LS337_SuperBonusRound/Maps/" => settings.s3_super_bonus,
        _ => Split::Never,
    }
}
