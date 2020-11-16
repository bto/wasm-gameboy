use super::mmu::MMU;
use super::registers::Registers;

macro_rules! op_ld_r_n {
    ( $self:ident, $register:ident ) => {{
        $self.registers.$register = $self.fetch_byte();
    }};
}

struct CPU {
    mmu: MMU,
    registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            mmu: MMU::new(),
            registers: Registers::new(),
        }
    }

    fn execute(&mut self) {
        match self.fetch_byte() {
            0x06 => op_ld_r_n!(self, b),
            0x0E => op_ld_r_n!(self, c),
            0x16 => op_ld_r_n!(self, d),
            0x1E => op_ld_r_n!(self, e),
            0x26 => op_ld_r_n!(self, h),
            0x2E => op_ld_r_n!(self, l),
            _ => panic!("not implemented instruction"),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.byte_get(self.registers.pc);
        self.registers.pc += 1;
        byte
    }
}

#[path = "./cpu_test.rs"]
mod tests;
