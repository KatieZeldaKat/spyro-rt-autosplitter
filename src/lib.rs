mod memory;
mod settings;

use std::collections::HashSet;
use memory::Memory;
use settings::{Settings, Split};
use asr::{
    future::next_tick, print_message, settings::Gui, timer::{self, TimerState}, watcher::{Pair, Watcher}, Process
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
                let memory = Memory::new(&process, address);

                loop {
                    start_run(&memory, &mut settings).await;
                }
            }
        }).await;

        if settings.reset_on_close {
            timer::reset();
        }
    }
}

async fn start_run<'a>(memory: &Memory::<'a>, settings: &mut Settings) {
    let mut games_started = HashSet::<u8>::new();

    loop {
        // Wait until a game is selected
        let timer_was_running = timer_running();
        let mut in_game = Watcher::<bool>::new();
        while !in_game.update_infallible(memory.read_in_game()).changed_to(&true) {
            if timer_was_running && !timer_running() {
                return;
            }
            next_tick().await;
        }

        // If it wasn't yet selected this run, wait until the player has control to start time
        if games_started.insert(memory.read_game()) {
            timer::start();
            timer::pause_game_time();

            // Wait to gain control
            while !memory.read_in_control() {
                next_tick().await;
            }
            if !timer_running() {
                return;
            }
        }

        timer::resume_game_time();

        // Update settings; assumes no settings are modified mid-game
        settings.update();

        // Main autosplitter logic
        continue_run(&memory, &settings).await;
        if !timer_running() {
            return;
        }

        timer::resume_game_time();
    }
}

async fn continue_run<'a>(memory: &Memory::<'a>, settings: &Settings) {
    // Maps
    let mut map_watcher = Watcher::<String>::new();
    let mut has_split = HashSet::<String>::new();

    // Bosses
    let mut boss_watcher = Watcher::<u8>::new();

    while timer::state() == TimerState::Running || timer::state() == TimerState::Paused {
        // If no longer in game, exit
        let in_game = memory.read_in_game();
        if !in_game {
            if settings.reset_on_title {
                timer::reset();
            }

            return;
        }

        // Read memory to determine game timer state
        let is_loading = memory.read_is_loading();
        let in_menu = memory.read_in_menu();

        // in_menu check prevents abuse of buffering loading and pausing in the exact same frame
        if !is_loading || in_menu || !in_game {
            timer::resume_game_time();
        }
        else if is_loading {
            timer::pause_game_time();
        }

        // Automatically split on map change
        if let Some(current_map) = memory.read_map() {
            let map = map_watcher.update_infallible(current_map);
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

            // Split on kill for boss fights
            match &map.current as &str {
                "/LS227_RiptosArena/Maps/" => {
                    let health = memory.read_ripto_health();
                    if boss_killed(&mut boss_watcher, health) && settings.s2_ripto_kill {
                        timer::split();
                    }
                },
                "/LS335_SorceressLair/Maps/" => {
                    let health = memory.read_sorceress_lair_health();
                    if boss_killed(&mut boss_watcher, health) && settings.s3_sorceress_lair_kill {
                        timer::split();
                    }
                },
                _ => {}
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

fn timer_running() -> bool {
    return timer::state() == TimerState::Running || timer::state() == TimerState::Paused;
}

fn is_valid_map_transition(map: &Pair<String>) -> bool {
    match &map.old as &str {
        "/LS208_CrushsDungeon/Maps/" => return "/LS210_AutumnPlains_Home/Maps/" == map.current,
        "/LS219_GulpsOverlook/Maps/" => return "/LS222_WinterTundra_Home/Maps/" == map.current,
        "/LS227_RiptosArena/Maps/" => return "/LS229_DragonShores/Maps/" == map.current,
        _ => return true,
    }
}

fn boss_killed(boss_watcher: &mut Watcher<u8>, current_health: u8) -> bool {
    let health = boss_watcher.update_infallible(current_health);
    return health.changed_from_to(&1, &0);
}
