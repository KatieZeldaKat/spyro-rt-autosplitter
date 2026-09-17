use crate::{
    memory::{Memory, boss::Boss, game_state::GameState, level::Level, loading::LoadState},
    settings::{BossDefeat, LevelExit, Settings},
};
use asr::timer::{self, TimerState};
use std::collections::HashSet;

trait TimerActive {
    fn active(&self) -> bool;
}

impl TimerActive for TimerState {
    fn active(&self) -> bool {
        *self == Self::Running || *self == Self::Paused
    }
}

/// Performs actions on the timer, such as starting, resetting, and splitting.
#[derive(Default)]
pub struct Splitter {
    game_state: GameState,
    levels_exited: HashSet<Level>,
    bosses_defeated: HashSet<Boss>,
}

impl Splitter {
    /// Updates the state of the auto-splitter, mutating the timer if necessary.
    /// This method should be called every tick.
    pub fn update(&mut self, memory: &Memory, settings: &Settings) {
        self.update_game_state(memory, settings);
        if !timer::state().active() {
            self.reset();
            return;
        }

        Self::update_load_state(memory);
        self.split_on_level_transition(memory, settings);
        self.split_on_boss_defeated(memory, settings);
        Self::split_on_collectible_earned(memory, settings);
    }

    fn reset(&mut self) {
        self.levels_exited.clear();
        self.bosses_defeated.clear();
    }

    fn update_game_state(&mut self, memory: &Memory, settings: &Settings) {
        if let Some(game_state) = memory.game_state_reader().game_state_changed() {
            self.game_state = game_state;
            match game_state {
                GameState::TitleScreen => {
                    if settings.reset_on_title() {
                        timer::reset();
                    }
                }
                GameState::GameLoading => {
                    timer::start();
                    timer::pause_game_time();
                }
                GameState::InControl => timer::resume_game_time(),
            }
        }
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

                #[cfg(debug_assertions)]
                asr::print_message("Split on level transition.");
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

                #[cfg(debug_assertions)]
                asr::print_message("Split on boss defeated.");
            }
        }
    }

    fn update_load_state(memory: &Memory) {
        if let Some(load_state) = memory.load_state_reader().load_state_changed() {
            match load_state {
                LoadState::Loading => timer::pause_game_time(),
                LoadState::Done => timer::resume_game_time(),
            }
        }
    }

    fn split_on_collectible_earned(memory: &Memory, settings: &Settings) {
        if let Some(collectible) = memory.collectible_reader().collectible_earned()
            && settings.get_split_on_collectible(collectible)
        {
            timer::split();

            #[cfg(debug_assertions)]
            asr::print_message("Split on collectible earned.");
        }
    }
}
