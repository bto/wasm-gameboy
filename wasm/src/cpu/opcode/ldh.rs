macro_rules! op_ldh_r_nh {
    ( $self:ident, $dest:ident ) => {{
        let addr = 0xFF00 | $self.fetch_byte() as u16;
        $self.registers.$dest = $self.mmu.byte_get(addr);
    }};
}

macro_rules! op_ldh_r_rh {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = 0xFF00 | $self.registers.$src as u16;
        $self.registers.$dest = $self.mmu.byte_get(addr);
    }};
}

macro_rules! op_ldh_nh_r {
    ( $self:ident, $src:ident ) => {{
        let addr = 0xFF00 | $self.fetch_byte() as u16;
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
    }};
}

macro_rules! op_ldh_rh_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let addr = 0xFF00 | $self.registers.$dest as u16;
        let value = $self.registers.$src;
        $self.mmu.byte_set(addr, value);
    }};
}
