//! For caching data in a run to know if some event has occurred and what kind of instance
//! that occurrence is.

mod boss;
mod collectables;
mod game;
mod map;

use crate::memory::{Boss, Game, Memory};
use boss::BossCache;
use collectables::CollectableCache;
pub use collectables::Collection;
use game::GameCache;
use map::MapCache;

/// Specifies the instance in which the contained data has occurred.
#[derive(Eq, PartialEq, Debug)]
pub enum Occurrence<T: Clone> {
    First(T),
    Additional(T),
}

impl<T: Clone> Occurrence<T> {
    /// Extracts the data contained in the occurrence. Useful if it doesn't matter *how many* times
    /// this instance has occurred and you just need *what* has occurred.
    pub fn data(&self) -> T {
        match self {
            Self::First(data) | Self::Additional(data) => data.clone(),
        }
    }
}

/// Caches all data for a given run. There is no way to clear this cache,
/// as it's intended to be dropped and re-instantiated every subsequent run.
#[derive(Default)]
pub struct Cache {
    game: GameCache,
    map: MapCache,
    boss: BossCache,
    collectables: CollectableCache,
}

impl Cache {
    /// Creates a new [`Cache`] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an [`Occurrence`] if a game has been started since the last time this
    /// method was called. Should only be called when transitioning from the main menu into
    /// a game to determine what game is starting and/or the specified instance it is.
    pub fn game_started(&mut self, memory: &impl Memory) -> Option<Occurrence<Game>> {
        self.game.started(memory)
    }

    /// Returns an [`Occurrence`] if a map has been exited since the last time this method was
    /// called *and* this is done through a valid map transition. Should be called every frame
    /// to ensure changes are tracked as soon as possible.
    pub fn map_exited(&mut self, memory: &impl Memory) -> Option<Occurrence<String>> {
        self.map.exited(memory)
    }

    /// Returns an [`Occurrence`] if a boss has been killed since the last time this method was
    /// called. Should be called every frame to ensure changes are tracked as soon as possible.
    pub fn boss_killed(&mut self, memory: &impl Memory) -> Option<Occurrence<Boss>> {
        self.boss.killed(memory)
    }

    /// Returns a [`Collection`] if a collectable has been collected since the last time this
    /// method was called. Should be called every frame to ensure changes are tracked as soon
    /// as possible.
    pub fn collectable_collected(
        &mut self,
        game: Game,
        memory: &impl Memory,
    ) -> Option<Collection> {
        self.collectables.collected(game, memory)
    }
}
