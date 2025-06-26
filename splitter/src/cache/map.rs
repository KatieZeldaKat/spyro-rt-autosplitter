use super::Occurrence;
use crate::memory::Memory;
use asr::watcher::{Pair, Watcher};
use std::collections::HashSet;

/// Caches the current map, tracking when map transitions occur.
pub struct MapCache {
    map: Watcher<String>,
    maps_exited: HashSet<String>,
}

impl MapCache {
    /// Creates a new [`MapCache`] instance.
    /// Intended to be owned by [`Cache`](super::Cache).
    pub fn new() -> Self {
        Self {
            map: Watcher::new(),
            maps_exited: HashSet::new(),
        }
    }

    /// Returns an [`Occurrence`] if a map has been exited *and* this is a valid map transition
    /// since the last time this method was called. Should be called every frame to ensure changes
    /// are tracked as soon as possible.
    pub fn exited(&mut self, memory: &impl Memory) -> Option<Occurrence<String>> {
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

impl Default for MapCache {
    fn default() -> Self {
        Self::new()
    }
}
