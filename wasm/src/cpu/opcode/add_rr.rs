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
