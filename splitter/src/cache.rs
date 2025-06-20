mod boss;
mod collectables;
mod game;
mod map;

pub use boss::BossCache;
pub use collectables::{CollectableCache, Collection};
pub use game::GameCache;
pub use map::MapCache;

pub enum Occurrence<T: Clone> {
    First(T),
    Additional(T),
}

impl<T: Clone> Occurrence<T> {
    pub fn data(&self) -> T {
        match self {
            Self::First(data) | Self::Additional(data) => data.clone(),
        }
    }
}

pub struct Cache {
    game: GameCache,
    map: MapCache,
    boss: BossCache,
    collectables: CollectableCache,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            game: GameCache::new(),
            map: MapCache::new(),
            boss: BossCache::new(),
            collectables: CollectableCache::new(),
        }
    }

    pub fn game(&mut self) -> &mut GameCache {
        &mut self.game
    }

    pub fn map(&mut self) -> &mut MapCache {
        &mut self.map
    }

    pub fn boss(&mut self) -> &mut BossCache {
        &mut self.boss
    }

    pub fn collectables(&mut self) -> &mut CollectableCache {
        &mut self.collectables
    }
}
