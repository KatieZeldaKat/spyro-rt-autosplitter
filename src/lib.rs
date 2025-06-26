use asr::{Process, print_message, settings::Gui};
use splitter::{Splitter, memory::MemoryReader, settings::Settings};

asr::async_main!(stable);

const EXE: &str = "Spyro-Win64-Shipping.exe";
const TICK_RATE: u8 = 30;

/// The entry point to the auto-splitter. More info can be found in [`Splitter`].
pub async fn main() {
    asr::set_tick_rate(f64::from(TICK_RATE));
    let mut settings = Settings::register();

    loop {
        let process = Process::wait_attach(EXE).await;
        process
            .until_closes(async {
                if let Ok((address, module_size)) = process.get_module_range(EXE) {
                    detect_game_version(module_size);

                    let memory = MemoryReader::new(&process, address);
                    let mut splitter = Splitter::new(&memory, &mut settings);
                    loop {
                        let _ = splitter.run().await;
                    }
                }
            })
            .await;
    }
}

fn detect_game_version(module_size: u64) {
    match module_size {
        61046784 => {
            print_message("Spyro Reignited Trilogy WASM started (game version detected: Windows)");
        }
        1052672 => {
            print_message("Spyro Reignited Trilogy WASM started (game version detected: Linux)");
        }
        size => {
            print_message("Spyro Reignited Trilogy WASM started (unknown game version)");
            print_message(&format!("Module size found: {}", &size.to_string()));
        }
    }
}
