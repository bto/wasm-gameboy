macro_rules! op_swap {
    ( $self:ident, $value:expr ) => {{
        let r = ($value >> 4) | ($value << 4);
        $self.registers.carry = false;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = r == 0;
        r
    }};
}

macro_rules! op_swap_r {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = op_swap!($self, $self.registers.$dest)
    }};
}

macro_rules! op_swap_rrn {
    ( $self:ident, $dest:ident ) => {{
        let value = register16_load!($self, $dest);
        let r = op_swap!($self, value);
        register16_store!($self, $dest, r)
    }};
}
