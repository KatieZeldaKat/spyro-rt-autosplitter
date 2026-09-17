mod memory;
mod settings;
mod splitter;

use asr::{future as asr_future, settings::Gui, timer, Process};
use memory::{Level, Memory};
use settings::Settings;
use splitter::Splitter;

const EXE_NAME: &str = "Spyro-Win64-Shipping.exe";

asr::async_main!(stable);
pub async fn main() {
    let mut settings = Settings::register();

    loop {
        let process = Process::wait_attach(EXE_NAME).await;
        if let Ok(address) = process.get_module_address(EXE_NAME) {
            timer::start();
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
