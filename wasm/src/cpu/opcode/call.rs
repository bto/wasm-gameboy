macro_rules! op_call_nn {
    ( $self:ident ) => {{
        jump!($self, $self.fetch_word());
        stack_push!($self, $self.registers.pc);
    }};
}

macro_rules! op_call_nn_c {
    ( $self:ident ) => {{
        if $self.registers.carry {
            op_call_nn!($self);
        } else {
            $self.registers.pc += 2;
        }
    }};
}

macro_rules! op_call_nn_nc {
    ( $self:ident ) => {{
        if $self.registers.carry {
            $self.registers.pc += 2;
        } else {
            op_call_nn!($self);
        }
    }};
}

macro_rules! op_call_nn_z {
    ( $self:ident ) => {{
        if $self.registers.zero {
            op_call_nn!($self);
        } else {
            $self.registers.pc += 2;
        }
    }};
}

macro_rules! op_call_nn_nz {
    ( $self:ident ) => {{
        if $self.registers.zero {
            $self.registers.pc += 2;
        } else {
            op_call_nn!($self);
        }
    }};
}
