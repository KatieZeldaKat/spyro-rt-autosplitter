mod splitter;

use asr::{Process, print_message};
use splitter::Splitter;

asr::async_main!(stable);

const EXE: &str = "Spyro-Win64-Shipping.exe";
const TICK_RATE: u8 = 30;

async fn main() {
    asr::set_tick_rate(f64::from(TICK_RATE));

    loop {
        let process = Process::wait_attach(EXE).await;
        process
            .until_closes(async {
                if let Ok((address, module_size)) = process.get_module_range(EXE) {
                    detect_game_version(module_size);

                    let mut splitter = Splitter::new(&process, address);
                    loop {
                        splitter.run().await;
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
