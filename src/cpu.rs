use crate::registers::{GeneralPurposeRegister, ProgramCounter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuState {
    Reset,
    Fetch,
    Decode,
    Execution,
    WriteBack,
}

#[derive(Debug)]
pub struct Cpu {
    // general purpose registers
    pub x_regs: [GeneralPurposeRegister; 32],
    // program counter
    pub pc: ProgramCounter,
    pub state: CpuState,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x_regs: [GeneralPurposeRegister::default(); 32],
            pc: ProgramCounter::default(),
            state: CpuState::Reset,
        }
    }
}

impl Cpu {
    pub fn decode(&self) {}
}
