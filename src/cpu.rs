type RegisterType = u32;

#[derive(Debug)]
pub struct Cpu {
    // general purpose registers
    x_regs: [RegisterType; 32],
    // program counter
    pc: RegisterType,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            x_regs: [0; 32],
            pc: 0,
        }
    }
}
