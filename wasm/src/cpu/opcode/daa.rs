macro_rules! op_daa {
    ( $self:ident, $value:expr ) => {{
        let v = $value;
        let mut adjust = 0;

        if $self.registers.carry {
            adjust |= 0x60;
        };
        if $self.registers.half_carry {
            adjust |= 0x06;
        };

        let r = if $self.registers.subtraction {
            v.wrapping_sub(adjust)
        } else {
            if v > 0x99 {
                adjust |= 0x60;
            }
            if v & 0x0F > 0x09 {
                adjust |= 0x06;
            }
            v.wrapping_add(adjust)
        };

        $self.registers.carry = adjust >= 0x60;
        $self.registers.half_carry = false;
        $self.registers.zero = r == 0;
        r
    }};
}
macro_rules! op_daa_r {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = op_daa!($self, $self.registers.$dest)
    }};
}
