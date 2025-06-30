use super::Occurrence;
use crate::memory::{Boss, Memory};
use asr::watcher::Watcher;
use std::collections::HashSet;

/// Caches defeated bosses and the current boss' health, tracking when values change.
#[derive(Default)]
pub struct BossCache {
    boss: Watcher<Option<Boss>>,
    bosses_killed: HashSet<Boss>,
}

impl BossCache {
    /// See [`boss_killed()`](super::Cache::boss_killed).
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockMemory;

    #[test]
    fn no_occurrence_on_first_run() {
        let memory = MockMemory::default();
        let occurrence = BossCache::default().killed(&memory);
        assert_eq!(occurrence, None);
    }

    #[test]
    fn no_occurrence_on_regular_blow() {
        let mut cache = BossCache::default();
        let mut memory = MockMemory::default();

        memory.boss = Some(Boss::Ripto(5));
        let _ = cache.killed(&memory);

        memory.boss = Some(Boss::Ripto(4));
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, None);
    }

    #[test]
    fn no_occurrence_on_static_health() {
        let mut cache = BossCache::default();
        let mut memory = MockMemory::default();

        memory.boss = Some(Boss::Ripto(4));
        let _ = cache.killed(&memory);
        let occurrence = cache.killed(&memory);

        assert_eq!(occurrence, None);
    }

    #[test]
    fn occurrence_on_final_blow() {
        let mut cache = BossCache::default();
        let mut memory = MockMemory::default();

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

        let mut cache = BossCache::default();
        let mut memory = MockMemory::default();

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
        let mut cache = BossCache::default();
        let mut memory = MockMemory::default();

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
