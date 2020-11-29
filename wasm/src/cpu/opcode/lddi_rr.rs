macro_rules! op_ldd_rr_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $dest);
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
        register16_set!($self, $dest, addr - 1);
    }};
}

macro_rules! op_ldi_rr_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = register16_get!($self, $dest);
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
        register16_set!($self, $dest, addr + 1);
    }};
}
