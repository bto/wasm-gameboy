macro_rules! op_ldd_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $src);
        $self.registers.$dest = $self.mmu.byte_get(addr);
        register16_set!($self, $src, addr - 1);
    }};
}

macro_rules! op_ldi_r_rr {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $src);
        $self.registers.$dest = $self.mmu.byte_get(addr);
        register16_set!($self, $src, addr + 1);
    }};
}
