use crate::{cpu::Cpu, ram::Ram};

#[derive(Debug)]
pub struct Emulator {
    cpu: Cpu,
    ram: Ram,
}

impl Emulator {
    pub fn new(ram_size: u64) -> Self {
        Self {
            cpu: Cpu::default(),
            ram: Ram::new(ram_size),
        }
    }

    pub fn run(&mut self) {
        self.reset();
        println!("cpu: {:?}", self.cpu);
        let pc = self.cpu.pc.load();
        let ram_data = self.ram.read8(pc);
        println!("0x{:x} at 0x{:x}", ram_data, pc);
    }

    fn reset(&mut self) {
        let ram_size = self.ram.size();
        self.cpu = Cpu::default();
        self.ram = Ram::new(ram_size as u64);
    }
}
