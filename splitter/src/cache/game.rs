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
