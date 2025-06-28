use crate::{Memory, memory::{Game, Boss}};

pub struct MockMemory {
    pub map: Option<String>,
    pub is_loading: bool,
    pub in_menu: bool,
    pub in_game: bool,
    pub in_control: bool,
    pub game: Option<Game>,
    pub boss: Option<Boss>,
    pub collectable_count: u8,
}

impl MockMemory {
    pub fn new() -> Self {
        Self {
            map: None,
            is_loading: false,
            in_menu: false,
            in_game: false,
            in_control: false,
            game: None,
            boss: None,
            collectable_count: 0,
        }
    }
}

impl Default for MockMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for MockMemory {
    fn read_map(&self) -> Option<String> {
        self.map.clone()
    }

    fn read_is_loading(&self) -> bool {
        self.is_loading
    }

    fn read_in_menu(&self) -> bool {
        self.in_menu
    }

    fn read_in_game(&self) -> bool {
        self.in_game
    }

    fn read_in_control(&self) -> bool {
        self.in_control
    }

    fn read_game(&self) -> Option<Game> {
        self.game
    }

    fn read_boss(&self) -> Option<Boss> {
        self.boss
    }

    fn read_collectable_count(&self, _: Game) -> u8 {
        self.collectable_count
    }
}
