#[derive(Debug)]
pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self { ram: vec![0; size] }
    }

    pub fn read8(&self, addr: usize) -> u8 {
        let abs_addr = self.abs_addr(addr);
        self.ram[abs_addr]
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        let abs_addr = self.abs_addr(addr);
        self.ram[abs_addr] = data;
    }

    pub fn size(&self) -> usize {
        self.ram.len()
    }

    fn abs_addr(&self, addr: usize) -> usize {
        addr % self.size()
    }
}
