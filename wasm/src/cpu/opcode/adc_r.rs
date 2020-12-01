macro_rules! op_adc_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest as u16;
        let y = $value as u16;
        let c = $self.registers.carry as u16;
        let r = x + y + c;
        $self.registers.$dest = r as u8;
        $self.registers.carry = r > 0xFF;
        $self.registers.half_carry = (x & 0xF) + (y & 0xF) + c > 0xF;
        $self.registers.subtraction = false;
        $self.registers.zero = (r as u8) == 0;
    }};
}

macro_rules! op_adc_r_n {
    ( $self:ident, $dest:ident ) => {{
        let value = $self.fetch_byte();
        op_adc_r!($self, $dest, value)
    }};
}

macro_rules! op_adc_r_r {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = $self.registers.$src;
        op_adc_r!($self, $dest, value)
    }};
}

macro_rules! op_adc_r_rrn {
    ( $self:ident, $dest:ident, $src:ident ) => {{
        let value = register16_load!($self, $src);
        op_adc_r!($self, $dest, value)
    }};
}
