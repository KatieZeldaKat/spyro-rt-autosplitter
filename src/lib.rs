pub mod settings;

use std::collections::HashSet;
use settings::{Settings, Split};
use bytemuck::Pod;
use asr::{
    future::next_tick, print_message, settings::Gui, string::ArrayWString, timer::{self, TimerState}, watcher::{Pair, Watcher}, Address, PointerSize, Process
};

asr::async_main!(stable);

const EXE: &str = "Spyro-Win64-Shipping.exe";
const TICK_RATE: i32 = 30;

async fn main() {
    // startup
    asr::set_tick_rate(f64::from(TICK_RATE));
    let mut settings = Settings::register();

    loop {
        let process = Process::wait_attach(EXE).await;
        process.until_closes(async {
            if let Ok(address) = process.get_module_address(EXE) {
                // init
                detect_game_version(&process);

                loop {
                    start(&process, &address).await;
                    settings.update();
                    run(&process, &address, &settings).await;
                }
            }
        }).await;
    }
}

async fn start(process: &Process, address: &Address) {
    // Wait until we enter a game from the title screen before starting timer
    let mut in_game = Watcher::<bool>::new();
    while !in_game.update_infallible(get_in_game(&process, &address)).changed_to(&true) {
        next_tick().await;
    }

    timer::start();
}

async fn run(process: &Process, address: &Address, settings: &Settings) {
    // Maps
    let mut map_watcher = Watcher::<String>::new();
    let mut has_split = HashSet::<String>::new();

    // Game
    let mut game_watcher = Watcher::<u8>::new();
    let mut game_started = HashSet::<u8>::from([get_game(&process, &address)]);

    // Bosses
    let mut ripto_watcher = Watcher::<u8>::new();

    while timer::state() == TimerState::Running || timer::state() == TimerState::Paused {
        // Check if in title or starting a new game
        let game = game_watcher.update_infallible(get_game(&process, &address));
        if game.changed() && game_started.insert(game.current) {
            match game.current {
                0 => if settings.reset_on_title {
                    timer::reset();
                    break;
                },
                1..3 => {
                    timer::pause_game_time();
                    while !get_in_control(&process, &address) {
                        next_tick().await;
                    }
                },
                _ => {},
            }
        }

        // Read memory to determine game timer state
        let is_loading = get_is_loading(&process, &address);
        let in_menu = get_in_menu(&process, &address);
        let in_game = get_in_game(&process, &address);

        // in_menu check prevents abuse of buffering loading and pausing in the exact same frame
        if !is_loading || in_menu || !in_game {
            timer::resume_game_time();
        }
        else if is_loading {
            timer::pause_game_time();
        }

        // Automatically split on map change
        let map = map_watcher.update_infallible(get_map(&process, &address));
        if map.changed() && is_valid_map_transition(&map) {
            match settings.get_map_split_setting(&map.old) {
                Split::FirstTime => if has_split.insert(map.old.clone()) {
                    timer::split();
                },
                Split::EveryTime => {
                    timer::split();
                },
                Split::Never => {},
            };
        }

        // Ripto's Arena
        if map.current == "/LS227_RiptosArena/Maps/" {
            let ripto_health =
                    ripto_watcher.update_infallible(get_ripto_health(&process, &address));

            // Split on kill according to the settings
            if ripto_health.changed_from_to(&1, &0) {
                match settings.s2_ripto_kill {
                    Split::FirstTime => if has_split.insert(String::from("s2_ripto_kill")) {
                        timer::split();
                    },
                    Split::EveryTime => {
                        timer::split();
                    },
                    Split::Never => {},
                }
            }
        }

        next_tick().await;
    }
}

fn detect_game_version(process: &Process) {
    if process.get_module_size(EXE).unwrap() == 61046784 {
        print_message("Spyro Reignited Trilogy WASM started (game version detected: Windows)");
    }
    else if process.get_module_size(EXE).unwrap() == 1052672 {
        print_message("Spyro Reignited Trilogy WASM started (game version detected: Linux)");
    }
    else {
        print_message("Spyro Reignited Trilogy WASM started (unknown game version)");
        print_message(&process.get_module_size(EXE).unwrap().to_string());
    }
}

fn is_valid_map_transition(map: &Pair<String>) -> bool {
    match &map.old as &str {
        "/LS208_CrushsDungeon/Maps/" => return "/LS210_AutumnPlains_Home/Maps/" == map.current,
        "/LS219_GulpsOverlook/Maps/" => return "/LS222_WinterTundra_Home/Maps/" == map.current,
        "/LS227_RiptosArena/Maps/" => return "/LS229_DragonShores/Maps/" == map.current,
        _ => return true,
    }
}

fn get_in_control(process: &Process, address: &Address) -> bool {
    let path = &[0x03415F30, 0xF8, 0x478];
    let in_control_raw = safely_read_memory(&process, &address, path, u8::default());

    // Set to 0 when immobilized, 1 once Spyro can move
    let in_control = in_control_raw > 0;

    timer::set_variable("in_control", &in_control.to_string());

    return in_control;
}

fn get_is_loading(process: &Process, address: &Address) -> bool {
    let path = &[0x03415F30, 0xF8, 0x4A8, 0xE19];
    let is_loading_raw = safely_read_memory(&process, &address, path, u8::default());

    // Set to 0 when loading, set to 1 otherwise
    let is_loading = is_loading_raw == 0;

    timer::set_variable("is_loading", &is_loading.to_string());

    return is_loading;
}

fn get_in_menu(process: &Process, address: &Address) -> bool {
    let path = &[0x034160D0, 0x20, 0x218, 0x60];
    let in_menu_raw = safely_read_memory(&process, &address, path, u8::default());

    // Set to 0 in game, 1 if in menu, 15 if in graphics submenu
    let in_menu = in_menu_raw > 0;

    timer::set_variable("in_menu", &in_menu.to_string());

    return in_menu;
}

fn get_in_game(process: &Process, address: &Address) -> bool {
    let path = &[0x03415F30, 0xF0, 0x378, 0x564];
    let in_game_raw = safely_read_memory(&process, &address, path, u8::default());

    // Set to 0 in title screen and main menu, set to 1 everywhere else
    let in_game = in_game_raw > 0;

    timer::set_variable("in_game", &in_game.to_string());

    return in_game;
}

fn get_game(process: &Process, address: &Address) -> u8 {
    let path = &[0x03415F30, 0xF8, 0x290, 0x0, 0x1F8];

    // Set to 0 in title screen, 1-3 corresponding to Spyro 1-3
    let game = safely_read_memory(&process, &address, path, u8::default());

    timer::set_variable("game", &game.to_string());

    return game;
}

fn get_ripto_health(process: &Process, address: &Address) -> u8 {
    let path = &[0x03415F30, 0x110, 0x50, 0x140, 0x8, 0x1D0, 0x134];

    // Set to 8 at beginning of fight, decreases towards 0 as damage is taken in 3rd phase
    let ripto_health = safely_read_memory(&process, &address, path, 8);

    timer::set_variable("ripto_health", &ripto_health.to_string());

    return ripto_health;
}

fn get_map(process: &Process, address: &Address) -> String {
    let path = &[0x03415F30, 0x138, 0xB0, 0xB0, 0x598, 0x210, 0xB8, 0x148, 0x190, 0x0];
    let map_raw = safely_read_memory(&process, &address, path, ArrayWString::<256>::default());

    // String that looks like a folder path (i.e. "/LS102_StoneHill/Maps/")
    let map = String::from_utf16(&map_raw.as_slice()).unwrap();

    timer::set_variable("map", &map,);

    return map;
}

fn safely_read_memory<T: Pod>(process: &Process, address: &Address, path: &[u64], default: T) -> T {
    if let Ok(data) = process.read_pointer_path::<T>(*address, PointerSize::Bit64, path) {
        return data;
    }

    return default;
}
