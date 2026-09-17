//! Extracts information from the executable in memory.

pub mod boss;
pub mod collectible;
pub mod level;

use boss::BossReader;
use collectible::CollectibleReader;
use level::LevelReader;

use asr::{Address, Process};
use bytemuck::Pod;

/// Contains memory readers that track the state of the game.
#[derive(Default)]
pub struct Memory {
    boss: BossReader,
    collectible: CollectibleReader,
    level: LevelReader,
}

impl Memory {
    /// Extracts information from the executable in memory, ensuring the game's state is updated.
    /// This method should be called every tick.
    pub fn update(&mut self, process: &Process, address: Address) {
        self.level.update(process, address);
        if let Some(current_level) = self.level.current_level() {
            self.boss.update(process, address, current_level);
        }
        self.collectible.update(process, address);
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
