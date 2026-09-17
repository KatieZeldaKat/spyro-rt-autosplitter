//! Extracts information from the executable in memory.

pub mod boss;
pub mod level;

use boss::BossReader;
use level::LevelReader;

use asr::{Address, Process};
use bytemuck::Pod;

/// Contains memory readers that track the state of the game.
#[derive(Default)]
pub struct Memory {
    boss_reader: BossReader,
    level_reader: LevelReader,
}

impl Memory {
    /// Extracts information from the executable in memory, ensuring the game's state is updated.
    /// This method should be called every tick.
    pub fn update(&mut self, process: &Process, address: Address) {
        self.level_reader.update(process, address);
        if let Some(current_level) = self.level_reader.current_level() {
            self.boss_reader.update(process, address, current_level);
        }
    }

    /// Gets the [`BossReader`] in memory.
    #[must_use]
    pub const fn boss_reader(&self) -> &BossReader {
        &self.boss_reader
    }

    /// Gets the [`LevelReader`] in memory.
    #[must_use]
    pub const fn level_reader(&self) -> &LevelReader {
        &self.level_reader
    }

    fn read<T: Pod>(process: &Process, address: Address, path: &[u64]) -> Option<T> {
        process
            .read_pointer_path(address, asr::PointerSize::Bit64, path)
            .ok()
    }
}
