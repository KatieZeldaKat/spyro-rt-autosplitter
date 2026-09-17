//! Going from the title screen to being in control of Spyro.

use super::Memory;
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

/// Extracts and caches information about the [`GameState`].
#[derive(Default)]
pub struct GameStateReader {
    game_state: Watcher<GameState>,

    on_title: Watcher<bool>,
}

impl GameStateReader {
    /// Updates what [`GameState`] the game is in.
    /// This should only be called by [`Memory`].
    pub fn update(&mut self, process: &Process, address: Address) {
        let game_state = self.game_state.pair.get_or_insert_default().current;
        match game_state {
            GameState::TitleScreen => {
                let on_title = self
                    .on_title
                    .update_infallible(Self::read_on_title(process, address));
                if on_title.changed_to(&false) {
                    self.game_state.update_infallible(GameState::GameLoading);
                } else {
                    self.game_state.update_infallible(GameState::TitleScreen);
                }
            }
            GameState::GameLoading => {
                if Self::read_in_control(process, address) {
                    self.game_state.update_infallible(GameState::InControl);
                } else {
                    self.game_state.update_infallible(GameState::GameLoading);
                }
            }
            GameState::InControl => {
                if Self::read_on_title(process, address) {
                    self.game_state.update_infallible(GameState::TitleScreen);
                } else {
                    self.game_state.update_infallible(GameState::InControl);
                }
            }
        }
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

    fn read_on_title(process: &Process, address: Address) -> bool {
        let path = &[0x0341_5F30, 0xF0, 0x378, 0x564];
        let on_title = Memory::read::<u8>(process, address, path).unwrap_or_default() == 0;

        #[cfg(debug_assertions)]
        timer::set_variable("on_title", &on_title.to_string());

        on_title
    }

    fn read_in_control(process: &Process, address: Address) -> bool {
        let path = &[0x0341_5F30, 0xF8, 0x478];
        let in_control = Memory::read::<u8>(process, address, path).unwrap_or_default() > 0;

        #[cfg(debug_assertions)]
        timer::set_variable("in_control", &in_control.to_string());

        in_control
    }
}
