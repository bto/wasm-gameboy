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
