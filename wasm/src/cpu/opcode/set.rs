macro_rules! op_set {
    ( $self:ident, $x:expr, $y:expr ) => {{
        let x = $x;
        let y = $y;
        x | 1 << y
    }};
}

macro_rules! op_set_r {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = $self.registers.$dest;
        let r = op_set!($self, x, $value);
        $self.registers.$dest = r;
    }};
}

macro_rules! op_set_rrn {
    ( $self:ident, $dest:ident, $value:expr ) => {{
        let x = register16_load!($self, $dest);
        let r = op_set!($self, x, $value);
        register16_store!($self, $dest, r);
    }};
}
