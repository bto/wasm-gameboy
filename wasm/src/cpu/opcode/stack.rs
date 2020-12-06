macro_rules! op_pop_rr {
    ( $self:ident, $src:ident ) => {{
        let addr = stack_pop!($self);
        register16_set!($self, $src, addr)
    }};
}

macro_rules! op_push_rr {
    ( $self:ident, $src:ident ) => {{
        let addr = register16_get!($self, $src);
        stack_push!($self, addr);
    }};
}
