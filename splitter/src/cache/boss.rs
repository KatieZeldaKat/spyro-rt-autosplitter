use super::Occurrence;
use crate::{Boss, Memory};
use asr::watcher::Watcher;
use std::collections::HashSet;

/// Caches defeated bosses and the current boss' health, tracking when values change.
pub struct BossCache {
    boss: Watcher<Option<Boss>>,
    bosses_killed: HashSet<Boss>,
}

impl BossCache {
    /// Creates a new [`BossCache`] instance.
    /// Intended to be owned by [`Cache`](super::Cache).
    pub fn new() -> Self {
        Self {
            boss: Watcher::new(),
            bosses_killed: HashSet::new(),
        }
    }

    /// Returns an [`Occurrence`] if a boss has been killed since the last time this method was
    /// called. Should be called every frame to ensure changes are tracked as soon as possible.
    pub fn killed(&mut self, memory: &Memory) -> Option<Occurrence<Boss>> {
        let boss = self.boss.update_infallible(memory.read_boss());
        let old_boss = boss.old?;
        let current_boss = boss.current?;

        if old_boss.health() == 1 && current_boss.health() == 0 {
            return match self.bosses_killed.insert(current_boss) {
                true => Some(Occurrence::First(current_boss)),
                false => Some(Occurrence::Additional(current_boss)),
            };
        }

        None
    }
}

impl Default for BossCache {
    fn default() -> Self {
        Self::new()
    }
}
