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
        op_jp!($self, register16_get!($self, $src));
    }};
}

macro_rules! op_jr_n {
    ( $self:ident ) => {{
        let n = $self.fetch_byte() as u16;
        op_jp!($self, $self.registers.pc + n);
    }};
}

macro_rules! op_jr_n_c {
    ( $self:ident ) => {{
        if $self.registers.carry {
            op_jr_n!($self);
        } else {
            $self.registers.pc += 1;
        }
    }};
}

macro_rules! op_jr_n_nc {
    ( $self:ident ) => {{
        if $self.registers.carry {
            $self.registers.pc += 1;
        } else {
            op_jr_n!($self);
        }
    }};
}

macro_rules! op_jr_n_z {
    ( $self:ident ) => {{
        if $self.registers.zero {
            op_jr_n!($self);
        } else {
            $self.registers.pc += 1;
        }
    }};
}

macro_rules! op_jr_n_nz {
    ( $self:ident ) => {{
        if $self.registers.zero {
            $self.registers.pc += 1;
        } else {
            op_jr_n!($self);
        }
    }};
}
