macro_rules! op_daa {
    ( $self:ident ) => {{
        let a = $self.registers.a;
        let mut adjust = 0;

        if $self.registers.carry {
            adjust |= 0x60;
        };
        if $self.registers.half_carry {
            adjust |= 0x06;
        };

        let r = if $self.registers.subtraction {
            a.wrapping_sub(adjust)
        } else {
            if a > 0x99 {
                adjust |= 0x60;
            }
            if a & 0x0F > 0x09 {
                adjust |= 0x06;
            }
            a.wrapping_add(adjust)
        };

        $self.registers.a = r;
        $self.registers.carry = adjust >= 0x60;
        $self.registers.half_carry = false;
        $self.registers.zero = r == 0;
    }};
}
