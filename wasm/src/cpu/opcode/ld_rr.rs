macro_rules! op_ld_rr_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        register16_store!($self, $dest, value);
    }};
}

macro_rules! op_ld_rr_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        register16_store!($self, $dest, value);
    }};
}

macro_rules! op_ld_rr_nn {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_word();
        register16_set!($self, $dest, value);
    }};
}

macro_rules! op_ld_rr_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_get!($self, $src);
        register16_set!($self, $dest, value);
    }};
}
