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

macro_rules! op_inc_rrn {
    ( $self:ident, $dest:ident ) => {{
        let value = register16_load!($self, $dest);
        let r = op_inc!($self, value);
        register16_store!($self, $dest, r)
    }};
}
