macro_rules! op_rr {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x01 == 0x01;
        let r = x >> 1 | ($self.registers.carry as u8) << 7;
        set_rotate_shift_flags!($self, r, c);
        r
    }};
}

macro_rules! op_rr_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let r = op_rr!($self, x);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_rr_rrn {
    ( $self:ident, $dest:ident ) => {{
        let x = register16_load!($self, $dest);
        let r = op_rr!($self, x);
        register16_store!($self, $dest, r);
    }};
}

macro_rules! op_rrc {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x01 == 0x01;
        let r = x >> 1 | (c as u8) << 7;
        set_rotate_shift_flags!($self, r, c);
        r
    }};
}

macro_rules! op_rrc_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let r = op_rrc!($self, x);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_rrc_rrn {
    ( $self:ident, $dest:ident ) => {{
        let x = register16_load!($self, $dest);
        let r = op_rrc!($self, x);
        register16_store!($self, $dest, r);
    }};
}
