macro_rules! op_add_sp {
    ( $self:ident, $value:expr ) => {{
        let x = $self.registers.sp;
        let y = $value as u16;
        $self.registers.sp = x.wrapping_add(y);
        $self.registers.carry = ((x & 0xFF) + (y & 0xFF)) > 0xFF;
        $self.registers.half_carry = ((x & 0xF) + (y & 0xF)) > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = false;
    }};
}

macro_rules! op_add_sp_n {
    ( $self:ident ) => {{
        let value = $self.fetch_byte();
        op_add_sp!($self, value)
    }};
}
