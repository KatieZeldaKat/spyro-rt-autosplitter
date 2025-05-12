use std::collections::HashSet;

use asr::watcher::{Pair, Watcher};

use crate::memory::{Boss, Game, Memory};

pub enum Occurrence<T: Clone> {
    First(T),
    Additional(T),
}

pub struct Cache {
    game: GameCache,
    map: MapCache,
    boss: BossCache,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            game: GameCache::new(),
            map: MapCache::new(),
            boss: BossCache::new(),
        }
    }

    pub fn game(&mut self) -> &mut GameCache {
        &mut self.game
    }

    pub fn map(&mut self) -> &mut MapCache {
        &mut self.map
    }

    pub fn boss(&mut self) -> &mut BossCache {
        &mut self.boss
    }
}

pub struct GameCache {
    games_started: HashSet<Game>,
}

impl GameCache {
    pub fn new() -> Self {
        Self {
            games_started: HashSet::new(),
        }
    }

    pub fn started(&mut self, memory: &Memory) -> Option<Occurrence<Game>> {
        let game = memory.read_game()?;
        match self.games_started.insert(game) {
            true => Some(Occurrence::First(game)),
            false => Some(Occurrence::Additional(game)),
        }
    }
}

pub struct MapCache {
    map: Watcher<String>,
    maps_exited: HashSet<String>,
}

impl MapCache {
    pub fn new() -> Self {
        Self {
            map: Watcher::new(),
            maps_exited: HashSet::new(),
        }
    }

    pub fn exited(&mut self, memory: &Memory) -> Option<Occurrence<String>> {
        let map = memory.read_map()?;
        let map = self.map.update_infallible(map);

        if map.changed() && MapCache::is_valid_map_transition(map) {
            return match self.maps_exited.insert(map.old.clone()) {
                true => Some(Occurrence::First(map.old.clone())),
                false => Some(Occurrence::Additional(map.old.clone())),
            };
        }

        None
    }

    fn is_valid_map_transition(map: &Pair<String>) -> bool {
        map.current
            == match &map.old as &str {
                "/LS101_ArtisansHome/Maps/" => "/LS107_PeacekeeperHome/Maps/",
                "/LS107_PeacekeeperHome/Maps/" => "/LS113_MagicHome/Maps/",
                "/LS113_MagicHome/Maps/" => "/LS119_BeastHome/Maps/",
                "/LS119_BeastHome/Maps/" => "/LS125_DreamWeaverHome/Maps/",
                "/LS125_DreamWeaverHome/Maps/" => "/LS131_GnastyHome/Maps/",
                "/LS208_CrushsDungeon/Maps/" => "/LS210_AutumnPlains_Home/Maps/",
                "/LS219_GulpsOverlook/Maps/" => "/LS222_WinterTundra_Home/Maps/",
                "/LS227_RiptosArena/Maps/" => "/LS229_DragonShores/Maps/",
                _ => &map.current,
            }
    }
}

pub struct BossCache {
    boss: Watcher<Option<Boss>>,
    bosses_killed: HashSet<Boss>,
}

impl BossCache {
    pub fn new() -> Self {
        Self {
            boss: Watcher::new(),
            bosses_killed: HashSet::new(),
        }
    }

    pub fn killed(&mut self, memory: &Memory) -> Option<Occurrence<Boss>> {
        let boss = self.boss.update_infallible(memory.read_boss());
        let old_boss = boss.old?;
        let current_boss = boss.current?;

        if old_boss.health() == 1 && current_boss.health() == 0 {
            return match self.bosses_killed.insert(current_boss) {
                true => Some(Occurrence::First(current_boss)),
                false => Some(Occurrence::Additional(current_boss)),
            };
        }

        None
    }
}
