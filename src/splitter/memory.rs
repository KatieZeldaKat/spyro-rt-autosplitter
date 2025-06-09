use asr::{Address, PointerSize, Process, string::ArrayWString, timer};
use bytemuck::Pod;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Game {
    Spyro1,
    Spyro2,
    Spyro3,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Boss {
    Ripto(u8),
    SorceressLair(u8),
    SorceressSBR(u8),
}

impl Boss {
    pub fn health(&self) -> u8 {
        match *self {
            Boss::Ripto(health) => health,
            Boss::SorceressLair(health) => health,
            Boss::SorceressSBR(health) => health,
        }
    }
}

pub struct Memory<'a> {
    process: &'a Process,
    address: Address,
}

impl<'a> Memory<'a> {
    pub fn new(process: &'a Process, address: Address) -> Self {
        Self { process, address }
    }

    pub fn read_map(&self) -> Option<String> {
        let path = &[0x03415F30, 0x138, 0xB0, 0xB0, 0x598, 0x210, 0xB8, 0x148, 0x190, 0x0];

        // String that looks like a folder path (i.e. "/LS102_StoneHill/Maps/")
        let map = self.read::<ArrayWString<256>>(path)?;
        let map = String::from_utf16(&map).expect("ArrayWString should convert to String.");

        timer::set_variable("map", &map);

        Some(map)
    }

    pub fn read_is_loading(&self) -> bool {
        let path = &[0x03415F30, 0xF8, 0x4A8, 0xE19];

        // Set to 0 when loading, set to 1 otherwise
        let is_loading = self.read::<u8>(path).unwrap_or_default() == 0;

        timer::set_variable("is_loading", &is_loading.to_string());

        is_loading
    }

    pub fn read_in_menu(&self) -> bool {
        let path = &[0x034160D0, 0x20, 0x218, 0x60];

        // Set to 0 in game, 1 if in menu, 15 if in graphics submenu
        let in_menu = self.read::<u8>(path).unwrap_or_default() > 0;

        timer::set_variable("in_menu", &in_menu.to_string());

        in_menu
    }

    pub fn read_in_game(&self) -> bool {
        let path = &[0x03415F30, 0xF0, 0x378, 0x564];

        // Set to 0 in title screen and main menu, set to 1 everywhere else
        let in_game = self.read::<u8>(path).unwrap_or_default() > 0;

        timer::set_variable("in_game", &in_game.to_string());

        in_game
    }

    pub fn read_in_control(&self) -> bool {
        let path = &[0x03415F30, 0xF8, 0x478];

        // Set to 0 when immobilized, 1 once Spyro can move
        let in_control = self.read::<u8>(path).unwrap_or_default() > 0;

        timer::set_variable("in_control", &in_control.to_string());

        in_control
    }

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

    fn read<T: Pod>(&self, path: &[u64]) -> Option<T> {
        self.process
            .read_pointer_path::<T>(self.address, PointerSize::Bit64, path)
            .ok()
    }
}
