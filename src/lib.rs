use asr::{Process, print_message, settings::Gui};
use splitter::{Splitter, memory::MemoryReader, settings::Settings};

asr::async_main!(stable);

const EXE: &str = "Spyro-Win64-Shipping.exe";
const TICK_RATE: u8 = 30;

enum GameVersion {
    Steam,
    GamePass,
    Unknown,
}

/// The entry point to the auto-splitter. More info can be found in [`Splitter`].
pub async fn main() {
    print_message("Spyro: Reignited WASM started.");
    asr::set_tick_rate(f64::from(TICK_RATE));
    let mut settings = Settings::register();

    loop {
        let process = Process::wait_attach(EXE).await;
        process
            .until_closes(async {
                if let Ok((address, module_size)) = process.get_module_range(EXE) {
                    let memory = match get_game_version(module_size) {
                        GameVersion::Steam => MemoryReader::new_steam(&process, address),
                        GameVersion::GamePass => MemoryReader::new_game_pass(&process, address),
                        GameVersion::Unknown => {
                            print_message(
                                "WARNING: Falling back on Steam version; may not be accurate",
                            );
                            MemoryReader::new_steam(&process, address)
                        }
                    };

                    let mut splitter = Splitter::new(&memory, &mut settings);
                    loop {
                        let _ = splitter.run().await;
                    }
                }
            })
            .await;
    }
}

fn get_game_version(module_size: u64) -> GameVersion {
    match module_size {
        // Windows and Linux respectively
        61046784 | 1052672 => {
            print_message("Game Detected: Steam");
            GameVersion::Steam
        }
        // Not tested; taken from ASL autosplitter and assumed to work with Game Pass on Windows
        95162368 => {
            print_message("Game Detected: Game Pass");
            GameVersion::GamePass
        }
        size => {
            print_message(&format!("Game Detected: Unknown Version (module size = `{}`", size));
            GameVersion::Unknown
        }
    }
}
