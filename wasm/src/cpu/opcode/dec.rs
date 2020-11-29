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
