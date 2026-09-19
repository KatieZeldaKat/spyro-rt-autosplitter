//! Extracts information from the executable in memory.

pub mod boss;
pub mod collectible;
pub mod game_state;
pub mod level;
pub mod loading;

use boss::BossReader;
use collectible::CollectibleReader;
use game_state::{GameState, GameStateReader};
use level::LevelReader;
use loading::LoadStateReader;

use asr::{Address, Process};
use bytemuck::Pod;

/// The versions of Spyro: Reignited supported by this auto-splitter.
#[derive(Default)]
pub enum GameVersion {
    #[default]
    Steam,
    GamePass,
}

impl GameVersion {
    /// Detects the game version based on the module size in memory.
    #[must_use]
    pub fn from_module_size(module_size: u64) -> Option<Self> {
        match module_size {
            61_046_784 | 1_052_672 => Some(Self::Steam),
            95_162_368 => Some(Self::GamePass),
            size => {
                #[cfg(debug_assertions)]
                asr::print_message(&format!("Unknown game version (module size = `{size}`)"));

                None
            }
        }
    }
}

/// Contains memory readers that track the state of the game.
#[derive(Default)]
pub struct Memory {
    boss: BossReader,
    collectible: CollectibleReader,
    game_state: GameStateReader,
    level: LevelReader,
    load_state: LoadStateReader,
}

impl Memory {
    /// Extracts information from the executable in memory, ensuring the game's state is updated.
    /// This method should be called every tick.
    pub fn update(&mut self, process: &Process, address: Address, paths: &PointerPaths) {
        self.game_state.update(process, address, paths);
        self.load_state
            .update(process, address, paths, self.game_state.game_state());
        if self.game_state.game_state() == GameState::InControl {
            self.level.update(process, address, paths);
            self.collectible
                .update(process, address, paths, self.game_state.game());
            if let Some(current_level) = self.level.current_level() {
                self.boss.update(process, address, paths, current_level);
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

    /// Gets the [`LoadStateReader`] in memory.
    #[must_use]
    pub const fn load_state_reader(&self) -> &LoadStateReader {
        &self.load_state
    }

    fn read<T: Pod>(process: &Process, address: Address, path: &[u64]) -> Option<T> {
        process
            .read_pointer_path(address, asr::PointerSize::Bit64, path)
            .ok()
    }
}

/// The locations in memory where pertinent information for the speedrun can be found.
///
/// See also: [`Process::read_pointer_path`].
pub struct PointerPaths {
    level: Box<[u64]>,
    loading: Box<[u64]>,
    on_title: Box<[u64]>,
    in_menu: Box<[u64]>,
    in_control: Box<[u64]>,
    game: Box<[u64]>,
    dragon_count: Box<[u64]>,
    egg_count: Box<[u64]>,
    ripto_health: Box<[u64]>,
    sorceress_lair_health: Box<[u64]>,
    sorceress_sbr_health: Box<[u64]>,
}

impl From<GameVersion> for PointerPaths {
    fn from(game_version: GameVersion) -> Self {
        match game_version {
            GameVersion::Steam => Self {
                level: Box::new([
                    0x0341_5F30,
                    0x138,
                    0xB0,
                    0xB0,
                    0x598,
                    0x210,
                    0xB8,
                    0x148,
                    0x190,
                    0x0,
                ]),
                loading: Box::new([0x0341_5F30, 0xF8, 0x4A8, 0xE19]),
                on_title: Box::new([0x0341_5F30, 0xF0, 0x378, 0x564]),
                in_menu: Box::new([0x0341_60D0, 0x20, 0x218, 0x60]),
                in_control: Box::new([0x0341_5F30, 0xF8, 0x478]),
                game: Box::new([0x0341_5F30, 0xF8, 0x290, 0x0, 0x1F8]),
                dragon_count: Box::new([0x0341_60D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x27C]),
                egg_count: Box::new([0x0341_60D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x28C]),
                ripto_health: Box::new([0x0341_5F30, 0x110, 0x50, 0x140, 0x8, 0x1D0, 0x134]),
                sorceress_lair_health: Box::new([
                    0x0360_1278,
                    0x40,
                    0x58,
                    0x20,
                    0xB0,
                    0x90,
                    0x140,
                    0xA28,
                ]),
                sorceress_sbr_health: Box::new([0x0341_B1D0, 0xF8, 0x290, 0x50, 0x8A0, 0xB28]),
            },
            GameVersion::GamePass => Self {
                level: Box::new([
                    0x054A_0CA0,
                    0x138,
                    0xB0,
                    0xB0,
                    0x598,
                    0x210,
                    0xB8,
                    0x148,
                    0x190,
                    0x0,
                ]),
                loading: Box::new([0x054A_0CA0, 0xF8, 0x4A8, 0xE19]),
                on_title: Box::new([0x054A_0CA0, 0xF0, 0x378, 0x564]),
                in_menu: Box::new([0x054A_12D0, 0x20, 0x218, 0x60]),
                in_control: Box::new([0x054A_0CA0, 0xF8, 0x478]),
                game: Box::new([0x054A_0CA0, 0xF8, 0x290, 0x0, 0x1F8]),
                dragon_count: Box::new([0x054A_12D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x27C]),
                egg_count: Box::new([0x054A_12D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x28C]),
                ripto_health: Box::new([0x054A_0CA0, 0x110, 0x50, 0x140, 0x8, 0x1D0, 0x134]),
                sorceress_lair_health: Box::new([
                    0x054A_0CA0,
                    0x30,
                    0xA0,
                    0xE8,
                    0xE90,
                    0x108,
                    0x3E8,
                    0xA28,
                ]),
                sorceress_sbr_health: Box::new([0x054A_B670, 0xF8, 0x290, 0x50, 0x8A0, 0xB28]),
            },
        }
    }
}
