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
            0b00_000_110 => op_ld_r_n!(self, b),
            0b00_001_110 => op_ld_r_n!(self, c),
            0b00_010_110 => op_ld_r_n!(self, d),
            0b00_011_110 => op_ld_r_n!(self, e),
            0b00_100_110 => op_ld_r_n!(self, h),
            0b00_101_110 => op_ld_r_n!(self, l),
            0b00_111_110 => op_ld_r_n!(self, a),
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
