mod boss;
mod collectables;
mod game;
mod map;

pub use boss::BossCache;
pub use collectables::{CollectableCache, Collection};
pub use game::GameCache;
pub use map::MapCache;

/// Specifies the instance in which the contained data has occurred.
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
pub struct Cache {
    game: GameCache,
    map: MapCache,
    boss: BossCache,
    collectables: CollectableCache,
}

impl Cache {
    /// Creates a new [`Cache`] instance. Intended to be owned by [`Splitter`](crate::Splitter).
    pub fn new() -> Self {
        Self {
            game: GameCache::new(),
            map: MapCache::new(),
            boss: BossCache::new(),
            collectables: CollectableCache::new(),
        }
    }

    /// The [`GameCache`] for the current run.
    pub fn game(&mut self) -> &mut GameCache {
        &mut self.game
    }

    /// The [`MapCache`] for the current run.
    pub fn map(&mut self) -> &mut MapCache {
        &mut self.map
    }

    /// The [`BossCache`] for the current run.
    pub fn boss(&mut self) -> &mut BossCache {
        &mut self.boss
    }

    /// The [`CollectableCache`] for the current run.
    pub fn collectables(&mut self) -> &mut CollectableCache {
        &mut self.collectables
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}
