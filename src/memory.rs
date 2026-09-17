mod level;

pub use level::Level;

use asr::{Address, Process};
use bytemuck::Pod;
use level::LevelReader;

#[derive(Default)]
pub struct Memory {
    level_reader: LevelReader,
}

impl Memory {
    pub fn update(&mut self, process: &Process, address: Address) {
        self.level_reader.update(process, address);
    }

    pub const fn level_reader(&self) -> &LevelReader {
        &self.level_reader
    }

    fn read<T: Pod>(process: &Process, address: Address, path: &[u64]) -> Option<T> {
        process
            .read_pointer_path(address, asr::PointerSize::Bit64, path)
            .ok()
    }
}
