macro_rules! op_jr_n {
    ( $self:ident ) => {{
        let n = $self.fetch_byte() as u16;
        jump!($self, $self.registers.pc + n);
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
