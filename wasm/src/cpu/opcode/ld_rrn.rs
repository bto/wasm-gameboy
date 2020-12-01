macro_rules! op_ld_rrn_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        register16_store!($self, $dest, value);
    }};
}

macro_rules! op_ld_rrn_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        register16_store!($self, $dest, value);
    }};
}
