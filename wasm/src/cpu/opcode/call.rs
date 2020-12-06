macro_rules! op_call_nn {
    ( $self:ident ) => {{
        jump!($self, $self.fetch_word());
        stack_push!($self, $self.registers.pc);
    }};
}
