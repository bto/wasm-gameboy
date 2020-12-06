macro_rules! set_shift_flags {
    ( $self:ident, $result:expr, $carry:expr ) => {{
        $self.registers.carry = $carry;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = $result == 0;
    }};
}

macro_rules! op_sla {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x80 == 0x80;
        let r = x << 1;
        set_shift_flags!($self, r, c);
        r
    }};
}

macro_rules! op_sla_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let r = op_sla!($self, x);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_sla_rrn {
    ( $self:ident, $dest:ident ) => {{
        let x = register16_load!($self, $dest);
        let r = op_sla!($self, x);
        register16_store!($self, $dest, r);
    }};
}

macro_rules! op_sra {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x01 == 0x01;
        let r = x >> 1 | x & 0x80;
        set_shift_flags!($self, r, c);
        r
    }};
}

macro_rules! op_sra_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let r = op_sra!($self, x);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_sra_rrn {
    ( $self:ident, $dest:ident ) => {{
        let x = register16_load!($self, $dest);
        let r = op_sra!($self, x);
        register16_store!($self, $dest, r);
    }};
}
