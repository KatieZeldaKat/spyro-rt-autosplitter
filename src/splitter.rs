use crate::{
    memory::{Memory, boss::Boss, level::Level},
    settings::{BossDefeat, LevelExit, Settings},
};
use asr::timer::{self, TimerState};
use std::collections::HashSet;

/// Performs actions on the timer, such as starting, resetting, and splitting.
#[derive(Default)]
pub struct Splitter {
    levels_exited: HashSet<Level>,
    bosses_defeated: HashSet<Boss>,
}

impl Splitter {
    /// Updates the state of the auto-splitter, mutating the timer if necessary.
    /// This method should be called every tick.
    pub fn update(&mut self, memory: &Memory, settings: &Settings) {
        if !Self::timer_running() {
            self.reset();
            return;
        }

        self.split_on_level_transition(memory, settings);
        self.split_on_boss_defeated(memory, settings);
        Self::split_on_collectible_earned(memory, settings);
    }

    fn reset(&mut self) {
        self.levels_exited.clear();
        self.bosses_defeated.clear();
    }

    fn split_on_level_transition(&mut self, memory: &Memory, settings: &Settings) {
        if let Some(transition) = memory.level_reader().level_changed() {
            let first_time_exited = self.levels_exited.insert(transition.from());
            let should_split = match settings.get_level_exit_setting(transition.from()) {
                LevelExit::Never => false,
                LevelExit::FirstExit => first_time_exited,
                LevelExit::Always => true,
            };

            if should_split {
                timer::split();
            }
        }
    }

    fn split_on_boss_defeated(&mut self, memory: &Memory, settings: &Settings) {
        if let Some(boss) = memory.boss_reader().boss_defeated() {
            let first_time_defeated = self.bosses_defeated.insert(boss);
            let should_split = match settings.get_boss_defeat_setting(boss) {
                BossDefeat::Never => false,
                BossDefeat::FirstDefeat => first_time_defeated,
                BossDefeat::Always => true,
            };

            if should_split {
                timer::split();
            }
        }
    }

    fn split_on_collectible_earned(memory: &Memory, settings: &Settings) {
        if let Some(collectible) = memory.collectible_reader().collectible_earned()
            && settings.get_split_on_collectible(collectible)
        {
            timer::split();
        }
    }

    fn timer_running() -> bool {
        timer::state() == TimerState::Running || timer::state() == TimerState::Paused
    }
}
