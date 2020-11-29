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
