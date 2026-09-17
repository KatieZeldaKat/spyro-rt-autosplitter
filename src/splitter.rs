use crate::{
    settings::{LevelExit, Settings},
    Level, Memory,
};
use asr::timer::{self, TimerState};
use std::collections::HashSet;

#[derive(Default)]
pub struct Splitter {
    levels_exited: HashSet<Level>,
}

impl Splitter {
    pub fn update(&mut self, memory: &Memory, settings: &Settings) {
        if !Self::timer_running() {
            self.reset();
            return;
        }

        self.split_on_level_transition(memory, settings);
    }

    pub fn reset(&mut self) {
        self.levels_exited.clear();
    }

    fn split_on_level_transition(&mut self, memory: &Memory, settings: &Settings) {
        if let Some(transition) = memory.level_reader().level_changed() {
            let first_time_exited = self.levels_exited.insert(transition.from);
            let should_split = match settings.get_level_exit_setting(transition.from) {
                LevelExit::Never => false,
                LevelExit::FirstExit => first_time_exited,
                LevelExit::Always => true,
            };

            if should_split {
                timer::split();
            }
        }
    }

    fn timer_running() -> bool {
        timer::state() == TimerState::Running || timer::state() == TimerState::Paused
    }
}
