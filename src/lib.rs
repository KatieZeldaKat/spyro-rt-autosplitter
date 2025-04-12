mod memory;
mod settings;

use std::collections::HashSet;
use memory::{Boss, Memory};
use settings::Settings;
use asr::{
    future::next_tick, print_message, settings::Gui, timer::{self, TimerState}, watcher::Watcher, Process
};

asr::async_main!(stable);

macro_rules! return_if_timer_reset_after {
    ($expression:expr) => {
        $expression;
        if (!timer_running()) {
            return;
        }
    }
}

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
                    run(&memory, &mut settings).await;
                }
            }
        }).await;

        if settings.reset_on_close {
            timer::reset();
        }
    }
}

async fn run<'a>(memory: &Memory::<'a>, settings: &mut Settings) {
    let mut games_started = HashSet::<u8>::new();

    loop {
        return_if_timer_reset_after!(select_game(&memory).await);

        // Only if we haven't entered this game yet should we wait to gain control
        if games_started.insert(memory.read_game()) {
            return_if_timer_reset_after!(gain_control(&memory).await);
        }

        // Update settings; assumes no settings are modified mid-game
        settings.update();

        return_if_timer_reset_after!(run_game(&memory, &settings).await);
        timer::resume_game_time();
    }
}

async fn select_game<'a>(memory: &Memory::<'a>) {
    let is_mid_run = timer_running();
    let mut in_game = Watcher::<bool>::new();
    while !in_game.update_infallible(memory.read_in_game()).changed_to(&true) {
        if is_mid_run {
            return_if_timer_reset_after!(next_tick().await);
        }
        else {
            next_tick().await;
        }
    }

    timer::start();
}

async fn gain_control<'a>(memory: &Memory::<'a>) {
    timer::pause_game_time();

    while !memory.read_in_control() {
        return_if_timer_reset_after!(next_tick().await);
    }

    timer::resume_game_time();
}

async fn run_game<'a>(memory: &Memory::<'a>, settings: &Settings) {
    // Maps
    let mut map_watcher = Watcher::<String>::new();
    let mut has_split = HashSet::<String>::new();

    // Bosses
    let mut boss = Boss::None;
    let mut boss_health_watcher = Watcher::<u8>::new();

    while timer_running() {
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

        // Detect map changes
        if let Some(current_map) = memory.read_map() {
            let map = map_watcher.update_infallible(current_map);
            if map.changed() {
                // Split on map change
                if settings.should_split(&map, &mut has_split) {
                    timer::split();
                }

                // Update current boss
                boss = memory.read_boss();
                if !settings.split_on_boss_kill(boss) {
                    boss = Boss::None;
                }
            }
        }

        // Automatically split on boss kill
        match boss {
            Boss::None => {},
            _ => {
                let current_boss_health = memory.read_boss_health(boss);
                let boss_health = boss_health_watcher.update_infallible(current_boss_health);
                if boss_health.changed_from_to(&1, &0) {
                    timer::split();
                }
            }
        }

        next_tick().await;
    }
}

fn timer_running() -> bool {
    return timer::state() == TimerState::Running || timer::state() == TimerState::Paused;
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
