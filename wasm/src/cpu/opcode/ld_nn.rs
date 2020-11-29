macro_rules! op_ld_nn_r {
    ( $self:ident, $src:ident ) => {{
        $self.fetch_store($self.registers.$src)
    }};
}

macro_rules! op_ld_nn_rr {
    ( $self:ident, $src:ident ) => {{
        let addr = $self.fetch_word();
        let value = register16_get!($self, $src);
        $self.mmu.word_set(addr, value)
    }};
}
