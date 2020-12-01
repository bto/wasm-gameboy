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

macro_rules! op_sub_r_rrn {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_sub_r!($self, $dest, value)
    }};
}
