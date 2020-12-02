macro_rules! op_cpl {
    ( $self:ident, $value:expr ) => {{
        let r = !$value;
        $self.registers.half_carry = true;
        $self.registers.subtraction = true;
        r
    }};
}

macro_rules! op_cpl_r {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = op_cpl!($self, $self.registers.$dest)
    }};
}
