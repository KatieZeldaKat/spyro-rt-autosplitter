use super::Occurrence;
use crate::memory::{Memory, Game};
use std::collections::HashSet;

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
