use crate::memory::{Game, Memory};
use asr::watcher::Watcher;
use std::collections::HashMap;

const DRAGON_CATEGORY_REQUIREMENT: u8 = 80;
const ORB_CATEGORY_REQUIREMENT: u8 = 40;
const EGG_CATEGORY_REQUIREMENT: u8 = 149;

/// Similar to [`Occurrence`](super::Occurrence), this specifies the instance the
/// collectable for the current game has been collected.
#[derive(Eq, PartialEq, Debug)]
pub enum Collection {
    Intermediate,
    CategoryRequirement,
}

/// Caches the collectables of the current game, tracking when values change.
pub struct CollectableCache {
    collectables: HashMap<Game, Watcher<u8>>,
}

impl CollectableCache {
    /// Creates a new [`CollectableCache`] instance.
    /// Intended to be owned by [`Cache`](super::Cache).
    pub fn new() -> Self {
        Self {
            collectables: HashMap::new(),
        }
    }

    /// Returns a [`Collection`] if a collectable has been collected since the last time this
    /// method was called. Should be called every frame to ensure changes are tracked as soon
    /// as possible.
    pub fn collected(&mut self, game: Game, memory: &impl Memory) -> Option<Collection> {
        let collectables = self
            .collectables
            .entry(game)
            .or_default()
            .update_infallible(memory.read_collectable_count(game));

        let category_requirement = match game {
            Game::Spyro1 => DRAGON_CATEGORY_REQUIREMENT,
            Game::Spyro2 => ORB_CATEGORY_REQUIREMENT,
            Game::Spyro3 => EGG_CATEGORY_REQUIREMENT,
        };

        if collectables.increased() {
            if collectables.current == category_requirement {
                Some(Collection::CategoryRequirement)
            } else {
                Some(Collection::Intermediate)
            }
        } else {
            None
        }
    }
}

impl Default for CollectableCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockMemory;

    #[test]
    fn no_collection_on_first_run() {
        let memory = MockMemory::new();
        let collection = CollectableCache::new().collected(Game::Spyro1, &memory);
        assert_eq!(collection, None);
    }

    #[test]
    fn no_collection_on_static_counter() {
        let mut cache = CollectableCache::new();
        let mut memory = MockMemory::new();

        memory.collectable_count = 42;
        let _ = cache.collected(Game::Spyro1, &memory);
        let collection = cache.collected(Game::Spyro1, &memory);

        assert_eq!(collection, None);
    }

    #[test]
    fn collection_detected() {
        let mut cache = CollectableCache::new();
        let mut memory = MockMemory::new();

        memory.collectable_count = 0;
        let _ = cache.collected(Game::Spyro1, &memory);

        memory.collectable_count = 1;
        let collection = cache.collected(Game::Spyro1, &memory);

        assert_ne!(collection, None);
    }

    #[test]
    fn spyro1_category_collection_detected() {
        let mut cache = CollectableCache::new();
        let mut memory = MockMemory::new();

        memory.collectable_count = DRAGON_CATEGORY_REQUIREMENT - 1;
        let _ = cache.collected(Game::Spyro1, &memory);

        memory.collectable_count = DRAGON_CATEGORY_REQUIREMENT;
        let collection = cache.collected(Game::Spyro1, &memory);

        assert_eq!(collection, Some(Collection::CategoryRequirement));
    }

    #[test]
    fn spyro2_category_collection_detected() {
        let mut cache = CollectableCache::new();
        let mut memory = MockMemory::new();

        memory.collectable_count = ORB_CATEGORY_REQUIREMENT - 1;
        let _ = cache.collected(Game::Spyro2, &memory);

        memory.collectable_count = ORB_CATEGORY_REQUIREMENT;
        let collection = cache.collected(Game::Spyro2, &memory);

        assert_eq!(collection, Some(Collection::CategoryRequirement));
    }

    #[test]
    fn spyro3_category_collection_detected() {
        let mut cache = CollectableCache::new();
        let mut memory = MockMemory::new();

        memory.collectable_count = EGG_CATEGORY_REQUIREMENT - 1;
        let _ = cache.collected(Game::Spyro3, &memory);

        memory.collectable_count = EGG_CATEGORY_REQUIREMENT;
        let collection = cache.collected(Game::Spyro3, &memory);

        assert_eq!(collection, Some(Collection::CategoryRequirement));
    }
}
