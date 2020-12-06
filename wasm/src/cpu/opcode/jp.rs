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
            op_jp!($self, $self.fetch_word());
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
            op_jp!($self, $self.fetch_word());
        }
    }};
}

macro_rules! op_jp_nn_z {
    ( $self:ident ) => {{
        if $self.registers.zero {
            op_jp!($self, $self.fetch_word());
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
            op_jp!($self, $self.fetch_word());
        }
    }};
}
