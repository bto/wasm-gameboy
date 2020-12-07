macro_rules! op_rst {
    ( $self:ident, $addr:expr ) => {{
        stack_push!($self, $self.registers.pc);
        jump!($self, $addr);
    }};
}
