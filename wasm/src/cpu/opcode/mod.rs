#[macro_use]
mod register16;

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
        let x = $self.registers.$dest;
        let y = $value;
        let (r, c) = x.overflowing_add(y);
        $self.registers.$dest = r;
        $self.registers.carry = c;
        $self.registers.half_carry = ((x & 0xF) + (y & 0xF)) > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = r == 0;
    }};
}

macro_rules! op_add_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_add_r!($self, $dest, value)
    }};
}

macro_rules! op_add_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_add_r!($self, $dest, value)
    }};
}

macro_rules! op_add_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_add_r!($self, $dest, value)
    }};
}

macro_rules! op_adc_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value as u16;
        let c = $self.registers.carry as u16;
        let r = x + y + c;
        $self.registers.$dest = r as u8;
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = (x & 0xF) + (y & 0xF) + c > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = (r as u8) == 0;
    }};
}

macro_rules! op_adc_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_adc_r!($self, $dest, value)
    }};
}

macro_rules! op_adc_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_adc_r!($self, $dest, value)
    }};
}

macro_rules! op_adc_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_adc_r!($self, $dest, value)
    }};
}

macro_rules! op_sub_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest;
        let y = $value;
        let (r, c) = x.overflowing_sub(y);
        $self.registers.$dest = r;
        $self.registers.carry = c;
        $self.registers.half_carry = (x & 0xF) < (y & 0xF);
        $self.registers.subtraction = true;
        $self.registers.zero = r == 0;
    }};
}

macro_rules! op_sub_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_sub_r!($self, $dest, value)
    }};
}

macro_rules! op_sub_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_sub_r!($self, $dest, value)
    }};
}

macro_rules! op_sub_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_sub_r!($self, $dest, value)
    }};
}

macro_rules! op_sbc_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value as u16;
        let c = $self.registers.carry as u16;
        let r = x.wrapping_sub(y).wrapping_sub(c);
        $self.registers.$dest = r as u8;
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = (x & 0xF) < (y & 0xF) + c;
        $self.registers.subtraction = true;
        $self.registers.zero = (r as u8) == 0;
    }};
}

macro_rules! op_sbc_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_sbc_r!($self, $dest, value)
    }};
}

macro_rules! op_sbc_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_sbc_r!($self, $dest, value)
    }};
}

macro_rules! op_sbc_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_sbc_r!($self, $dest, value)
    }};
}

macro_rules! op_and_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        $self.registers.$dest &= $value;
        $self.registers.carry = false;
        $self.registers.half_carry = true;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_and_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_and_r!($self, $dest, value)
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
        $self.registers.$dest ^= $value;
        $self.registers.carry = false;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_xor_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_xor_r!($self, $dest, value)
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
        $self.registers.$dest |= $value;
        $self.registers.carry = false;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = $self.registers.$dest == 0;
    }};
}

macro_rules! op_or_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_or_r!($self, $dest, value)
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
        let x = $self.registers.$dest;
        let y = $value;
        let (r, c) = x.overflowing_sub(y);
        $self.registers.carry = c;
        $self.registers.half_carry = (x & 0xF) < (y & 0xF);
        $self.registers.subtraction = true;
        $self.registers.zero = r == 0;
    }};
}

macro_rules! op_cp_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_cp_r!($self, $dest, value)
    }};
}

macro_rules! op_cp_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_cp_r!($self, $dest, value)
    }};
}

macro_rules! op_cp_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
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
