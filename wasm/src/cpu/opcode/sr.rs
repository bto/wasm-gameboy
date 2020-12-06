macro_rules! op_sra {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x01 == 0x01;
        let r = x >> 1 | x & 0x80;
        set_rotate_shift_flags!($self, r, c);
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

macro_rules! op_srl {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x01 == 0x01;
        let r = x >> 1;
        set_rotate_shift_flags!($self, r, c);
        r
    }};
}

macro_rules! op_srl_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let r = op_srl!($self, x);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_srl_rrn {
    ( $self:ident, $dest:ident ) => {{
        let x = register16_load!($self, $dest);
        let r = op_srl!($self, x);
        register16_store!($self, $dest, r);
    }};
}
