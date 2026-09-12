mod settings;
mod splitter;

use asr::{settings::Gui, Process};
use settings::Settings;
use splitter::Splitter;

const EXE_NAME: &str = "Spyro-Win64-Shipping.exe";

asr::async_main!(stable);
pub async fn main() {
    let mut settings = Settings::register();
    let splitter = Splitter::default();

    loop {
        let process = Process::wait_attach(EXE_NAME).await;
        process
            .until_closes(async {
                loop {
                    settings.update();
                    splitter.next_tick(&settings).await;
                }
            })
            .await;
    }
}
