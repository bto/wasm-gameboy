use super::mmu::MMU;
use super::registers::Registers;

macro_rules! op_ld_hl_n {
    ( $self:ident ) => {{
        let addr = $self.registers.hl_get();
        let value = $self.fetch_byte();
        $self.mmu.byte_set(addr, value);
    }};
}

macro_rules! op_ld_hl_r {
    ( $self:ident, $src:ident ) => {{
        let addr = $self.registers.hl_get();
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
    }};
}

macro_rules! op_ld_r_hl {
    ( $self:ident, $dest:ident ) => {{
        let addr = $self.registers.hl_get();
        $self.registers.$dest = $self.mmu.byte_get(addr);
    }};
}

macro_rules! op_ld_r_n {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = $self.fetch_byte();
    }};
}

macro_rules! op_ld_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        $self.registers.$dest = $self.registers.$src;
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
            0b00_110_110 => op_ld_hl_n!(self),
            0b00_111_110 => op_ld_r_n!(self, a),

            0b01_000_000 => op_ld_r_r!(self, b, b),
            0b01_000_001 => op_ld_r_r!(self, b, c),
            0b01_000_010 => op_ld_r_r!(self, b, d),
            0b01_000_011 => op_ld_r_r!(self, b, e),
            0b01_000_100 => op_ld_r_r!(self, b, h),
            0b01_000_101 => op_ld_r_r!(self, b, l),
            0b01_000_110 => op_ld_r_hl!(self, b),
            0b01_000_111 => op_ld_r_r!(self, b, a),

            0b01_001_000 => op_ld_r_r!(self, c, b),
            0b01_001_001 => op_ld_r_r!(self, c, c),
            0b01_001_010 => op_ld_r_r!(self, c, d),
            0b01_001_011 => op_ld_r_r!(self, c, e),
            0b01_001_100 => op_ld_r_r!(self, c, h),
            0b01_001_101 => op_ld_r_r!(self, c, l),
            0b01_001_110 => op_ld_r_hl!(self, c),
            0b01_001_111 => op_ld_r_r!(self, c, a),

            0b01_010_000 => op_ld_r_r!(self, d, b),
            0b01_010_001 => op_ld_r_r!(self, d, c),
            0b01_010_010 => op_ld_r_r!(self, d, d),
            0b01_010_011 => op_ld_r_r!(self, d, e),
            0b01_010_100 => op_ld_r_r!(self, d, h),
            0b01_010_101 => op_ld_r_r!(self, d, l),
            0b01_010_110 => op_ld_r_hl!(self, d),
            0b01_010_111 => op_ld_r_r!(self, d, a),

            0b01_011_000 => op_ld_r_r!(self, e, b),
            0b01_011_001 => op_ld_r_r!(self, e, c),
            0b01_011_010 => op_ld_r_r!(self, e, d),
            0b01_011_011 => op_ld_r_r!(self, e, e),
            0b01_011_100 => op_ld_r_r!(self, e, h),
            0b01_011_101 => op_ld_r_r!(self, e, l),
            0b01_011_110 => op_ld_r_hl!(self, e),
            0b01_011_111 => op_ld_r_r!(self, e, a),

            0b01_100_000 => op_ld_r_r!(self, h, b),
            0b01_100_001 => op_ld_r_r!(self, h, c),
            0b01_100_010 => op_ld_r_r!(self, h, d),
            0b01_100_011 => op_ld_r_r!(self, h, e),
            0b01_100_100 => op_ld_r_r!(self, h, h),
            0b01_100_101 => op_ld_r_r!(self, h, l),
            0b01_100_110 => op_ld_r_hl!(self, h),
            0b01_100_111 => op_ld_r_r!(self, h, a),

            0b01_101_000 => op_ld_r_r!(self, l, b),
            0b01_101_001 => op_ld_r_r!(self, l, c),
            0b01_101_010 => op_ld_r_r!(self, l, d),
            0b01_101_011 => op_ld_r_r!(self, l, e),
            0b01_101_100 => op_ld_r_r!(self, l, h),
            0b01_101_101 => op_ld_r_r!(self, l, l),
            0b01_101_110 => op_ld_r_hl!(self, l),
            0b01_101_111 => op_ld_r_r!(self, l, a),

            0b01_110_000 => op_ld_hl_r!(self, b),
            0b01_110_001 => op_ld_hl_r!(self, c),
            0b01_110_010 => op_ld_hl_r!(self, d),
            0b01_110_011 => op_ld_hl_r!(self, e),
            0b01_110_100 => op_ld_hl_r!(self, h),
            0b01_110_101 => op_ld_hl_r!(self, l),
            0b01_110_110 => {}
            0b01_110_111 => op_ld_hl_r!(self, a),

            0b01_111_000 => op_ld_r_r!(self, a, b),
            0b01_111_001 => op_ld_r_r!(self, a, c),
            0b01_111_010 => op_ld_r_r!(self, a, d),
            0b01_111_011 => op_ld_r_r!(self, a, e),
            0b01_111_100 => op_ld_r_r!(self, a, h),
            0b01_111_101 => op_ld_r_r!(self, a, l),
            0b01_111_110 => op_ld_r_hl!(self, a),
            0b01_111_111 => op_ld_r_r!(self, a, a),

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
