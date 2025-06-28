use super::Occurrence;
use crate::memory::{Boss, Memory};
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
    pub fn killed(&mut self, memory: &impl Memory) -> Option<Occurrence<Boss>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockMemory;

    #[test]
    fn no_occurrence_on_first_run() {
        let memory = MockMemory::new();
        let occurrence = BossCache::new().killed(&memory);
        assert_eq!(occurrence, None);
    }

    #[test]
    fn no_occurrence_on_regular_blow() {
        let mut cache = BossCache::new();
        let mut memory = MockMemory::new();

        memory.boss = Some(Boss::Ripto(5));
        let _ = cache.killed(&memory);

        memory.boss = Some(Boss::Ripto(4));
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, None);
    }

    #[test]
    fn no_occurrence_on_static_health() {
        let mut cache = BossCache::new();
        let mut memory = MockMemory::new();

        memory.boss = Some(Boss::Ripto(4));
        let _ = cache.killed(&memory);
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, None);
    }

    #[test]
    fn occurrence_on_final_blow() {
        let mut cache = BossCache::new();
        let mut memory = MockMemory::new();

        memory.boss = Some(Boss::Ripto(1));
        let _ = cache.killed(&memory);

        memory.boss = Some(Boss::Ripto(0));
        let occurrence = cache.killed(&memory);

        assert_ne!(occurrence, None);
    }

    #[test]
    fn first_and_additional_occurrences() {
        let one_health = Boss::Ripto(1);
        let zero_health = Boss::Ripto(0);

        let mut cache = BossCache::new();
        let mut memory = MockMemory::new();

        // First Kill
        memory.boss = Some(one_health);
        let _ = cache.killed(&memory);
        memory.boss = Some(zero_health);
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, Some(Occurrence::First(zero_health)));

        // Second Kill
        memory.boss = Some(one_health);
        let _ = cache.killed(&memory);
        memory.boss = Some(zero_health);
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, Some(Occurrence::Additional(zero_health)));
    }

    #[test]
    fn different_bosses_first_occurrences() {
        let mut cache = BossCache::new();
        let mut memory = MockMemory::new();

        memory.boss = Some(Boss::Ripto(1));
        let _ = cache.killed(&memory);
        memory.boss = Some(Boss::Ripto(0));
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, Some(Occurrence::First(Boss::Ripto(0))));

        memory.boss = Some(Boss::SorceressLair(1));
        let _ = cache.killed(&memory);
        memory.boss = Some(Boss::SorceressLair(0));
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, Some(Occurrence::First(Boss::SorceressLair(0))));
    }
}
