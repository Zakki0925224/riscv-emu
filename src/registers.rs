#[derive(Debug, Default)]
pub struct ProgramCounter(u64);

impl ProgramCounter {
    pub fn load(&self) -> u64 {
        self.0
    }

    pub fn store(&mut self, value: u64) {
        self.0 = value;
    }

    pub fn increment(&mut self) {
        self.0 += 4
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GeneralPurposeRegister(u64);

impl GeneralPurposeRegister {
    pub fn load(&self) -> u64 {
        self.0
    }

    pub fn store(&mut self, value: u64) {
        self.0 = value;
    }
}
