macro_rules! op_sla {
    ( $self:ident, $value:expr ) => {{
        let x = $value;
        let c = x & 0x80 == 0x80;
        let r = x << 1;
        set_rotate_shift_flags!($self, r, c);
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
