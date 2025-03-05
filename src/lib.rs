use asr::{future::next_tick, print_message, timer, Address, PointerSize, Process, watcher::Watcher};
asr::async_main!(stable);

const EXE: &str = "Spyro-Win64-Shipping.exe";

async fn main() {
    // startup
    asr::set_tick_rate(30.0);

    loop {
        let process = Process::wait_attach(EXE).await;
        process.until_closes(async {
            if let Ok(address) = process.get_module_address(EXE) {
                // init
                detect_game_version(&process);

                let mut is_loading: Watcher<bool> = Watcher::new();
                let mut in_menu: Watcher<bool> = Watcher::new();
                let mut in_game: Watcher<bool> = Watcher::new();

                // update
                loop {
                    is_loading.update_infallible(get_is_loading(&process, &address));
                    in_menu.update_infallible(get_in_menu(&process, &address));
                    in_game.update_infallible(get_in_game(&process, &address));

                    // start
                    if timer::state() != timer::TimerState::Running &&
                        in_game.pair.unwrap_or_default().changed_from_to(
                            &false, &true
                    ) {
                        timer::start();
                    }

                    next_tick().await;
                }
            }
        }).await;
    }
}

fn detect_game_version(process: &Process) {
    if process.get_module_size(EXE).unwrap() == 61046784 {
        print_message("Spyro Reignited Trilogy ASL started (game version detected: Release)");
    }
    else {
        print_message("Spyro Reignited Trilogy ASL started (unknown game version)");
        print_message(&process.get_module_size(EXE).unwrap().to_string());
    }
}

fn get_is_loading(process: &Process, address: &Address) -> bool {
    if let Ok(is_not_loading_raw) = process.read_pointer_path::<u8>(
        *address,
        PointerSize::Bit64,
        &[0x03415F30, 0xF8, 0x4A8, 0xE19]
    ) {
        // Set to 0 when loading, set to 1 otherwise
        let is_loading_value = is_not_loading_raw == 0;

        timer::set_variable(
            "is_loading",
            &is_loading_value.to_string()
        );

        return is_loading_value;
    }

    return true;
}

fn get_in_menu(process: &Process, address: &Address) -> bool {
    if let Ok(in_menu_raw) = process.read_pointer_path::<u8>(
        *address,
        PointerSize::Bit64,
        &[0x034160D0, 0x20, 0x218, 0x60]
    ) {
        // Set to 0 in game, 1 if in menu, 15 if in graphics submenu
        let in_menu_value = in_menu_raw > 0;

        timer::set_variable(
            "in_menu",
            &in_menu_value.to_string()
        );

        return in_menu_value;
    }

    return true;
}

fn get_in_game(process: &Process, address: &Address) -> bool {
    if let Ok(in_game_raw) = process.read_pointer_path::<u8>(
        *address,
        PointerSize::Bit64,
        &[0x03415F30, 0xF0, 0x378, 0x564]
    ) {
        // Set to 0 in title screen and main menu, set to 1 everywhere else
        let in_game_value = in_game_raw > 0;

        timer::set_variable(
            "in_game",
            &in_game_value.to_string()
        );

        return in_game_value;
    }

    return false;
}
