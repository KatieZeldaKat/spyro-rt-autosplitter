//! Going from the title screen to being in control of Spyro.

use super::{Memory, PointerPaths};
#[cfg(debug_assertions)]
use asr::timer;
use asr::{Address, Process, watcher::Watcher};

/// The states the game can be in. Useful for knowing when to start and stop the timer.
///
/// Traversal of the `GameState` is always in-order:
///
/// - `TitleScreen` → `GameLoading`
/// - `GameLoading` → `InControl`
/// - `InControl` → `TitleScreen`
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum GameState {
    #[default]
    TitleScreen,
    GameLoading,
    InControl,
}

/// The three games included in the Reignited Trilogy.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Game {
    /// Spyro the Dragon
    Spyro1,
    /// Spyro 2: Ripto's Rage
    Spyro2,
    /// Spyro: Year of the Dragon
    Spyro3,
}

/// Extracts and caches information about the [`GameState`].
#[derive(Default)]
pub struct GameStateReader {
    game: Option<Game>,
    game_state: Watcher<GameState>,

    on_title: Watcher<bool>,
}

impl GameStateReader {
    /// Updates what [`GameState`] the game is in.
    /// This should only be called by [`Memory`].
    pub fn update(&mut self, process: &Process, address: Address, paths: &PointerPaths) {
        let game_state = self.game_state.pair.get_or_insert_default().current;
        match game_state {
            GameState::TitleScreen => {
                // It isn't guaranteed that we will know the game from the first frame `on_title`
                // changes. Thus, we need to preserve the state of `on_title` changing to `false`
                // until we can get a read on what game is currently being loaded.
                let on_title = if let Some(on_title) = self.on_title.pair
                    && on_title.changed_to(&false)
                {
                    &on_title.clone()
                } else {
                    self.on_title
                        .update_infallible(Self::read_on_title(process, address, paths))
                };

                // We only transition to `GameLoading` if we went from the title screen to the game.
                if on_title.changed_to(&false)
                    && let Some(game) = Self::read_game(process, address, paths)
                {
                    self.game = Some(game);
                    self.on_title.update_infallible(false);
                    self.game_state.update_infallible(GameState::GameLoading);
                } else {
                    self.game_state.update_infallible(GameState::TitleScreen);
                }
            }
            GameState::GameLoading => {
                if Self::read_in_control(process, address, paths) {
                    self.game_state.update_infallible(GameState::InControl);
                } else {
                    self.game_state.update_infallible(GameState::GameLoading);
                }
            }
            GameState::InControl => {
                if Self::read_on_title(process, address, paths) {
                    self.game = None;
                    self.game_state.update_infallible(GameState::TitleScreen);
                } else {
                    self.game_state.update_infallible(GameState::InControl);
                }
            }
        }
    }

    /// Returns the current [`Game`] if in one, [`None`] if still on the title screen.
    #[must_use]
    pub const fn game(&self) -> Option<Game> {
        self.game
    }

    /// Returns the current [`GameState`].
    #[must_use]
    pub fn game_state(&self) -> GameState {
        self.game_state.pair.unwrap_or_default().current
    }

    /// Returns the [`GameState`] if it has just changed, [`None`] otherwise.
    #[must_use]
    pub fn game_state_changed(&self) -> Option<GameState> {
        let state = self.game_state.pair?;
        state.changed().then_some(state.current)
    }

    fn read_on_title(process: &Process, address: Address, paths: &PointerPaths) -> bool {
        let on_title =
            Memory::read::<u8>(process, address, &paths.on_title).unwrap_or_default() == 0;

        #[cfg(debug_assertions)]
        timer::set_variable("on_title", &on_title.to_string());

        on_title
    }

    fn read_game(process: &Process, address: Address, paths: &PointerPaths) -> Option<Game> {
        let game = Memory::read::<u8>(process, address, &paths.game)?;

        #[cfg(debug_assertions)]
        timer::set_variable("game", &game.to_string());

        match game {
            1 => Some(Game::Spyro1),
            2 => Some(Game::Spyro2),
            3 => Some(Game::Spyro3),
            _ => None,
        }
    }

    fn read_in_control(process: &Process, address: Address, paths: &PointerPaths) -> bool {
        let in_control =
            Memory::read::<u8>(process, address, &paths.in_control).unwrap_or_default() > 0;

        #[cfg(debug_assertions)]
        timer::set_variable("in_control", &in_control.to_string());

        in_control
    }
}
