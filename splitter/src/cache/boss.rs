use super::Occurrence;
use crate::memory::{Memory, Boss};
use asr::watcher::Watcher;
use std::collections::HashSet;

pub struct BossCache {
    boss: Watcher<Option<Boss>>,
    bosses_killed: HashSet<Boss>,
}

impl BossCache {
    pub fn new() -> Self {
        Self {
            boss: Watcher::new(),
            bosses_killed: HashSet::new(),
        }
    }

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
