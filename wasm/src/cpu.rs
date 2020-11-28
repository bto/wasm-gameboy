use super::mmu::MMU;
use super::registers::Registers;

macro_rules! register16_get {
    ( $self:ident, af ) => {
        register16_get!($self, af_get)
    };

    ( $self:ident, bc ) => {
        register16_get!($self, bc_get)
    };

    ( $self:ident, de ) => {
        register16_get!($self, de_get)
    };

    ( $self:ident, hl ) => {
        register16_get!($self, hl_get)
    };

    ( $self:ident, sp ) => {
        $self.registers.sp
    };

    ( $self:ident, $method:ident ) => {
        $self.registers.$method()
    };
}

macro_rules! register16_set {
    ( $self:ident, af, $value:expr ) => {
        register16_set!($self, af_set, $value)
    };

    ( $self:ident, bc, $value:expr ) => {
        register16_set!($self, bc_set, $value)
    };

    ( $self:ident, de, $value:expr ) => {
        register16_set!($self, de_set, $value)
    };

    ( $self:ident, hl, $value:expr ) => {
        register16_set!($self, hl_set, $value)
    };

    ( $self:ident, sp, $value:expr ) => {
        $self.registers.sp = $value
    };

    ( $self:ident, $method:ident, $value:expr ) => {
        $self.registers.$method($value)
    };
}

macro_rules! register16_load {
    ( $self:ident, $register:ident ) => {{
        let addr = register16_get!($self, $register);
        $self.mmu.byte_get(addr)
    }};
}

macro_rules! register16_store {
    ( $self:ident, $register:ident, $value:expr ) => {{
        let addr = register16_get!($self, $register);
        $self.mmu.byte_set(addr, $value);
    }};
}

macro_rules! op_ld_nn_r {
    ( $self:ident, $src:ident ) => {{
        $self.fetch_store($self.registers.$src)
    }};
}

macro_rules! op_ld_nn_rr {
    ( $self:ident, $src:ident ) => {{
        let addr = $self.fetch_word();
        let value = register16_get!($self, $src);
        $self.mmu.word_set(addr, value)
    }};
}

macro_rules! op_ld_r_n {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = $self.fetch_byte();
    }};
}

macro_rules! op_ld_r_nn {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = $self.fetch_load();
    }};
}

macro_rules! op_ld_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        $self.registers.$dest = $self.registers.$src;
    }};
}

macro_rules! op_ld_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        $self.registers.$dest = register16_load!($self, $src);
    }};
}

macro_rules! op_ld_rr_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        register16_store!($self, $dest, value);
    }};
}

macro_rules! op_ld_rr_nn {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_word();
        register16_set!($self, $dest, value);
    }};
}

macro_rules! op_ld_rr_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        register16_store!($self, $dest, value);
    }};
}

macro_rules! op_ld_rr_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_get!($self, $src);
        register16_set!($self, $dest, value);
    }};
}

macro_rules! op_ldd_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $src);
        $self.registers.$dest = $self.mmu.byte_get(addr);
        register16_set!($self, $src, addr - 1);
    }};
}

macro_rules! op_ldi_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $src);
        $self.registers.$dest = $self.mmu.byte_get(addr);
        register16_set!($self, $src, addr + 1);
    }};
}

macro_rules! op_ldd_rr_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $dest);
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
        register16_set!($self, $dest, addr - 1);
    }};
}

macro_rules! op_ldi_rr_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $dest);
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
        register16_set!($self, $dest, addr + 1);
    }};
}

macro_rules! op_ldh_r_nh {
    ( $self:ident, $dest:ident ) => {{
        let addr = 0xFF00 | $self.fetch_byte() as u16;
        $self.registers.$dest = $self.mmu.byte_get(addr);
    }};
}

macro_rules! op_ldh_r_rh {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = 0xFF00 | $self.registers.$src as u16;
        $self.registers.$dest = $self.mmu.byte_get(addr);
    }};
}

macro_rules! op_ldh_nh_r {
    ( $self:ident, $src:ident ) => {{
        let addr = 0xFF00 | $self.fetch_byte() as u16;
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
    }};
}

macro_rules! op_ldh_rh_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = 0xFF00 | $self.registers.$dest as u16;
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
    }};
}

macro_rules! op_pop_rr {
    ( $self:ident, $src:ident ) => {{
        let value = $self.mmu.word_get($self.registers.sp);
        $self.registers.sp += 2;
        register16_set!($self, $src, value)
    }};
}

macro_rules! op_push_rr {
    ( $self:ident, $src:ident ) => {{
        let value = register16_get!($self, $src);
        $self.registers.sp -= 1;
        $self.mmu.byte_set($self.registers.sp, (value >> 8) as u8);
        $self.registers.sp -= 1;
        $self.mmu.byte_set($self.registers.sp, (value & 0xFF) as u8);
    }};
}

macro_rules! op_add_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value;
        let r = x + y;
        $self.registers.$dest = r as u8;
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = ((x & 0xF) + (y & 0xF)) > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_add_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src as u16;
        op_add_r!($self, $dest, value)
    }};
}

macro_rules! op_add_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src) as u16;
        op_add_r!($self, $dest, value)
    }};
}

macro_rules! op_adc_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value;
        let c = $self.registers.carry as u16;
        let r = x + y + c;
        $self.registers.$dest = r as u8;
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = (x & 0xF) + (y & 0xF) + c > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_adc_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src as u16;
        op_adc_r!($self, $dest, value)
    }};
}

macro_rules! op_adc_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src) as u16;
        op_adc_r!($self, $dest, value)
    }};
}

macro_rules! op_sub_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value;
        let r = x.wrapping_sub(y);
        $self.registers.$dest = r as u8;
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = (x & 0xF) < (y & 0xF);
        $self.registers.subtraction = true;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_sub_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src as u16;
        op_sub_r!($self, $dest, value)
    }};
}

macro_rules! op_sub_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src) as u16;
        op_sub_r!($self, $dest, value)
    }};
}

macro_rules! op_sbc_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value;
        let c = $self.registers.carry as u16;
        let r = x.wrapping_sub(y).wrapping_sub(c);
        $self.registers.$dest = r as u8;
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = (x & 0xF) < (y & 0xF) + c;
        $self.registers.subtraction = true;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_sbc_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src as u16;
        op_sbc_r!($self, $dest, value)
    }};
}

macro_rules! op_sbc_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src) as u16;
        op_sbc_r!($self, $dest, value)
    }};
}

macro_rules! op_and_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        $self.registers.$dest = $self.registers.$dest & $value;
        $self.registers.carry = false;
        $self.registers.half_carry = true;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_and_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_and_r!($self, $dest, value)
    }};
}

macro_rules! op_and_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_and_r!($self, $dest, value)
    }};
}

macro_rules! op_xor_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        $self.registers.$dest = $self.registers.$dest ^ $value;
        $self.registers.carry = false;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_xor_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_xor_r!($self, $dest, value)
    }};
}

macro_rules! op_xor_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_xor_r!($self, $dest, value)
    }};
}

macro_rules! op_or_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        $self.registers.$dest = $self.registers.$dest | $value;
        $self.registers.carry = false;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_or_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_or_r!($self, $dest, value)
    }};
}

macro_rules! op_or_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_or_r!($self, $dest, value)
    }};
}

macro_rules! op_cp_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value;
        let r = x.wrapping_sub(y);
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = (x & 0xF) < (y & 0xF);
        $self.registers.subtraction = true;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_cp_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src as u16;
        op_cp_r!($self, $dest, value)
    }};
}

macro_rules! op_cp_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src) as u16;
        op_cp_r!($self, $dest, value)
    }};
}

macro_rules! op_inc {
    ( $self:ident, $value:expr ) => {{
        let r = $value.wrapping_add(1);
        $self.registers.half_carry = (($value & 0xF) + 1) > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = r == 0;
        r
    }};
}

macro_rules! op_inc_r {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = op_inc!($self, $self.registers.$dest)
    }};
}

macro_rules! op_inc_rr {
    ( $self:ident, $dest:ident ) => {{
        let value = register16_load!($self, $dest);
        let r = op_inc!($self, value);
        register16_store!($self, $dest, r)
    }};
}

macro_rules! op_dec {
    ( $self:ident, $value:expr ) => {{
        let r = $value.wrapping_sub(1);
        $self.registers.half_carry = ($value & 0xF) == 0;
        $self.registers.subtraction = true;
        $self.registers.zero = r == 0;
        r
    }};
}

macro_rules! op_dec_r {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = op_dec!($self, $self.registers.$dest)
    }};
}

macro_rules! op_dec_rr {
    ( $self:ident, $dest:ident ) => {{
        let value = register16_load!($self, $dest);
        let r = op_dec!($self, value);
        register16_store!($self, $dest, r)
    }};
}

macro_rules! op_add_rr {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = register16_get!($self, $dest);
        let y = $value;
        let (r, c) = x.overflowing_add(y);
        register16_set!($self, $dest, r);
        $self.registers.carry = c;
        $self.registers.half_carry = ((x & 0x7FF) + (y & 0x7FF)) > 0x7FF;
        $self.registers.subtraction = false;
        $self.registers.zero = r == 0;
    }};
}

macro_rules! op_add_rr_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_get!($self, $src);
        op_add_rr!($self, $dest, value)
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
            0b00_110_110 => op_ld_rr_n!(self, hl),
            0b00_111_110 => op_ld_r_n!(self, a),

            0b01_000_000 => {}
            0b01_000_001 => op_ld_r_r!(self, b, c),
            0b01_000_010 => op_ld_r_r!(self, b, d),
            0b01_000_011 => op_ld_r_r!(self, b, e),
            0b01_000_100 => op_ld_r_r!(self, b, h),
            0b01_000_101 => op_ld_r_r!(self, b, l),
            0b01_000_110 => op_ld_r_rr!(self, b, hl),
            0b01_000_111 => op_ld_r_r!(self, b, a),

            0b01_001_000 => op_ld_r_r!(self, c, b),
            0b01_001_001 => {}
            0b01_001_010 => op_ld_r_r!(self, c, d),
            0b01_001_011 => op_ld_r_r!(self, c, e),
            0b01_001_100 => op_ld_r_r!(self, c, h),
            0b01_001_101 => op_ld_r_r!(self, c, l),
            0b01_001_110 => op_ld_r_rr!(self, c, hl),
            0b01_001_111 => op_ld_r_r!(self, c, a),

            0b01_010_000 => op_ld_r_r!(self, d, b),
            0b01_010_001 => op_ld_r_r!(self, d, c),
            0b01_010_010 => {}
            0b01_010_011 => op_ld_r_r!(self, d, e),
            0b01_010_100 => op_ld_r_r!(self, d, h),
            0b01_010_101 => op_ld_r_r!(self, d, l),
            0b01_010_110 => op_ld_r_rr!(self, d, hl),
            0b01_010_111 => op_ld_r_r!(self, d, a),

            0b01_011_000 => op_ld_r_r!(self, e, b),
            0b01_011_001 => op_ld_r_r!(self, e, c),
            0b01_011_010 => op_ld_r_r!(self, e, d),
            0b01_011_011 => {}
            0b01_011_100 => op_ld_r_r!(self, e, h),
            0b01_011_101 => op_ld_r_r!(self, e, l),
            0b01_011_110 => op_ld_r_rr!(self, e, hl),
            0b01_011_111 => op_ld_r_r!(self, e, a),

            0b01_100_000 => op_ld_r_r!(self, h, b),
            0b01_100_001 => op_ld_r_r!(self, h, c),
            0b01_100_010 => op_ld_r_r!(self, h, d),
            0b01_100_011 => op_ld_r_r!(self, h, e),
            0b01_100_100 => {}
            0b01_100_101 => op_ld_r_r!(self, h, l),
            0b01_100_110 => op_ld_r_rr!(self, h, hl),
            0b01_100_111 => op_ld_r_r!(self, h, a),

            0b01_101_000 => op_ld_r_r!(self, l, b),
            0b01_101_001 => op_ld_r_r!(self, l, c),
            0b01_101_010 => op_ld_r_r!(self, l, d),
            0b01_101_011 => op_ld_r_r!(self, l, e),
            0b01_101_100 => op_ld_r_r!(self, l, h),
            0b01_101_101 => {}
            0b01_101_110 => op_ld_r_rr!(self, l, hl),
            0b01_101_111 => op_ld_r_r!(self, l, a),

            0b01_110_000 => op_ld_rr_r!(self, hl, b),
            0b01_110_001 => op_ld_rr_r!(self, hl, c),
            0b01_110_010 => op_ld_rr_r!(self, hl, d),
            0b01_110_011 => op_ld_rr_r!(self, hl, e),
            0b01_110_100 => op_ld_rr_r!(self, hl, h),
            0b01_110_101 => op_ld_rr_r!(self, hl, l),
            0b01_110_110 => {}
            0b01_110_111 => op_ld_rr_r!(self, hl, a),

            0b01_111_000 => op_ld_r_r!(self, a, b),
            0b01_111_001 => op_ld_r_r!(self, a, c),
            0b01_111_010 => op_ld_r_r!(self, a, d),
            0b01_111_011 => op_ld_r_r!(self, a, e),
            0b01_111_100 => op_ld_r_r!(self, a, h),
            0b01_111_101 => op_ld_r_r!(self, a, l),
            0b01_111_110 => op_ld_r_rr!(self, a, hl),
            0b01_111_111 => {}

            0b00_00_1010 => op_ld_r_rr!(self, a, bc),
            0b00_01_1010 => op_ld_r_rr!(self, a, de),
            0b00_10_1010 => op_ldi_r_rr!(self, a, hl),
            0b00_11_1010 => op_ldd_r_rr!(self, a, hl),

            0b00_00_0010 => op_ld_rr_r!(self, bc, a),
            0b00_01_0010 => op_ld_rr_r!(self, de, a),
            0b00_10_0010 => op_ldi_rr_r!(self, hl, a),
            0b00_11_0010 => op_ldd_rr_r!(self, hl, a),

            0b11101010 => op_ld_nn_r!(self, a),
            0b11111010 => op_ld_r_nn!(self, a),

            0b11100010 => op_ldh_rh_r!(self, c, a),
            0b11110010 => op_ldh_r_rh!(self, a, c),

            0b11100000 => op_ldh_nh_r!(self, a),
            0b11110000 => op_ldh_r_nh!(self, a),

            0b00_00_0001 => op_ld_rr_nn!(self, bc),
            0b00_01_0001 => op_ld_rr_nn!(self, de),
            0b00_10_0001 => op_ld_rr_nn!(self, hl),
            0b00_11_0001 => op_ld_rr_nn!(self, sp),

            0b00001000 => op_ld_nn_rr!(self, sp),

            0b11111001 => op_ld_rr_rr!(self, sp, hl),

            0b11_00_0101 => op_push_rr!(self, bc),
            0b11_01_0101 => op_push_rr!(self, de),
            0b11_10_0101 => op_push_rr!(self, hl),
            0b11_11_0101 => op_push_rr!(self, af),

            0b11_00_0001 => op_pop_rr!(self, bc),
            0b11_01_0001 => op_pop_rr!(self, de),
            0b11_10_0001 => op_pop_rr!(self, hl),
            0b11_11_0001 => op_pop_rr!(self, af),

            0b10000_000 => op_add_r_r!(self, a, b),
            0b10000_001 => op_add_r_r!(self, a, c),
            0b10000_010 => op_add_r_r!(self, a, d),
            0b10000_011 => op_add_r_r!(self, a, e),
            0b10000_100 => op_add_r_r!(self, a, h),
            0b10000_101 => op_add_r_r!(self, a, l),
            0b10000_110 => op_add_r_rr!(self, a, hl),
            0b10000_111 => op_add_r_r!(self, a, a),

            0b10001_000 => op_adc_r_r!(self, a, b),
            0b10001_001 => op_adc_r_r!(self, a, c),
            0b10001_010 => op_adc_r_r!(self, a, d),
            0b10001_011 => op_adc_r_r!(self, a, e),
            0b10001_100 => op_adc_r_r!(self, a, h),
            0b10001_101 => op_adc_r_r!(self, a, l),
            0b10001_110 => op_adc_r_rr!(self, a, hl),
            0b10001_111 => op_adc_r_r!(self, a, a),

            0b10010_000 => op_sub_r_r!(self, a, b),
            0b10010_001 => op_sub_r_r!(self, a, c),
            0b10010_010 => op_sub_r_r!(self, a, d),
            0b10010_011 => op_sub_r_r!(self, a, e),
            0b10010_100 => op_sub_r_r!(self, a, h),
            0b10010_101 => op_sub_r_r!(self, a, l),
            0b10010_110 => op_sub_r_rr!(self, a, hl),
            0b10010_111 => op_sub_r_r!(self, a, a),

            0b10011_000 => op_sbc_r_r!(self, a, b),
            0b10011_001 => op_sbc_r_r!(self, a, c),
            0b10011_010 => op_sbc_r_r!(self, a, d),
            0b10011_011 => op_sbc_r_r!(self, a, e),
            0b10011_100 => op_sbc_r_r!(self, a, h),
            0b10011_101 => op_sbc_r_r!(self, a, l),
            0b10011_110 => op_sbc_r_rr!(self, a, hl),
            0b10011_111 => op_sbc_r_r!(self, a, a),

            0b10100_000 => op_and_r_r!(self, a, b),
            0b10100_001 => op_and_r_r!(self, a, c),
            0b10100_010 => op_and_r_r!(self, a, d),
            0b10100_011 => op_and_r_r!(self, a, e),
            0b10100_100 => op_and_r_r!(self, a, h),
            0b10100_101 => op_and_r_r!(self, a, l),
            0b10100_110 => op_and_r_rr!(self, a, hl),
            0b10100_111 => op_and_r_r!(self, a, a),

            0b10101_000 => op_xor_r_r!(self, a, b),
            0b10101_001 => op_xor_r_r!(self, a, c),
            0b10101_010 => op_xor_r_r!(self, a, d),
            0b10101_011 => op_xor_r_r!(self, a, e),
            0b10101_100 => op_xor_r_r!(self, a, h),
            0b10101_101 => op_xor_r_r!(self, a, l),
            0b10101_110 => op_xor_r_rr!(self, a, hl),
            0b10101_111 => op_xor_r_r!(self, a, a),

            0b10110_000 => op_or_r_r!(self, a, b),
            0b10110_001 => op_or_r_r!(self, a, c),
            0b10110_010 => op_or_r_r!(self, a, d),
            0b10110_011 => op_or_r_r!(self, a, e),
            0b10110_100 => op_or_r_r!(self, a, h),
            0b10110_101 => op_or_r_r!(self, a, l),
            0b10110_110 => op_or_r_rr!(self, a, hl),
            0b10110_111 => op_or_r_r!(self, a, a),

            0b10111_000 => op_cp_r_r!(self, a, b),
            0b10111_001 => op_cp_r_r!(self, a, c),
            0b10111_010 => op_cp_r_r!(self, a, d),
            0b10111_011 => op_cp_r_r!(self, a, e),
            0b10111_100 => op_cp_r_r!(self, a, h),
            0b10111_101 => op_cp_r_r!(self, a, l),
            0b10111_110 => op_cp_r_rr!(self, a, hl),
            0b10111_111 => op_cp_r_r!(self, a, a),

            0b00_000_100 => op_inc_r!(self, b),
            0b00_001_100 => op_inc_r!(self, c),
            0b00_010_100 => op_inc_r!(self, d),
            0b00_011_100 => op_inc_r!(self, e),
            0b00_100_100 => op_inc_r!(self, h),
            0b00_101_100 => op_inc_r!(self, l),
            0b00_110_100 => op_inc_rr!(self, hl),
            0b00_111_100 => op_inc_r!(self, a),

            0b00_000_101 => op_dec_r!(self, b),
            0b00_001_101 => op_dec_r!(self, c),
            0b00_010_101 => op_dec_r!(self, d),
            0b00_011_101 => op_dec_r!(self, e),
            0b00_100_101 => op_dec_r!(self, h),
            0b00_101_101 => op_dec_r!(self, l),
            0b00_110_101 => op_dec_rr!(self, hl),
            0b00_111_101 => op_dec_r!(self, a),

            0b00_00_1001 => op_add_rr_rr!(self, hl, bc),
            0b00_01_1001 => op_add_rr_rr!(self, hl, de),
            0b00_10_1001 => op_add_rr_rr!(self, hl, hl),
            0b00_11_1001 => op_add_rr_rr!(self, hl, sp),

            _ => panic!("not implemented instruction"),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.byte_get(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_load(&mut self) -> u8 {
        let value = self.fetch_word();
        self.mmu.byte_get(value)
    }

    fn fetch_store(&mut self, value: u8) {
        let addr = self.fetch_word();
        self.mmu.byte_set(addr, value)
    }

    fn fetch_word(&mut self) -> u16 {
        let lsb = self.mmu.byte_get(self.registers.pc);
        self.registers.pc += 1;
        let msb = self.mmu.byte_get(self.registers.pc);
        self.registers.pc += 1;
        (msb as u16) << 8 | lsb as u16
    }
}

#[path = "./cpu_test.rs"]
mod tests;
