macro_rules! op_jp_nn {
    ( $self:ident ) => {{
        jump!($self, $self.fetch_word());
    }};
}

macro_rules! op_jp_nn_c {
    ( $self:ident ) => {{
        if $self.registers.carry {
            op_jp_nn!($self);
        } else {
            $self.registers.pc += 2;
        }
    }};
}

macro_rules! op_jp_nn_nc {
    ( $self:ident ) => {{
        if $self.registers.carry {
            $self.registers.pc += 2;
        } else {
            op_jp_nn!($self);
        }
    }};
}

macro_rules! op_jp_nn_z {
    ( $self:ident ) => {{
        if $self.registers.zero {
            op_jp_nn!($self);
        } else {
            $self.registers.pc += 2;
        }
    }};
}

macro_rules! op_jp_nn_nz {
    ( $self:ident ) => {{
        if $self.registers.zero {
            $self.registers.pc += 2;
        } else {
            op_jp_nn!($self);
        }
    }};
}

macro_rules! op_jp_rr {
    ( $self:ident, $src:ident ) => {{
        jump!($self, register16_get!($self, $src));
    }};
}
