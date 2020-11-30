macro_rules! set_inst {
    ( $cpu:ident, $pc:ident, $v1:expr ) => {{
        $cpu.mmu.byte_set($pc + 0, $v1)
    }};

    ( $cpu:ident, $pc:ident, $v1:expr, $v2:expr ) => {{
        set_inst!($cpu, $pc, $v1);
        $cpu.mmu.byte_set($pc + 1, $v2)
    }};

    ( $cpu:ident, $pc:ident, $v1:expr, $v2:expr, $v3:expr ) => {{
        set_inst!($cpu, $pc, $v1, $v2);
        $cpu.mmu.byte_set($pc + 2, $v3)
    }};
}
