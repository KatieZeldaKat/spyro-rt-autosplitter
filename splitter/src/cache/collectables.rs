use crate::{Game, Memory};
use asr::watcher::Watcher;
use std::collections::HashMap;

const DRAGON_CATEGORY_REQUIREMENT: u8 = 80;
const ORB_CATEGORY_REQUIREMENT: u8 = 40;
const EGG_CATEGORY_REQUIREMENT: u8 = 149;

/// Similar to [`Occurrence`](super::Occurrence), this specifies the instance the
/// collectable for the current game has been collected.
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
    pub fn collected(&mut self, game: Game, memory: &Memory) -> Option<Collection> {
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
