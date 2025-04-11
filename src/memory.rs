use asr::{string::ArrayWString, timer, Address, PointerSize, Process};
use bytemuck::Pod;

pub struct Memory<'a> {
    process: &'a Process,
    address: Address,
}

impl<'a> Memory<'a> {
    pub fn new(process: &'a Process, address: Address) -> Self {
        return Self {
            process,
            address,
        };
    }

    pub fn read_map(&self) -> Option<String> {
        let path = &[0x03415F30, 0x138, 0xB0, 0xB0, 0x598, 0x210, 0xB8, 0x148, 0x190, 0x0];
        if let Some(map_raw) = self.read_raw::<ArrayWString::<256>>(path) {
            // String that looks like a folder path (i.e. "/LS102_StoneHill/Maps/")
            let map = String::from_utf16(map_raw.as_slice()).unwrap();

            timer::set_variable("map", &map);
    
            return Some(map);
        }
        
        return None;
    }

    pub fn read_is_loading(&self) -> bool {
        let path = &[0x03415F30, 0xF8, 0x4A8, 0xE19];
        let is_loading_raw = self.read_raw_or_default(path, u8::default());

        // Set to 0 when loading, set to 1 otherwise
        let is_loading = is_loading_raw == 0;

        timer::set_variable("is_loading", &is_loading.to_string());

        return is_loading;
    }

    pub fn read_in_menu(&self) -> bool {
        let path = &[0x034160D0, 0x20, 0x218, 0x60];
        let in_menu_raw = self.read_raw_or_default(path, u8::default());

        // Set to 0 in game, 1 if in menu, 15 if in graphics submenu
        let in_menu = in_menu_raw > 0;

        timer::set_variable("in_menu", &in_menu.to_string());

        return in_menu;
    }

    pub fn read_in_game(&self) -> bool {
        let path = &[0x03415F30, 0xF0, 0x378, 0x564];
        let in_game_raw = self.read_raw_or_default(path, u8::default());

        // Set to 0 in title screen and main menu, set to 1 everywhere else
        let in_game = in_game_raw > 0;

        timer::set_variable("in_game", &in_game.to_string());

        return in_game;
    }

    pub fn read_in_control(&self) -> bool {
        let path = &[0x03415F30, 0xF8, 0x478];
        let in_control_raw = self.read_raw_or_default(path, u8::default());

        // Set to 0 when immobilized, 1 once Spyro can move
        let in_control = in_control_raw > 0;

        timer::set_variable("in_control", &in_control.to_string());

        return in_control;
    }

    pub fn read_game(&self) -> u8 {
        let path = &[0x03415F30, 0xF8, 0x290, 0x0, 0x1F8];

        // Set to 0 in title screen, 1-3 corresponding to Spyro 1-3
        let game = self.read_raw_or_default(path, u8::default());

        timer::set_variable("game", &game.to_string());

        return game;
    }

    pub fn read_ripto_health(&self) -> u8 {
        let path = &[0x03415F30, 0x110, 0x50, 0x140, 0x8, 0x1D0, 0x134];

        // Set to 8 at beginning of fight, decreases towards 0 as damage is taken in 3rd phase
        let ripto_health = self.read_raw_or_default(path, 8);

        timer::set_variable("ripto_health", &ripto_health.to_string());

        return ripto_health;
    }

    pub fn read_sorceress_lair_health(&self) -> u8 {
        let path = &[0x03601278, 0x40, 0x58, 0x20, 0xB0, 0x90, 0x140, 0xA28];

        // Set to 10 at beginning of fight, decreases towards 0 as damage is taken
        let sorceress_lair_health = self.read_raw_or_default(path, 10);

        timer::set_variable("sorceress_lair_health", &sorceress_lair_health.to_string());

        return sorceress_lair_health;
    }

    fn read_raw<T: Pod>(&self, path: &[u64]) -> Option<T> {
        if let Ok(data) = self.process.read_pointer_path::<T>(
            self.address,
            PointerSize::Bit64,
            path,
        ) {
            return Some(data);
        }

        return None;
    }

    fn read_raw_or_default<T: Pod>(&self, path: &[u64], default: T) -> T {
        if let Some(data) = self.read_raw::<T>(path) {
            return data;
        }

        return default;
    }
}
