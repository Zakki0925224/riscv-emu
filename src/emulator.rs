use crate::{cpu::Cpu, ram::Ram};

#[derive(Debug)]
pub struct Emulator {
    cpu: Cpu,
    ram: Ram,
}

impl Emulator {
    pub fn new(ram_size: usize) -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(ram_size),
        }
    }
}
