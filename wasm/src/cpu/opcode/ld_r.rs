macro_rules! op_ld_r_n {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = $self.fetch_byte();
    }};
}

macro_rules! op_ld_r_nn {
    ( $self:ident, $dest:ident ) => {{
        $self.registers.$dest = $self.fetch_load();
    }};
}

macro_rules! op_ld_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        $self.registers.$dest = $self.registers.$src;
    }};
}

macro_rules! op_ld_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        $self.registers.$dest = register16_load!($self, $src);
    }};
}
