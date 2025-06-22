use asr::{Address, PointerSize, Process, string::ArrayWString, timer};
use bytemuck::Pod;

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

/// Contains methods to read memory from Spyro: Reignited.
/// Intended to be owned by [`Splitter`](crate::Splitter).
pub struct Memory<'a> {
    process: &'a Process,
    address: Address,
}

impl<'a> Memory<'a> {
    /// Instantiates a new [`Memory`] instance. Since the splitter needs to read the memory
    /// of a process, the instance must live as long as the process provided.
    pub fn new(process: &'a Process, address: Address) -> Self {
        Self { process, address }
    }

    /// Reads the loading/loaded map. This value updates the moment Spyro begins to leave a level,
    /// meaning a level does not need to be loaded yet for a split to register.
    ///
    /// Sometimes, the map cannot be read from memory. In this case, [`None`] is returned. This
    /// is common when exiting boss levels in Spyro: Year of the Dragon, but can happen elsewhere.
    pub fn read_map(&self) -> Option<String> {
        let path = &[0x03415F30, 0x138, 0xB0, 0xB0, 0x598, 0x210, 0xB8, 0x148, 0x190, 0x0];

        // String that looks like a folder path (i.e. "/LS102_StoneHill/Maps/")
        let map = self.read::<ArrayWString<256>>(path)?;
        let map = String::from_utf16(&map).ok()?;

        timer::set_variable("map", &map);

        Some(map)
    }

    /// True if the game is loading, false otherwise. Note that this value shouldn't be the sole
    /// decider to pause game time, so [`read_in_menu()`](Memory::read_in_menu) and
    /// [`read_in_game()`](Memory::read_in_game) should also be considered.
    pub fn read_is_loading(&self) -> bool {
        let path = &[0x03415F30, 0xF8, 0x4A8, 0xE19];

        // Set to 0 when loading, set to 1 otherwise
        let is_loading = self.read::<u8>(path).unwrap_or_default() == 0;

        timer::set_variable("is_loading", &is_loading.to_string());

        is_loading
    }

    /// True if in a menu or graphics submenu, false otherwise.
    /// Timer should always be running if in a menu in the middle of a run.
    pub fn read_in_menu(&self) -> bool {
        let path = &[0x034160D0, 0x20, 0x218, 0x60];

        // Set to 0 in game, 1 if in menu, 15 if in graphics submenu
        let in_menu = self.read::<u8>(path).unwrap_or_default() > 0;

        timer::set_variable("in_menu", &in_menu.to_string());

        in_menu
    }

    /// True if in the game (not in the title screen and main menu), false otherwise.
    pub fn read_in_game(&self) -> bool {
        let path = &[0x03415F30, 0xF0, 0x378, 0x564];

        // Set to 0 in title screen and main menu, set to 1 everywhere else
        let in_game = self.read::<u8>(path).unwrap_or_default() > 0;

        timer::set_variable("in_game", &in_game.to_string());

        in_game
    }

    /// True if Spyro can move, false otherwise. Should be used to await Spyro gaining control
    /// at the beginning of a run, otherwise other methods should be used to determine loading.
    pub fn read_in_control(&self) -> bool {
        let path = &[0x03415F30, 0xF8, 0x478];

        // Set to 0 when immobilized, 1 once Spyro can move
        let in_control = self.read::<u8>(path).unwrap_or_default() > 0;

        timer::set_variable("in_control", &in_control.to_string());

        in_control
    }

    /// A [`Game`] if in a game file (Spyro 1-3), [`None`] otherwise.
    pub fn read_game(&self) -> Option<Game> {
        let path = &[0x03415F30, 0xF8, 0x290, 0x0, 0x1F8];

        // Set to 0 in title screen, 1-3 corresponding to Spyro 1-3
        let game: u8 = self.read(path).unwrap_or_default();

        timer::set_variable("game", &game.to_string());

        match game {
            1 => Some(Game::Spyro1),
            2 => Some(Game::Spyro2),
            3 => Some(Game::Spyro3),
            _ => None,
        }
    }

    /// A [`Boss`] if facing a boss with trackable health, [`None`] otherwise.
    pub fn read_boss(&self) -> Option<Boss> {
        match &self.read_map().unwrap_or_default() as &str {
            "/LS227_RiptosArena/Maps/" => Some(Boss::Ripto(self.read_ripto_health())),
            "/LS335_SorceressLair/Maps/" => {
                Some(Boss::SorceressLair(self.read_sorceress_lair_health()))
            }
            "/LS337_SuperBonusRound/Maps/" => {
                Some(Boss::SorceressSBR(self.read_sorceress_sbr_health()))
            }
            _ => None,
        }
    }

    fn read_ripto_health(&self) -> u8 {
        let path = &[0x03415F30, 0x110, 0x50, 0x140, 0x8, 0x1D0, 0x134];

        // Set to 8 at beginning of fight, decreases towards 0 as damage is taken in 3rd phase
        let ripto_health = self.read(path).unwrap_or(u8::MAX);

        timer::set_variable("ripto_health", &ripto_health.to_string());

        ripto_health
    }

    fn read_sorceress_lair_health(&self) -> u8 {
        let path = &[0x03601278, 0x40, 0x58, 0x20, 0xB0, 0x90, 0x140, 0xA28];

        // Set to 10 at beginning of fight, decreases towards 0 as damage is taken
        let sorceress_lair_health = self.read(path).unwrap_or(u8::MAX);

        timer::set_variable("sorceress_lair_health", &sorceress_lair_health.to_string());

        sorceress_lair_health
    }

    fn read_sorceress_sbr_health(&self) -> u8 {
        let path = &[0x0341B1D0, 0xF8, 0x290, 0x50, 0x8A0, 0xB28];

        // Set to 15 at beginning of fight, decreases towards 0 as damage is taken
        let sorceress_sbr_health = self.read(path).unwrap_or(u8::MAX);

        timer::set_variable("sorceress_sbr_health", &sorceress_sbr_health.to_string());

        sorceress_sbr_health
    }

    /// Given a game, returns the number of collectables earned for that specific game.
    /// This value updates the frame that Spyro gains control after collecting the item.
    ///
    /// # Collectables
    ///
    /// - Spyro the Dragon - Dragons
    /// - Spyro 2: Ripto's Rage - Orbs
    /// - Spyro: Year of the Dragon - Eggs
    pub fn read_collectable_count(&self, game: Game) -> u8 {
        match game {
            Game::Spyro1 => self.read_dragons(),
            Game::Spyro2 => u8::MAX, // Orbs not yet supported
            Game::Spyro3 => self.read_eggs(),
        }
    }

    fn read_dragons(&self) -> u8 {
        let path = &[0x034160D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x27C];

        // The amount of dragons collected when playing Spyro 1
        let dragons = self.read::<u8>(path).unwrap_or_default();

        timer::set_variable("dragons", &dragons.to_string());

        dragons
    }

    fn read_eggs(&self) -> u8 {
        let path = &[0x034160D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x28C];

        // The amount of eggs collected when playing Spyro 3
        let eggs = self.read::<u8>(path).unwrap_or_default();

        timer::set_variable("eggs", &eggs.to_string());

        eggs
    }

    fn read<T: Pod>(&self, path: &[u64]) -> Option<T> {
        self.process
            .read_pointer_path::<T>(self.address, PointerSize::Bit64, path)
            .ok()
    }
}
