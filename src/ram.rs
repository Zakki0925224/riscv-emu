#[derive(Debug)]
pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    pub fn new(size: u64) -> Self {
        Self {
            ram: vec![0; size as usize],
        }
    }

    pub fn read8(&self, addr: u64) -> u8 {
        let abs_addr = self.abs_addr(addr);
        self.ram[abs_addr]
    }

    pub fn write8(&mut self, addr: u64, data: u8) {
        let abs_addr = self.abs_addr(addr);
        self.ram[abs_addr] = data;
    }

    pub fn size(&self) -> usize {
        self.ram.len()
    }

    fn abs_addr(&self, addr: u64) -> usize {
        addr as usize % self.size()
    }
}
