macro_rules! op_ret {
    ( $self:ident ) => {{
        jump!($self, stack_pop!($self));
    }};
}

macro_rules! op_ret_c {
    ( $self:ident ) => {{
        if $self.registers.carry {
            op_ret!($self);
        }
    }};
}

macro_rules! op_ret_nc {
    ( $self:ident ) => {{
        if !$self.registers.carry {
            op_ret!($self);
        }
    }};
}

macro_rules! op_ret_z {
    ( $self:ident ) => {{
        if $self.registers.zero {
            op_ret!($self);
        }
    }};
}

macro_rules! op_ret_nz {
    ( $self:ident ) => {{
        if !$self.registers.zero {
            op_ret!($self);
        }
    }};
}

macro_rules! op_reti {
    ( $self:ident ) => {{
        jump!($self, stack_pop!($self));
        op_int_enable!($self);
    }};
}
