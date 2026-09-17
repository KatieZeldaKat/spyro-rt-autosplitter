pub mod memory;
pub mod settings;
mod splitter;

use memory::Memory;
use settings::Settings;
pub use splitter::Splitter;

use asr::{Process, future as asr_future, settings::Gui, timer};

const EXE_NAME: &str = "Spyro-Win64-Shipping.exe";

asr::async_main!(stable);

/// The entry point to the auto-splitter.
pub async fn main() {
    let mut settings = Settings::register();

    loop {
        let process = Process::wait_attach(EXE_NAME).await;
        if let Ok(address) = process.get_module_address(EXE_NAME) {
            let mut memory = Memory::default();
            let mut splitter = Splitter::default();

            process
                .until_closes(async {
                    loop {
                        settings.update();
                        memory.update(&process, address);
                        splitter.update(&memory, &settings);

                        asr_future::next_tick().await;
                    }
                })
                .await;

            timer::reset();
        }
    }
}
