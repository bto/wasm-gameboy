macro_rules! op_add_rr {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = register16_get!($self, $dest);
        let y = $value;
        let (r, c) = x.overflowing_add(y);
        register16_set!($self, $dest, r);
        $self.registers.carry = c;
        $self.registers.half_carry = ((x & 0x7FF) + (y & 0x7FF)) > 0x7FF;
        $self.registers.subtraction = false;
    }};
}

macro_rules! op_add_rr_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_get!($self, $src);
        op_add_rr!($self, $dest, value)
    }};
}

macro_rules! op_add_rr_rr_n {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let x = register16_get!($self, $src);
        let y = $self.fetch_byte() as u16;
        register16_set!($self, $dest, x.wrapping_add(y));
        $self.registers.carry = ((x & 0xFF) + (y & 0xFF)) > 0xFF;
        $self.registers.half_carry = ((x & 0xF) + (y & 0xF)) > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = false;
    }};
}
