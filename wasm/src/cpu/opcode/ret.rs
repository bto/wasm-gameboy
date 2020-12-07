macro_rules! op_ret {
    ( $self:ident ) => {{
        jump!($self, stack_pop!($self));
    }};
}
