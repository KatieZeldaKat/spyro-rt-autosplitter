//! When the game is loading and the player isn't in a menu.

use super::{Memory, game_state::GameState};
#[cfg(debug_assertions)]
use asr::timer;
use asr::{Address, Process, watcher::Watcher};

/// Whether or not the game is loading.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum LoadState {
    Loading,
    #[default]
    Done,
}

/// Extracts and caches information about the [`LoadState`].
#[derive(Default)]
pub struct LoadStateReader {
    load_state: Watcher<LoadState>,
}

impl LoadStateReader {
    /// Updates what [`LoadState`] the game is in.
    /// This should only be called by [`Memory`].
    pub fn update(&mut self, process: &Process, address: Address, game_state: GameState) {
        let event = match game_state {
            GameState::TitleScreen => LoadState::Done,
            GameState::GameLoading | GameState::InControl => {
                if Self::read_in_menu(process, address) {
                    LoadState::Done
                } else if Self::read_loading(process, address) {
                    LoadState::Loading
                } else {
                    LoadState::Done
                }
            }
        };

        self.load_state.update_infallible(event);
    }

    /// Returns the [`LoadState`] if it has just changed, [`None`] otherwise.
    #[must_use]
    pub fn load_state_changed(&self) -> Option<LoadState> {
        let state = self.load_state.pair?;
        state.changed().then_some(state.current)
    }

    fn read_loading(process: &Process, address: Address) -> bool {
        let path = &[0x0341_5F30, 0xF8, 0x4A8, 0xE19];
        let loading = Memory::read::<u8>(process, address, path).unwrap_or_default() == 0;

        #[cfg(debug_assertions)]
        timer::set_variable("loading", &loading.to_string());

        loading
    }

    fn read_in_menu(process: &Process, address: Address) -> bool {
        let path = &[0x0341_60D0, 0x20, 0x218, 0x60];
        let in_menu = Memory::read::<u8>(process, address, path).unwrap_or_default() > 0;

        #[cfg(debug_assertions)]
        timer::set_variable("in_menu", &in_menu.to_string());

        in_menu
    }
}
