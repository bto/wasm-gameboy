macro_rules! op_rlc_r {
    ( $self:ident, $dest:ident ) => {{
        let x = $self.registers.$dest;
        let c = x & 0x80 == 0x80;
        let r = x << 1 | $self.registers.carry as u8;
        $self.registers.$dest = r;
        $self.registers.carry = c;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = r == 0;
    }};
}
