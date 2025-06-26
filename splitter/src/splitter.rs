use crate::{
    cache::{Cache, Collection, Occurrence},
    memory::{Game, Memory},
    settings::{CollectableSplit, Settings, Split},
};
use asr::{
    future,
    settings::Gui,
    timer::{self, TimerState},
    watcher::Watcher,
};

/// Manages all runs for the auto-splitter.
pub struct Splitter<'a, T: Memory> {
    memory: &'a T,
    settings: &'a mut Settings,
}

impl<'a, T: Memory> Splitter<'a, T> {
    /// Initializes a new [`Splitter`] instance. Since the splitter needs to read the memory
    /// of a process, the instance must live as long as the process provided.
    ///
    /// # Example:
    ///
    /// ```ignore
    /// let mut settings = Settings::register();
    /// let process = Process::attach("game.exe").unwrap();
    /// if let Ok(address) = process.get_module_address("game.exe") {
    ///     let memory = MemoryReader::new_steam(&process, address);
    ///     let splitter = Splitter::new(&memory, &mut settings);
    ///
    ///     // code to run the splitter
    /// }
    /// ```
    pub fn new(memory: &'a T, settings: &'a mut Settings) -> Self {
        Self { memory, settings }
    }

    /// Initializes a run of the auto-splitter. This function will only return once the timer is no
    /// longer running. This will always return an [`Err`] containing a [`TimerState`], which can
    /// be inspected to see what state the timer was in when the run concluded.
    /// ([`TimerState::NotRunning`], [`TimerState::Ended`], etc.).
    pub async fn run(&mut self) -> Result<(), TimerState> {
        let mut cache = Cache::new();

        loop {
            // Wait until we select a game in the title screen
            self.select_game().await?;

            // Wait to gain control of Spyro if this is the first time we are starting this game
            let game = self.start_game(&mut cache).await?;
            if let Occurrence::First(_) = game {
                self.gain_control().await?;
            }

            // Update settings; assumes no settings are modified mid-game
            self.settings.update();

            // Run the current game
            self.run_game(game.data(), &mut cache).await?;
            timer::resume_game_time();
        }
    }

    async fn select_game(&self) -> Result<(), TimerState> {
        let is_mid_run = timer_running();

        // We use a watcher to check that in_game *changes* to true rather than *is* true so
        // that if someone resets the timer while in a game, the timer won't immediately restart
        let mut in_game = Watcher::<bool>::new();
        while !in_game
            .update_infallible(self.memory.read_in_game())
            .changed_to(&true)
        {
            if is_mid_run {
                next_tick().await?;
            } else {
                let _ = next_tick().await;
            }
        }

        timer::start();

        Ok(())
    }

    async fn start_game(&self, cache: &mut Cache) -> Result<Occurrence<Game>, TimerState> {
        loop {
            if let Some(occurrence) = cache.game().started(self.memory) {
                return Ok(occurrence);
            }

            // It's unknown whether we should pause game time; preemptively do so just in case
            timer::pause_game_time();
            next_tick().await?;
        }
    }

    async fn gain_control(&self) -> Result<(), TimerState> {
        timer::pause_game_time();

        while !self.memory.read_in_control() {
            next_tick().await?;
        }

        timer::resume_game_time();

        Ok(())
    }

    async fn run_game(&self, game: Game, cache: &mut Cache) -> Result<(), TimerState> {
        loop {
            // If no longer in game, exit
            let in_game = self.memory.read_in_game();
            if !in_game {
                return match self.settings.reset_on_title {
                    true => {
                        timer::reset();
                        Err(timer::state())
                    }
                    false => Ok(()),
                };
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
            if let Some(occurrence) = cache.map().exited(self.memory) {
                split_on_occurrence(occurrence, |map| self.settings.split_on_map_exit(&map));
            }

            // Automatically split on boss kill
            if let Some(occurrence) = cache.boss().killed(self.memory) {
                split_on_occurrence(occurrence, |boss| self.settings.split_on_boss_kill(boss));
            }

            // Automaticaly split when collectables are picked up (varies by game)
            if let Some(collection) = cache.collectables().collected(game, self.memory) {
                match self.settings.split_on_collectable_collected(game) {
                    CollectableSplit::Never => (),
                    CollectableSplit::EveryCollection => timer::split(),
                    CollectableSplit::OnCategoryRequirement => match collection {
                        Collection::Intermediate => (),
                        Collection::CategoryRequirement => timer::split(),
                    },
                }
            }

            next_tick().await?;
        }
    }
}

/// Any time we are waiting for the next tick, we should verify after it's over that the timer
/// is still running. If not, return an [`Err<TimerState>`] with what state the timer is in.
async fn next_tick() -> Result<(), TimerState> {
    future::next_tick().await;
    match timer_running() {
        true => Ok(()),
        false => Err(timer::state()),
    }
}

fn split_on_occurrence<T: Clone, P: FnOnce(T) -> Split>(occurrence: Occurrence<T>, get_setting: P) {
    match occurrence {
        Occurrence::First(data) => match get_setting(data) {
            Split::FirstTime | Split::EveryTime => timer::split(),
            Split::Never => (),
        },
        Occurrence::Additional(data) => match get_setting(data) {
            Split::EveryTime => timer::split(),
            Split::FirstTime | Split::Never => (),
        },
    }
}

fn timer_running() -> bool {
    timer::state() == TimerState::Running || timer::state() == TimerState::Paused
}
