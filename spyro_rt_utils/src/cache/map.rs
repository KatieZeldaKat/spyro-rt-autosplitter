use super::Occurrence;
use crate::memory::Memory;
use asr::watcher::{Pair, Watcher};
use std::collections::HashSet;

/// Caches the current map, tracking when map transitions occur.
#[derive(Default)]
pub struct MapCache {
    map: Watcher<String>,
    maps_exited: HashSet<String>,
}

impl MapCache {
    /// See [`map_exited()`](super::Cache::map_exited).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockMemory;

    #[test]
    fn no_occurrence_on_first_run() {
        let memory = MockMemory::default();
        let occurrence = MapCache::default().exited(&memory);
        assert_eq!(occurrence, None);
    }

    #[test]
    fn no_occurrence_on_static_map() {
        let mut cache = MapCache::default();
        let mut memory = MockMemory::default();

        memory.map = Some(String::default());
        let _ = cache.exited(&memory);
        let occurrence = cache.exited(&memory);

        assert_eq!(occurrence, None);
    }

    #[test]
    fn occurrence_on_map_change() {
        let mut cache = MapCache::default();
        let mut memory = MockMemory::default();

        memory.map = Some(String::from("one"));
        let _ = cache.exited(&memory);

        memory.map = Some(String::from("two"));
        let occurrence = cache.exited(&memory);

        assert_ne!(occurrence, None);
    }

    #[test]
    fn first_and_additional_occurrences() {
        let one = Some(String::from("one"));
        let two = Some(String::from("two"));

        let mut cache = MapCache::default();
        let mut memory = MockMemory::default();

        // First Exit
        memory.map = one.clone();
        let _ = cache.exited(&memory);
        memory.map = two.clone();
        let occurrence = cache.exited(&memory);

        assert_eq!(occurrence, Some(Occurrence::First(one.clone().unwrap())));

        // Second Exit
        memory.map = one.clone();
        let _ = cache.exited(&memory);
        memory.map = two.clone();
        let occurrence = cache.exited(&memory);

        assert_eq!(
            occurrence,
            Some(Occurrence::Additional(one.clone().unwrap()))
        );
    }

    #[test]
    fn different_maps_first_occurrences() {
        let one = Some(String::from("one"));
        let two = Some(String::from("two"));

        let mut cache = MapCache::default();
        let mut memory = MockMemory::default();

        // First Exit
        memory.map = one.clone();
        let _ = cache.exited(&memory);
        memory.map = two.clone();
        let occurrence = cache.exited(&memory);

        assert_eq!(occurrence, Some(Occurrence::First(one.clone().unwrap())));

        // Second Exit
        memory.map = two.clone();
        let _ = cache.exited(&memory);
        memory.map = one.clone();
        let occurrence = cache.exited(&memory);

        assert_eq!(occurrence, Some(Occurrence::First(two.clone().unwrap())));
    }
}
