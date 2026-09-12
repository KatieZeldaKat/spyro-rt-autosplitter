use crate::Settings;
use asr::future::{self as asr_future, NextTick};

#[derive(Default)]
pub struct Splitter {}

impl Splitter {
    pub fn next_tick(&self, _settings: &Settings) -> NextTick {
        asr_future::next_tick()
    }
}
