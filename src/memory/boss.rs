//! The primary bosses that end an Any% run.

use super::{Memory, PointerPaths, level::Level};
#[cfg(debug_assertions)]
use asr::timer;
use asr::{Address, Process, watcher::Watcher};

/// The bosses in the game whose health can be read from memory.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Boss {
    Ripto,
    SorceressLair,
    SorceressSbr,
}

/// Extracts and caches information about [`Boss`]' health.
#[derive(Default)]
pub struct BossReader {
    current_level: Option<Level>,

    ripto_health: Watcher<u8>,
    sorceress_lair_health: Watcher<u8>,
    sorceress_sbr_health: Watcher<u8>,
}

impl BossReader {
    /// Updates the health of the current [`Boss`] (if there is one).
    /// This should only be called by [`Memory`].
    pub fn update(
        &mut self,
        process: &Process,
        address: Address,
        paths: &PointerPaths,
        current_level: Level,
    ) {
        self.current_level = Some(current_level);
        match current_level {
            Level::RiptosArena => {
                if let Some(health) = Self::read_ripto_health(process, address, paths) {
                    self.ripto_health.update_infallible(health);
                }
            }
            Level::SorceresssLair => {
                if let Some(health) = Self::read_sorceress_lair_health(process, address, paths) {
                    self.sorceress_lair_health.update_infallible(health);
                }
            }
            Level::SuperBonusRound => {
                if let Some(health) = Self::read_sorceress_sbr_health(process, address, paths) {
                    self.sorceress_sbr_health.update_infallible(health);
                }
            }
            _ => (),
        }
    }

    /// Returns a [`Boss`] if it was just defeated, [`None`] otherwise.
    #[must_use]
    pub fn boss_defeated(&self) -> Option<Boss> {
        match self.current_level? {
            Level::RiptosArena => self
                .ripto_health
                .pair?
                .changed_from_to(&1, &0)
                .then_some(Boss::Ripto),
            Level::SorceresssLair => self
                .sorceress_lair_health
                .pair?
                .changed_from_to(&1, &0)
                .then_some(Boss::SorceressLair),
            Level::SuperBonusRound => self
                .sorceress_sbr_health
                .pair?
                .changed_from_to(&1, &0)
                .then_some(Boss::SorceressSbr),
            _ => None,
        }
    }

    fn read_ripto_health(process: &Process, address: Address, paths: &PointerPaths) -> Option<u8> {
        let health = Memory::read::<u8>(process, address, &paths.ripto_health)?;

        #[cfg(debug_assertions)]
        timer::set_variable("ripto_health", &health.to_string());

        Some(health)
    }

    fn read_sorceress_lair_health(
        process: &Process,
        address: Address,
        paths: &PointerPaths,
    ) -> Option<u8> {
        let health = Memory::read::<u8>(process, address, &paths.sorceress_lair_health)?;

        #[cfg(debug_assertions)]
        timer::set_variable("sorceress_lair_health", &health.to_string());

        Some(health)
    }

    fn read_sorceress_sbr_health(
        process: &Process,
        address: Address,
        paths: &PointerPaths,
    ) -> Option<u8> {
        let health = Memory::read::<u8>(process, address, &paths.sorceress_sbr_health)?;

        #[cfg(debug_assertions)]
        timer::set_variable("sorceress_sbr_health", &health.to_string());

        Some(health)
    }
}
