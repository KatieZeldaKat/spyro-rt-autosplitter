use std::collections::HashMap;
use crate::memory::{Game, Memory};
use asr::watcher::Watcher;

const DRAGON_CATEGORY_REQUIREMENT: u8 = 80;
const ORB_CATEGORY_REQUIREMENT: u8 = 40;
const EGG_CATEGORY_REQUIREMENT: u8 = 149;

pub enum Collection {
    Intermediate,
    CategoryRequirement,
}

pub struct CollectableCache {
    collectables: HashMap<Game, Watcher<u8>>,
}

impl CollectableCache {
    pub fn new() -> Self {
        Self {
            collectables: HashMap::new(),
        }
    }

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
