use super::Occurrence;
use crate::memory::{Game, Memory};
use std::collections::HashSet;

/// Caches the current game, tracking when it changes.
pub struct GameCache {
    games_started: HashSet<Game>,
}

impl GameCache {
    /// Creates a new [`GameCache`] instance.
    /// Intended to be owned by [`Cache`](super::Cache).
    pub fn new() -> Self {
        Self {
            games_started: HashSet::new(),
        }
    }

    /// Returns an [`Occurrence`] if a game has been started since the last time this
    /// method was called. Should only be called when transitioning from the main menu into
    /// a game to determine what game is starting and/or the specified instance it is.
    pub fn started(&mut self, memory: &impl Memory) -> Option<Occurrence<Game>> {
        let game = memory.read_game()?;
        match self.games_started.insert(game) {
            true => Some(Occurrence::First(game)),
            false => Some(Occurrence::Additional(game)),
        }
    }
}

impl Default for GameCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockMemory;

    #[test]
    fn no_occurrence_on_menu() {
        let mut memory = MockMemory::new();
        memory.game = None;

        let occurrence = GameCache::new().started(&memory);
        assert_eq!(occurrence, None);
    }

    #[test]
    fn occurrence_when_in_game() {
        let mut memory = MockMemory::new();
        memory.game = Some(Game::Spyro1);

        let occurrence = GameCache::new().started(&memory);
        assert_ne!(occurrence, None);
    }

    #[test]
    fn first_and_additional_occurrences() {
        let mut game_cache = GameCache::new();
        let mut memory = MockMemory::new();
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
        let mut game_cache = GameCache::new();
        let mut memory = MockMemory::new();

        memory.game = Some(Game::Spyro1);
        let occurrence = game_cache.started(&memory);
        assert_eq!(occurrence, Some(Occurrence::First(Game::Spyro1)));

        memory.game = Some(Game::Spyro2);
        let occurrence = game_cache.started(&memory);
        assert_eq!(occurrence, Some(Occurrence::First(Game::Spyro2)));
    }
}
