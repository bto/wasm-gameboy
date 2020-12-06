macro_rules! op_bit {
    ( $self:ident, $x:expr, $y:expr ) => {{
        let x = $x;
        let y = $y;
        $self.registers.half_carry = true;
        $self.registers.subtraction = false;
        $self.registers.zero = (x & (1 << y)) == 0;
    }};
}

macro_rules! op_bit_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest;
        op_bit!($self, x, $value);
    }};
}

macro_rules! op_bit_rrn {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = register16_load!($self, $dest);
        op_bit!($self, x, $value);
    }};
}
