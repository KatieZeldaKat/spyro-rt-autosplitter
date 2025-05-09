use asr::{
    Address, Process,
    future::next_tick,
    settings::Gui,
    timer::{self, TimerState},
    watcher::Watcher,
};

use crate::{
    cache::{Cache, Occurrence},
    memory::Memory,
    settings::{Settings, Split},
};

macro_rules! return_if_timer_reset_after {
    ($expression:expr) => {
        $expression;
        if (!timer_running()) {
            return;
        }
    };
}

pub struct Splitter<'a> {
    memory: Memory<'a>,
    settings: Settings,
}

impl<'a> Splitter<'a> {
    pub fn new(process: &'a Process, address: Address) -> Self {
        Self {
            memory: Memory::new(process, address),
            settings: Settings::register(),
        }
    }

    pub async fn run(&mut self) {
        let mut cache = Cache::new();

        loop {
            // Wait until we select a game in the title screen
            return_if_timer_reset_after!(self.select_game().await);

            // Wait to gain control of Spyro if this is the first time we are starting this game
            if let Occurrence::First(_) = loop {
                if let Some(occurrence) = cache.game().started(&self.memory) {
                    break occurrence;
                }

                next_tick().await;
            } {
                return_if_timer_reset_after!(self.gain_control().await);
            }

            // Update settings; assumes no settings are modified mid-game
            self.settings.update();

            // Run the current game
            return_if_timer_reset_after!(self.run_game(&mut cache).await);
            timer::resume_game_time();
        }
    }

    async fn select_game(&self) {
        let is_mid_run = timer_running();

        // We use a watcher to check that in_game *changes* to true rather than *is* true so
        // that if someone resets the timer while in a game, the timer won't immediately restart
        let mut in_game = Watcher::<bool>::new();
        while !in_game
            .update_infallible(self.memory.read_in_game())
            .changed_to(&true)
        {
            if is_mid_run {
                return_if_timer_reset_after!(next_tick().await);
            } else {
                next_tick().await;
            }
        }

        timer::start();
        timer::pause_game_time();
    }

    async fn gain_control(&self) {
        while !self.memory.read_in_control() {
            return_if_timer_reset_after!(next_tick().await);
        }
    }

    async fn run_game(&self, cache: &mut Cache) {
        while timer_running() {
            // If no longer in game, exit
            let in_game = self.memory.read_in_game();
            if !in_game {
                if self.settings.reset_on_title {
                    timer::reset();
                }

                return;
            }

            // Read memory to determine game timer state
            let is_loading = self.memory.read_is_loading();
            let in_menu = self.memory.read_in_menu();

            // in_menu check prevents abuse of buffering loading and pausing in the exact same frame
            if !is_loading || in_menu || !in_game {
                timer::resume_game_time();
            } else if is_loading {
                timer::pause_game_time();
            }

            // Automatically split on map exit
            if let Some(occurrence) = cache.map().exited(&self.memory) {
                match occurrence {
                    Occurrence::First(map) => match self.settings.split_on_map_exit(&map) {
                        Split::FirstTime | Split::EveryTime => timer::split(),
                        Split::Never => (),
                    },
                    Occurrence::Additional(map) => match self.settings.split_on_map_exit(&map) {
                        Split::EveryTime => timer::split(),
                        Split::FirstTime | Split::Never => (),
                    },
                }
            }

            // Automatically split on boss kill
            if let Some(occurrence) = cache.boss().killed(&self.memory) {
                match occurrence {
                    Occurrence::First(boss) => match self.settings.split_on_boss_kill(boss) {
                        Split::FirstTime | Split::EveryTime => timer::split(),
                        Split::Never => (),
                    },
                    Occurrence::Additional(boss) => match self.settings.split_on_boss_kill(boss) {
                        Split::EveryTime => timer::split(),
                        Split::FirstTime | Split::Never => (),
                    },
                }
            }

            next_tick().await;
        }
    }
}

fn timer_running() -> bool {
    timer::state() == TimerState::Running || timer::state() == TimerState::Paused
}
