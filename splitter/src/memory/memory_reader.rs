use super::{Boss, Game, Memory};
use asr::{Address, PointerSize, Process, string::ArrayWString, timer};
use bytemuck::Pod;

/// Contains methods to read memory from Spyro: Reignited without the need to pass pointer paths.
pub struct MemoryReader<'a> {
    process: &'a Process,
    address: Address,

    map_path: Box<[u64]>,
    loading_path: Box<[u64]>,
    in_menu_path: Box<[u64]>,
    in_game_path: Box<[u64]>,
    in_control_path: Box<[u64]>,
    game_path: Box<[u64]>,
    ripto_path: Box<[u64]>,
    sorceress_lair_path: Box<[u64]>,
    sorceress_sbr_path: Box<[u64]>,
    dragons_path: Box<[u64]>,
    eggs_path: Box<[u64]>,
}

impl<'a> MemoryReader<'a> {
    /// Instantiates a new [`MemoryReader`] instance to read from the Steam version.
    pub fn new_steam(process: &'a Process, address: Address) -> Self {
        Self {
            process,
            address,

            map_path: Box::new([
                0x03415F30, 0x138, 0xB0, 0xB0, 0x598, 0x210, 0xB8, 0x148, 0x190, 0x0,
            ]),
            loading_path: Box::new([0x03415F30, 0xF8, 0x4A8, 0xE19]),
            in_menu_path: Box::new([0x034160D0, 0x20, 0x218, 0x60]),
            in_game_path: Box::new([0x03415F30, 0xF0, 0x378, 0x564]),
            in_control_path: Box::new([0x03415F30, 0xF8, 0x478]),
            game_path: Box::new([0x03415F30, 0xF8, 0x290, 0x0, 0x1F8]),
            ripto_path: Box::new([0x03415F30, 0x110, 0x50, 0x140, 0x8, 0x1D0, 0x134]),
            sorceress_lair_path: Box::new([0x03601278, 0x40, 0x58, 0x20, 0xB0, 0x90, 0x140, 0xA28]),
            sorceress_sbr_path: Box::new([0x0341B1D0, 0xF8, 0x290, 0x50, 0x8A0, 0xB28]),
            dragons_path: Box::new([0x034160D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x27C]),
            eggs_path: Box::new([0x034160D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x28C]),
        }
    }

    /// Instantiates a new [`MemoryReader`] instance to read from the Game Pass version.
    pub fn new_game_pass(process: &'a Process, address: Address) -> Self {
        Self {
            process,
            address,

            map_path: Box::new([
                0x054A0CA0, 0x138, 0xB0, 0xB0, 0x598, 0x210, 0xB8, 0x148, 0x190, 0x0,
            ]),
            loading_path: Box::new([0x054A0CA0, 0xF8, 0x4A8, 0xE19]),
            in_menu_path: Box::new([0x054A12D0, 0x20, 0x218, 0x60]),
            in_game_path: Box::new([0x054A0CA0, 0xF0, 0x378, 0x564]),
            in_control_path: Box::new([0x054A0CA0, 0xF8, 0x478]),
            game_path: Box::new([0x054A0CA0, 0xF8, 0x290, 0x0, 0x1F8]),
            ripto_path: Box::new([0x054A0CA0, 0x110, 0x50, 0x140, 0x8, 0x1D0, 0x134]),
            // Can flicker between health and `null` on occassion; should be further tested
            sorceress_lair_path: Box::new([
                0x054A0CA0, 0x30, 0xA0, 0xE8, 0xE90, 0x108, 0x3E8, 0xA28,
            ]),
            sorceress_sbr_path: Box::new([0x054AB670, 0xF8, 0x290, 0x50, 0x8A0, 0xB28]),
            dragons_path: Box::new([0x054A12D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x27C]),
            eggs_path: Box::new([0x054A12D0, 0x28, 0x20, 0x100, 0x8, 0x30, 0x28C]),
        }
    }

    fn read<T: Pod>(&self, path: &[u64]) -> Option<T> {
        self.process
            .read_pointer_path::<T>(self.address, PointerSize::Bit64, path)
            .ok()
    }

    // Set to 8 at beginning of fight, decreases towards 0 as damage is taken in 3rd phase
    fn read_ripto_health(&self) -> u8 {
        let ripto_health = self.read(&self.ripto_path).unwrap_or(u8::MAX);

        timer::set_variable("ripto_health", &ripto_health.to_string());

        ripto_health
    }

    // Set to 10 at beginning of fight, decreases towards 0 as damage is taken
    fn read_sorceress_lair_health(&self) -> u8 {
        let sorceress_lair_health = self.read(&self.sorceress_lair_path).unwrap_or(u8::MAX);

        timer::set_variable("sorceress_lair_health", &sorceress_lair_health.to_string());

        sorceress_lair_health
    }

    // Set to 15 at beginning of fight, decreases towards 0 as damage is taken
    fn read_sorceress_sbr_health(&self) -> u8 {
        let sorceress_sbr_health = self.read(&self.sorceress_sbr_path).unwrap_or(u8::MAX);

        timer::set_variable("sorceress_sbr_health", &sorceress_sbr_health.to_string());

        sorceress_sbr_health
    }

    // The amount of dragons collected when playing Spyro 1
    fn read_dragons(&self) -> u8 {
        let dragons = self.read::<u8>(&self.dragons_path).unwrap_or_default();

        timer::set_variable("dragons", &dragons.to_string());

        dragons
    }

    // The amount of eggs collected when playing Spyro 3
    fn read_eggs(&self) -> u8 {
        let eggs = self.read::<u8>(&self.eggs_path).unwrap_or_default();

        timer::set_variable("eggs", &eggs.to_string());

        eggs
    }
}

impl Memory for MemoryReader<'_> {
    // String that looks like a folder path (i.e. "/LS102_StoneHill/Maps/")
    fn read_map(&self) -> Option<String> {
        let map = self.read::<ArrayWString<256>>(&self.map_path)?;
        let map = String::from_utf16(&map).ok()?;

        timer::set_variable("map", &map);

        Some(map)
    }

    // Set to 0 when loading, set to 1 otherwise
    fn read_is_loading(&self) -> bool {
        let is_loading = self.read::<u8>(&self.loading_path).unwrap_or_default() == 0;

        timer::set_variable("is_loading", &is_loading.to_string());

        is_loading
    }

    // Set to 0 in game, 1 if in menu, 15 if in graphics submenu
    fn read_in_menu(&self) -> bool {
        let in_menu = self.read::<u8>(&self.in_menu_path).unwrap_or_default() > 0;

        timer::set_variable("in_menu", &in_menu.to_string());

        in_menu
    }

    // Set to 0 in title screen and main menu, set to 1 everywhere else
    fn read_in_game(&self) -> bool {
        let in_game = self.read::<u8>(&self.in_game_path).unwrap_or_default() > 0;

        timer::set_variable("in_game", &in_game.to_string());

        in_game
    }

    // Set to 0 when immobilized, 1 once Spyro can move
    fn read_in_control(&self) -> bool {
        let in_control = self.read::<u8>(&self.in_control_path).unwrap_or_default() > 0;

        timer::set_variable("in_control", &in_control.to_string());

        in_control
    }

    // Set to 0 in title screen, 1-3 corresponding to Spyro 1-3
    fn read_game(&self) -> Option<Game> {
        let game: u8 = self.read(&self.game_path).unwrap_or_default();

        timer::set_variable("game", &game.to_string());

        match game {
            1 => Some(Game::Spyro1),
            2 => Some(Game::Spyro2),
            3 => Some(Game::Spyro3),
            _ => None,
        }
    }

    fn read_boss(&self) -> Option<Boss> {
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

    fn read_collectable_count(&self, game: Game) -> u8 {
        match game {
            Game::Spyro1 => self.read_dragons(),
            Game::Spyro2 => u8::MAX, // Orbs not yet supported
            Game::Spyro3 => self.read_eggs(),
        }
    }
}
