macro_rules! register16_get {
    ( $self:ident, af ) => {
        register16_get!($self, af_get)
    };

    ( $self:ident, bc ) => {
        register16_get!($self, bc_get)
    };

    ( $self:ident, de ) => {
        register16_get!($self, de_get)
    };

    ( $self:ident, hl ) => {
        register16_get!($self, hl_get)
    };

    ( $self:ident, sp ) => {
        $self.registers.sp
    };

    ( $self:ident, $method:ident ) => {
        $self.registers.$method()
    };
}

macro_rules! register16_set {
    ( $self:ident, af, $value:expr ) => {
        register16_set!($self, af_set, $value)
    };

    ( $self:ident, bc, $value:expr ) => {
        register16_set!($self, bc_set, $value)
    };

    ( $self:ident, de, $value:expr ) => {
        register16_set!($self, de_set, $value)
    };

    ( $self:ident, hl, $value:expr ) => {
        register16_set!($self, hl_set, $value)
    };

    ( $self:ident, sp, $value:expr ) => {
        $self.registers.sp = $value
    };

    ( $self:ident, $method:ident, $value:expr ) => {
        $self.registers.$method($value)
    };
}

macro_rules! register16_load {
    ( $self:ident, $register:ident ) => {{
        let addr = register16_get!($self, $register);
        $self.mmu.byte_get(addr)
    }};
}

macro_rules! register16_store {
    ( $self:ident, $register:ident, $value:expr ) => {{
        let addr = register16_get!($self, $register);
        $self.mmu.byte_set(addr, $value);
    }};
}

macro_rules! set_rotate_shift_flags {
    ( $self:ident, $result:expr, $carry:expr ) => {{
        $self.registers.carry = $carry;
        $self.registers.half_carry = false;
        $self.registers.subtraction = false;
        $self.registers.zero = $result == 0;
    }};
}
