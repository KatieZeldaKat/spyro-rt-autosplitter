//! The primary objectives in Spyro.

use super::Memory;
#[cfg(debug_assertions)]
use asr::timer;
use asr::{Address, Process, watcher::Watcher};

/// The collectibles whose values can be read from memory.
///
/// The associated value is the total number of this collectible earned so far.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Collectible {
    /// Only available when playing Spyro the Dragon.
    Dragon(u8),

    /// Only available when playing Spyro: Year of the Dragon.
    Egg(u8),
}

#[derive(Clone, Copy)]
enum Game {
    Spyro1,
    Spyro3,
}

/// Extracts and caches information about the games' [`Collectible`]s.
#[derive(Default)]
pub struct CollectibleReader {
    game: Option<Game>,

    dragon_count: Watcher<u8>,
    egg_count: Watcher<u8>,
}

impl CollectibleReader {
    /// Updates the number of [`Collectible`]s earned for the current game.
    /// This should only be called by [`Memory`].
    pub fn update(&mut self, process: &Process, address: Address) {
        self.game = Self::read_game(process, address);
        if let Some(game) = &self.game {
            match game {
                Game::Spyro1 => {
                    if let Some(dragon_count) = Self::read_dragon_count(process, address) {
                        self.dragon_count.update_infallible(dragon_count);
                    }
                }
                Game::Spyro3 => {
                    if let Some(egg_count) = Self::read_egg_count(process, address) {
                        self.egg_count.update_infallible(egg_count);
                    }
                }
            }
        }
    }

    /// Returns a [`Collectible`] if one was just earned, [`None`] otherwise.
    #[must_use]
    pub fn collectible_earned(&self) -> Option<Collectible> {
        match self.game? {
            Game::Spyro1 => {
                let dragons = self.dragon_count.pair?;
                dragons
                    .changed()
                    .then_some(Collectible::Dragon(dragons.current))
            }
            Game::Spyro3 => {
                let eggs = self.egg_count.pair?;
                eggs.changed().then_some(Collectible::Egg(eggs.current))
            }
        }
    }

    fn read_game(process: &Process, address: Address) -> Option<Game> {
        let path = &[0x0341_5F30, 0xF8, 0x290, 0x0, 0x1F8];
        let game = Memory::read::<u8>(process, address, path)?;

        #[cfg(debug_assertions)]
        timer::set_variable("game", &game.to_string());

        match game {
            1 => Some(Game::Spyro1),
            3 => Some(Game::Spyro3),
            _ => None,
        }
    }

    fn read_dragon_count(process: &Process, address: Address) -> Option<u8> {
        let path = &[0x0341_60D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x27C];
        let dragon_count = Memory::read::<u8>(process, address, path)?;

        #[cfg(debug_assertions)]
        timer::set_variable("dragon_count", &dragon_count.to_string());

        Some(dragon_count)
    }

    fn read_egg_count(process: &Process, address: Address) -> Option<u8> {
        let path = &[0x0341_60D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x28C];
        let egg_count = Memory::read::<u8>(process, address, path)?;

        #[cfg(debug_assertions)]
        timer::set_variable("egg_count", &egg_count.to_string());

        Some(egg_count)
    }
}
