> [!warning]
> 
> This is an archived version of the wiki that used to be enabled on https://github.com/KatieZeldaKat/spyro-rt-autosplitter. It contains outdated information, so do not use it as a source of truth!

---

LiveSplit has a GUI to edit settings, but LiveSplit One platforms currently do not. The only way to edit settings for these is to edit them in the `.lss` file directly. This serves as a guide to add/change settings manually.

`.lss` files are XML files. It is recommended to use software which can make this look more "pretty," such as using [VSCodium](https://vscodium.com/) with the [XML](https://github.com/redhat-developer/vscode-xml) extension.

# `<AutoSplitterSettings />`

At the bottom of the XML, there may exist a tag for `<AutoSplitterSettings />`. It should be just before the ending tag of `</Run>`.  Something like this:

```xml
    <!-- snip -->
    </Segments>
    <AutoSplitterSettings />
</Run>
```

Replace the tag with a more verbose version so there's room to add settings:

```xml
    <!-- snip -->
    </Segments>
    <AutoSplitterSettings>
        <CustomSettings>
            <!-- Settings go here! -->
        </CustomSettings>
    </AutoSplitterSettings>
</Run>
```

For all the examples below, you should place the snippets inside the comment block within `<CustomSettings>` so the auto-splitter can recognize them.

# `bool`

The simplest setting, where its value can either be `True` or `False`. In ASL, this is the only way for settings to be set. Since WASM gives more versatility, the only setting in which a `bool` is useful is for resetting on the title screen:

```xml
<!-- This setting is False by default. -->
<!-- You don't need to add a setting if you want to keep the default. -->
<Setting id="reset_on_title" type="bool">True</Setting>
```

# `Split`

This is the most common setting for the auto-splitter. Most settings are defined by an event which can occur multiple times within a run (i.e. exiting a level or killing a boss). This setting controls when the specified event should trigger a timer split. These options are:

- `FirstTime`
- `EveryTime`
- `Never`

In practice, the XML looks something like this:

```xml
<!-- This setting is FirstTime by default. -->
<!-- You don't need to add a setting if you want to keep the default. -->
<Setting id="s3_molten_crater" type="string" value="EveryTime" />
```

## Level Splits

Any setting to split for a given level will do so upon *exiting* the level. Entering a level will never trigger a split unless the level being exited does so.

There are exceptions for when specific level-transitions should occur, such as level storage and going from one home world to another. You can see these exceptions in `MapCache::is_valid_map_transition()`.

Below is a table of levels with their corresponding `id` values that can be added to the settings like in the `Split` example.

| Level             | Setting                |
| ----------------- | ---------------------- |
| Artisans          | `s1_artisans`          |
| Stone Hill        | `s1_stone_hill`        |
| Dark Hollow       | `s1_dark_hollow`       |
| Town Square       | `s1_town_square`       |
| Sunny Flight      | `s1_sunny_flight`      |
| Toasty            | `s1_toasty`            |
| Peace Keepers     | `s1_peace_keepers`     |
| Dry Canyon        | `s1_dry_canyon`        |
| Cliff Town        | `s1_cliff_town`        |
| Ice Cavern        | `s1_ice_cavern`        |
| Night Flight      | `s1_night_flight`      |
| Doctor Shemp      | `s1_doctor_shemp`      |
| Magic Crafters    | `s1_magic_crafters`    |
| Alpine Ridge      | `s1_alpine_ridge`      |
| High Caves        | `s1_high_caves`        |
| Wizard Peak       | `s1_wizard_peak`       |
| Crystal Flight    | `s1_crystal_flight`    |
| Blowhard          | `s1_blowhard`          |
| Beast Makers      | `s1_beast_makers`      |
| Terrace Village   | `s1_terrace_village`   |
| Misty Bog         | `s1_misty_bog`         |
| Tree Tops         | `s1_tree_tops`         |
| Wild Flight       | `s1_wild_flight`       |
| Metalhead         | `s1_metalhead`         |
| Dream Weavers     | `s1_dream_weavers`     |
| Dark Passage      | `s1_dark_passage`      |
| Lofty Castle      | `s1_lofty_castle`      |
| Haunted Towers    | `s1_haunted_towers`    |
| Icy Flight        | `s1_icy_flight`        |
| Jacques           | `s1_jacques`           |
| Gnorc Cove        | `s1_gnorc_cove`        |
| Twilight Harbor   | `s1_twilight_harbor`   |
| Gnasty Gnorc      | `s1_gnasty_gnorc`      |
| Gnasty's Loot     | `s1_gnastys_loot`      |
| Glimmer           | `s2_glimmer`           |
| Idol Springs      | `s2_idol_springs`      |
| Colossus          | `s2_colossus`          |
| Hurricos          | `s2_hurricos`          |
| Sunny Beach       | `s2_sunny_beach`       |
| Aquaria Towers    | `s2_aquaria_towers`    |
| Crush's Dungeon   | `s2_crushs_dungeon`    |
| Ocean Speedway    | `s2_ocean_speedway`    |
| Crystal Glacier   | `s2_crystal_glacier`   |
| Skelos Badlands   | `s2_skelos_badlands`   |
| Zephyr            | `s2_zephyr`            |
| Breeze Harbor     | `s2_breeze_harbor`     |
| Scorch            | `s2_scorch`            |
| Fracture Hills    | `s2_fracture_hills`    |
| Magma Cone        | `s2_magma_cone`        |
| Shady Oasis       | `s2_shady_oasis`       |
| Gulp's Overlook   | `s2_gulps_overlook`    |
| Icy Speedway      | `s2_icy_speedway`      |
| Metro Speedway    | `s2_metro_speedway`    |
| Mystic Marsh      | `s2_mystic_marsh`      |
| Cloud Temples     | `s2_cloud_temples`     |
| Metropolis        | `s2_metropolis`        |
| Robotica Farms    | `s2_robotica_farms`    |
| Ripto's Arena     | `s2_riptos_arena`      |
| Canyon Speedway   | `s2_canyon_speedway`   |
| Dragon Shores     | `s2_dragon_shores`     |
| Sunny Villa       | `s3_sunny_villa`       |
| Cloud Spires      | `s3_cloud_spires`      |
| Molten Crater     | `s3_molten_crater`     |
| Seashell Shore    | `s3_seashell_shore`    |
| Sheila's Alp      | `s3_sheilas_alp`       |
| Mushroom Speedway | `s3_mushroom_speedway` |
| Buzz's Dungeon    | `s3_buzzs_dungeon`     |
| Crawdad Farm      | `s3_crawdad_farm`      |
| Icy Peak          | `s3_icy_peak`          |
| Enchanted Towers  | `s3_enchanted_towers`  |
| Spooky Swamp      | `s3_spooky_swamp`      |
| Bamboo Terrace    | `s3_bamboo_terrace`    |
| Sgt. Byrd's Base  | `s3_sgt_byrds_base`    |
| Country Speedway  | `s3_country_speedway`  |
| Spike's Arena     | `s3_spikes_arena`      |
| Spider Town       | `s3_spider_town`       |
| Lost Fleet        | `s3_lost_fleet`        |
| Frozen Altars     | `s3_frozen_altars`     |
| Fireworks Factory | `s3_fireworks_factory` |
| Charmed Ridge     | `s3_charmed_ridge`     |
| Bentley's Outpost | `s3_bentleys_outpost`  |
| Honey Speedway    | `s3_honey_speedway`    |
| Scorch's Pit      | `s3_scorchs_pit`       |
| Starfish Reef     | `s3_starfish_reef`     |
| Crystal Islands   | `s3_crystal_islands`   |
| Desert Ruins      | `s3_desert_ruins`      |
| Haunted Tomb      | `s3_haunted_tomb`      |
| Dino Mines        | `s3_dino_mines`        |
| Agent 9's Lab     | `s3_agent_9s_lab`      |
| Harbor Speedway   | `s3_harbor_speedway`   |
| Sorceress's Lair  | `s3_sorceresss_lair`   |
| Bugbot Factory    | `s3_bugbot_factory`    |
| Super Bonus Round | `s3_super_bonus_round` |

## Boss Splits

Any setting to split for a given boss will do so upon killing them. Below is a table of supported bosses with their corresponding `id` values that can be added to the settings like in the `Split` example.

| Boss                          | Setting                  |
| ----------------------------- | ------------------------ |
| Ripto                         | `s2_ripto_kill`          |
| Sorceress (Lair)              | `s3_sorceress_lair_kill` |
| Sorceress (Super Bonus Round) | `s3_sorceress_sbr_kill`  |

# `CollectableSplit`

Similar to `Split`, this is also defined by an event. In this case, it's picking up a collectable, dependent on the game. The options are:

- `Never`
- `OnCategoryRequirement`
- `EveryCollection`

The category requirement is specifically for [category extensions](https://www.speedrun.com/spyrortce). These are as follows:

- **Spyro the Dragon** - 80 Dragons
- **Spyro 2: Ripto's Rage** - 40 Orbs (not supported by auto-splitter)
- **Spyro: Year of the Dragon** - 149 Eggs

In practice, the XML looks something like this:

```xml
<!-- This setting is Never by default. -->
<!-- You don't need to add a setting if you want to keep the default. -->
<Setting id="s3_egg_collected" type="string" value="EveryCollection"/>
```

Below is a table of the games with their corresponding `id` values that can be added to the settings like in the above example.

| Game    | Setting               |
| ------- | --------------------- |
| Spyro 1 | `s1_dragon_collected` |
| Spyro 3 | `s3_egg_collected`    |
