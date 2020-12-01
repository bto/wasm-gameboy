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

macro_rules! op_and_r_rrn {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_and_r!($self, $dest, value)
    }};
}
