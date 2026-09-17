//! Extracts information from the executable in memory.

pub mod boss;
pub mod collectible;
pub mod game_state;
pub mod level;

use boss::BossReader;
use collectible::CollectibleReader;
use game_state::{GameState, GameStateReader};
use level::LevelReader;

use asr::{Address, Process};
use bytemuck::Pod;

/// Contains memory readers that track the state of the game.
#[derive(Default)]
pub struct Memory {
    boss: BossReader,
    collectible: CollectibleReader,
    game_state: GameStateReader,
    level: LevelReader,
}

impl Memory {
    /// Extracts information from the executable in memory, ensuring the game's state is updated.
    /// This method should be called every tick.
    pub fn update(&mut self, process: &Process, address: Address) {
        self.game_state.update(process, address);
        if self.game_state.game_state() == GameState::InControl {
            self.level.update(process, address);
            self.collectible.update(process, address);
            if let Some(current_level) = self.level.current_level() {
                self.boss.update(process, address, current_level);
            }
        }
    }

    /// Gets the [`BossReader`] in memory.
    #[must_use]
    pub const fn boss_reader(&self) -> &BossReader {
        &self.boss
    }

    /// Gets the [`CollectibleReader`] in memory.
    #[must_use]
    pub const fn collectible_reader(&self) -> &CollectibleReader {
        &self.collectible
    }

    /// Gets the [`GameStateReader`] in memory.
    #[must_use]
    pub const fn game_state_reader(&self) -> &GameStateReader {
        &self.game_state
    }

    /// Gets the [`LevelReader`] in memory.
    #[must_use]
    pub const fn level_reader(&self) -> &LevelReader {
        &self.level
    }

    fn read<T: Pod>(process: &Process, address: Address, path: &[u64]) -> Option<T> {
        process
            .read_pointer_path(address, asr::PointerSize::Bit64, path)
            .ok()
    }
}
