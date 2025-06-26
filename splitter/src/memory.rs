mod memory_reader;
pub use memory_reader::MemoryReader;

/// The games present in Spyro: Reignited.
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Game {
    Spyro1,
    Spyro2,
    Spyro3,
}

/// The bosses which can have their health tracked. See [`Boss::health()`].
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Boss {
    Ripto(u8),
    SorceressLair(u8),
    SorceressSBR(u8),
}

impl Boss {
    /// The health a boss currently has. Useful for places where you don't care what boss is
    /// being faced and just need to extract what the current health is.
    pub fn health(&self) -> u8 {
        match *self {
            Boss::Ripto(health) | Boss::SorceressLair(health) | Boss::SorceressSBR(health) => {
                health
            }
        }
    }
}

pub trait Memory {
    /// Reads the loading/loaded map. This value updates the moment Spyro begins to leave a level,
    /// meaning a level does not need to be loaded yet for a split to register.
    ///
    /// Sometimes, the map cannot be read from memory. In this case, [`None`] is returned. This
    /// is common when exiting boss levels in Spyro: Year of the Dragon, but can happen elsewhere.
    fn read_map(&self) -> Option<String>;

    /// True if the game is loading, false otherwise. Note that this value shouldn't be the sole
    /// decider to pause game time, so [`read_in_menu()`](Memory::read_in_menu) and
    /// [`read_in_game()`](Memory::read_in_game) should also be considered.
    fn read_is_loading(&self) -> bool;

    /// True if in a menu or graphics submenu, false otherwise.
    /// Timer should always be running if in a menu in the middle of a run.
    fn read_in_menu(&self) -> bool;

    /// True if in the game (not in the title screen and main menu), false otherwise.
    fn read_in_game(&self) -> bool;

    /// True if Spyro can move, false otherwise. Should be used to await Spyro gaining control
    /// at the beginning of a run, otherwise other methods should be used to determine loading.
    fn read_in_control(&self) -> bool;

    /// A [`Game`] if in a game file (Spyro 1-3), [`None`] otherwise.
    fn read_game(&self) -> Option<Game>;

    /// A [`Boss`] if facing a boss with trackable health, [`None`] otherwise.
    fn read_boss(&self) -> Option<Boss>;

    /// Given a game, returns the number of collectables earned for that specific game.
    /// This value updates the frame that Spyro gains control after collecting the item.
    ///
    /// # Collectables
    ///
    /// - Spyro the Dragon - Dragons
    /// - Spyro 2: Ripto's Rage - Orbs
    /// - Spyro: Year of the Dragon - Eggs
    fn read_collectable_count(&self, game: Game) -> u8;
}
