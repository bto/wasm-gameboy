macro_rules! op_jp {
    ( $self:ident, $addr:expr ) => {{
        $self.registers.pc = $addr;
    }};
}

macro_rules! op_jp_nn {
    ( $self:ident ) => {{
        op_jp!($self, $self.fetch_word());
    }};
}
