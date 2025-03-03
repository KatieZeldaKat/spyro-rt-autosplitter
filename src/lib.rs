use asr::{future::next_tick, Process};
asr::async_main!(stable);

const EXE: &str = "Spyro-Win64-Shipping.exe";

async fn main() {
    loop {
        let process = Process::wait_attach(EXE).await;
        process.until_closes(async {
            if let Ok(address) = process.get_module_address(EXE) {
                detect_game_version(&process);

                let mut is_loading: bool;
                let mut in_menu: bool;
                let mut in_game: bool;

                loop {
                    // is_loading
                    if let Ok(is_not_loading_value) = process.read_pointer_path::<u8>(
                        address,
                        asr::PointerSize::Bit64,
                        &[0x03415F30, 0xF8, 0x4A8, 0xE19]
                    ) {
                        is_loading = is_not_loading_value == 0;
                        asr::timer::set_variable(
                            "is_loading",
                            &is_loading.to_string()
                        );
                    }

                    // in_menu
                    if let Ok(in_menu_value) = process.read_pointer_path::<u8>(
                        address,
                        asr::PointerSize::Bit64,
                        &[0x034160D0, 0x20, 0x218, 0x60]
                    ) {
                        in_menu = in_menu_value > 0;
                        asr::timer::set_variable(
                            "in_menu",
                            &in_menu.to_string()
                        );
                    }

                    // in_game
                    if let Ok(in_game_value) = process.read_pointer_path::<u8>(
                        address,
                        asr::PointerSize::Bit64,
                        &[0x03415F30, 0xF0, 0x378, 0x564]
                    ) {
                        in_game = in_game_value > 0;
                        asr::timer::set_variable(
                            "in_game",
                            &in_game.to_string()
                        );
                    }

                    next_tick().await;
                }
            }
        }).await;
    }
}

fn detect_game_version(process: &Process) {
    if process.get_module_size(EXE).unwrap() == 61046784 {
        asr::print_message("Spyro Reignited Trilogy ASL started (game version detected: Release)");
    }
    else {
        asr::print_message("Spyro Reignited Trilogy ASL started (unknown game version)");
        asr::print_message(&process.get_module_size(EXE).unwrap().to_string());
    }
}
