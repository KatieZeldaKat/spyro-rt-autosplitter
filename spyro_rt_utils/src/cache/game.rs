use super::Occurrence;
use crate::memory::{Game, Memory};
use std::collections::HashSet;

/// Caches the current game, tracking when it changes.
#[derive(Default)]
pub struct GameCache {
    games_started: HashSet<Game>,
}

impl GameCache {
    /// See [`game_started()`](super::Cache::game_started).
    pub fn started(&mut self, memory: &impl Memory) -> Option<Occurrence<Game>> {
        let game = memory.read_game()?;
        match self.games_started.insert(game) {
            true => Some(Occurrence::First(game)),
            false => Some(Occurrence::Additional(game)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockMemory;

    #[test]
    fn no_occurrence_on_menu() {
        let mut memory = MockMemory::default();
        memory.game = None;

        let occurrence = GameCache::default().started(&memory);
        assert_eq!(occurrence, None);
    }

    #[test]
    fn occurrence_when_in_game() {
        let mut memory = MockMemory::default();
        memory.game = Some(Game::Spyro1);

        let occurrence = GameCache::default().started(&memory);
        assert_ne!(occurrence, None);
    }

    #[test]
    fn first_and_additional_occurrences() {
        let mut game_cache = GameCache::default();
        let mut memory = MockMemory::default();
        memory.game = Some(Game::Spyro1);

        // First Occurrence
        let occurrence = game_cache.started(&memory);
        assert_eq!(occurrence, Some(Occurrence::First(Game::Spyro1)));

        // Second Occurrence
        let occurrence = game_cache.started(&memory);
        assert_eq!(occurrence, Some(Occurrence::Additional(Game::Spyro1)));
    }

    #[test]
    fn different_games_first_occurrences() {
        let mut game_cache = GameCache::default();
        let mut memory = MockMemory::default();

        memory.game = Some(Game::Spyro1);
        let occurrence = game_cache.started(&memory);
        assert_eq!(occurrence, Some(Occurrence::First(Game::Spyro1)));

        memory.game = Some(Game::Spyro2);
        let occurrence = game_cache.started(&memory);
        assert_eq!(occurrence, Some(Occurrence::First(Game::Spyro2)));
    }
}
