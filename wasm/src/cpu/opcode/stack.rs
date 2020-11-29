macro_rules! op_pop_rr {
    ( $self:ident, $src:ident ) => {{
        let value = $self.mmu.word_get($self.registers.sp);
        $self.registers.sp += 2;
        register16_set!($self, $src, value)
    }};
}

macro_rules! op_push_rr {
    ( $self:ident, $src:ident ) => {{
        let value = register16_get!($self, $src);
        $self.registers.sp -= 1;
        $self.mmu.byte_set($self.registers.sp, (value >> 8) as u8);
        $self.registers.sp -= 1;
        $self.mmu.byte_set($self.registers.sp, (value & 0xFF) as u8);
    }};
}
