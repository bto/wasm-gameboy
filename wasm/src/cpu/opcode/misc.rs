macro_rules! op_ccf {
    ( $self:ident ) => {{
        $self.registers.carry = !$self.registers.carry;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
    }};
}

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
