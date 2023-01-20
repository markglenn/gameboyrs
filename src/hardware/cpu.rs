use super::{mmu::MMU, registers::Registers};

pub struct LR35902 {
    pub regs: Registers,
    pub mmu: MMU,
}

impl LR35902 {
    pub fn new() -> LR35902 {
        LR35902 {
            regs: Registers::new(),
            mmu: MMU::new(),
        }
    }
}

#[cfg(test)]
mod test {}
