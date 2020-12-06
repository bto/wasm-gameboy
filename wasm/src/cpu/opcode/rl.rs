macro_rules! op_rl {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x80 == 0x80;
        let r = x << 1 | $self.registers.carry as u8;
        set_rotate_shift_flags!($self, r, c);
        r
    }};
}

macro_rules! op_rl_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let r = op_rl!($self, x);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_rl_rrn {
    ( $self:ident, $dest:ident ) => {{
        let x = register16_load!($self, $dest);
        let r = op_rl!($self, x);
        register16_store!($self, $dest, r);
    }};
}

macro_rules! op_rlc {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x80 == 0x80;
        let r = x << 1 | c as u8;
        set_rotate_shift_flags!($self, r, c);
        r
    }};
}

macro_rules! op_rlc_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let r = op_rlc!($self, x);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_rlc_rrn {
    ( $self:ident, $dest:ident ) => {{
        let x = register16_load!($self, $dest);
        let r = op_rlc!($self, x);
        register16_store!($self, $dest, r);
    }};
}
