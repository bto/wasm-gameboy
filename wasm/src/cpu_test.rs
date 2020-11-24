use super::*;

macro_rules! set_inst {
    ( $cpu:ident, $pc:ident, $v1:expr ) => {
        $cpu.mmu.byte_set($pc + 0, $v1);
    };

    ( $cpu:ident, $pc:ident, $v1:expr, $v2:expr ) => {
        $cpu.mmu.byte_set($pc + 0, $v1);
        $cpu.mmu.byte_set($pc + 1, $v2);
    };

    ( $cpu:ident, $pc:ident, $v1:expr, $v2:expr, $v3:expr ) => {
        $cpu.mmu.byte_set($pc + 0, $v1);
        $cpu.mmu.byte_set($pc + 1, $v2);
        $cpu.mmu.byte_set($pc + 2, $v3);
    };
}

#[test]
fn test_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.mmu, MMU::new());
    assert_eq!(cpu.registers, Registers::new());
}

#[test]
fn test_op_ld_r_n() {
    // LD r, n
    let mut cpu = CPU::new();

    let opcode_base = 0b00_000_110;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | (i << 3);
        let v = *i;
        set_inst!(cpu, pc, opcode, v);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 2);
        match i {
            0b000 => assert_eq!(cpu.registers.b, v),
            0b001 => assert_eq!(cpu.registers.c, v),
            0b010 => assert_eq!(cpu.registers.d, v),
            0b011 => assert_eq!(cpu.registers.e, v),
            0b100 => assert_eq!(cpu.registers.h, v),
            0b101 => assert_eq!(cpu.registers.l, v),
            0b111 => assert_eq!(cpu.registers.a, v),
            _ => panic!("never reach"),
        }
    }
}

#[test]
fn test_op_ld_rr_n() {
    // LD (HL), n
    let mut cpu = CPU::new();

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_110_110, 1);

    cpu.registers.h = 2;
    cpu.registers.l = 0;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.mmu.byte_get(0x200), 1);
}

#[test]
fn test_op_ld_r_r() {
    // LD r, r
    let mut cpu = CPU::new();

    let opcode_base = 0b01_000_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        for j in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
            let pc = cpu.registers.pc;
            let opcode = opcode_base | (i << 3) | j;
            set_inst!(cpu, pc, opcode);

            let v = i << 4 | j;
            match j {
                0b000 => cpu.registers.b = v,
                0b001 => cpu.registers.c = v,
                0b010 => cpu.registers.d = v,
                0b011 => cpu.registers.e = v,
                0b100 => cpu.registers.h = v,
                0b101 => cpu.registers.l = v,
                0b111 => cpu.registers.a = v,
                _ => panic!("never reach"),
            }

            cpu.execute();

            assert_eq!(cpu.registers.pc, pc + 1);
            match i {
                0b000 => assert_eq!(cpu.registers.b, v),
                0b001 => assert_eq!(cpu.registers.c, v),
                0b010 => assert_eq!(cpu.registers.d, v),
                0b011 => assert_eq!(cpu.registers.e, v),
                0b100 => assert_eq!(cpu.registers.h, v),
                0b101 => assert_eq!(cpu.registers.l, v),
                0b111 => assert_eq!(cpu.registers.a, v),
                _ => panic!("never reach"),
            }
        }
    }
}

#[test]
fn test_op_ld_r_hl() {
    // LD r, (HL)
    let mut cpu = CPU::new();

    let opcode_base = 0b01_000_110;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | (i << 3);
        set_inst!(cpu, pc, opcode);

        let v = *i + 10;
        cpu.registers.h = 2;
        cpu.registers.l = *i;
        cpu.mmu.byte_set(cpu.registers.hl_get(), v);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 1);
        match i {
            0b000 => assert_eq!(cpu.registers.b, v),
            0b001 => assert_eq!(cpu.registers.c, v),
            0b010 => assert_eq!(cpu.registers.d, v),
            0b011 => assert_eq!(cpu.registers.e, v),
            0b100 => assert_eq!(cpu.registers.h, v),
            0b101 => assert_eq!(cpu.registers.l, v),
            0b111 => assert_eq!(cpu.registers.a, v),
            _ => panic!("never reach"),
        }
    }
}

#[test]
fn test_op_ld_hl_r() {
    // LD (HL), r
    let mut cpu = CPU::new();

    let opcode_base = 0b01_110_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i;
        set_inst!(cpu, pc, opcode);

        cpu.registers.h = 2;
        cpu.registers.l = 0;

        let v = *i;
        match i {
            0b000 => cpu.registers.b = v,
            0b001 => cpu.registers.c = v,
            0b010 => cpu.registers.d = v,
            0b011 => cpu.registers.e = v,
            0b100 => cpu.registers.h = v,
            0b101 => cpu.registers.l = v,
            0b111 => cpu.registers.a = v,
            _ => panic!("never reach"),
        }

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.mmu.byte_get(cpu.registers.hl_get()), v);
    }
}

#[test]
fn test_op_ld_r_rr() {
    // LD A, (rr)
    let mut cpu = CPU::new();

    // LD A, (BC)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_00_1010);
    cpu.registers.b = 2;
    cpu.registers.c = 1;
    cpu.mmu.byte_set(0x201, 3);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x201), 3);

    // LD A, (DE)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_01_1010);
    cpu.registers.d = 3;
    cpu.registers.e = 2;
    cpu.mmu.byte_set(0x302, 4);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x302), 4);

    // LDI A, (HL)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_10_1010);
    cpu.registers.h = 4;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x402, 5);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 5);
    assert_eq!(cpu.registers.h, 4);
    assert_eq!(cpu.registers.l, 3);

    // LDD A, (HL)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_11_1010);
    cpu.registers.h = 5;
    cpu.registers.l = 4;
    cpu.mmu.byte_set(0x504, 6);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 6);
    assert_eq!(cpu.registers.h, 5);
    assert_eq!(cpu.registers.l, 3);
}

#[test]
fn test_op_ld_rr_r() {
    let mut cpu = CPU::new();

    // LD (BC), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_00_0010);
    cpu.registers.b = 2;
    cpu.registers.c = 1;
    cpu.registers.a = 3;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x201), 3);

    // LD (DE), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_01_0010);
    cpu.registers.d = 3;
    cpu.registers.e = 2;
    cpu.registers.a = 4;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x302), 4);

    // LDI (HL), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_10_0010);
    cpu.registers.h = 4;
    cpu.registers.l = 2;
    cpu.registers.a = 5;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x402), 5);
    assert_eq!(cpu.registers.l, 3);

    // LDD (HL), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_11_0010);
    cpu.registers.h = 5;
    cpu.registers.l = 4;
    cpu.registers.a = 6;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x504), 6);
    assert_eq!(cpu.registers.l, 3);
}

#[test]
fn test_op_ld_nn_r() {
    let mut cpu = CPU::new();

    // LD (nn), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11101010, 2, 3);

    cpu.registers.a = 4;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.mmu.byte_get(0x302), 4);
}

#[test]
fn test_op_ld_r_nn() {
    let mut cpu = CPU::new();

    // LD A, (nn)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11111010, 2, 3);

    cpu.mmu.byte_set(0x302, 4);

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.registers.a, 4);
}

#[test]
fn test_op_ldh_rh_r() {
    let mut cpu = CPU::new();

    // LDH (C), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11100010);

    cpu.registers.a = 1;
    cpu.registers.c = 2;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0xFF02), 1);
}

#[test]
fn test_op_ldh_r_rh() {
    let mut cpu = CPU::new();

    // LDH A, (C)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11110010);

    cpu.registers.c = 1;
    cpu.mmu.byte_set(0xFF01, 2);

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 2);
}

#[test]
fn test_op_ldh_nh_r() {
    let mut cpu = CPU::new();

    // LDH (n), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11100000, 1);

    cpu.registers.a = 2;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.mmu.byte_get(0xFF01), 2);
}

#[test]
fn test_op_ldh_r_nh() {
    let mut cpu = CPU::new();

    // LDH A, (n)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11110000, 1);

    cpu.mmu.byte_set(0xFF01, 2);

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 2);
}

#[test]
fn test_op_ld_rr_nn() {
    let mut cpu = CPU::new();

    // LD rr, nn
    let opcode_base = 0b00_00_0001;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i << 4;
        let lsb = *i;
        let msb = lsb + 1;
        set_inst!(cpu, pc, opcode, lsb, msb);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 3);
        let v = (msb as u16) << 8 | lsb as u16;
        match i {
            0b00 => assert_eq!(cpu.registers.bc_get(), v),
            0b01 => assert_eq!(cpu.registers.de_get(), v),
            0b10 => assert_eq!(cpu.registers.hl_get(), v),
            0b11 => assert_eq!(cpu.registers.sp, v),
            _ => panic!("never reach"),
        }
    }
}

#[test]
fn test_op_ld_nn_rr() {
    let mut cpu = CPU::new();

    // LD (nn), SP
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00001000, 2, 3);

    cpu.registers.sp = 0x405;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.mmu.byte_get(0x302), 5);
    assert_eq!(cpu.mmu.byte_get(0x303), 4);
}

#[test]
fn test_op_ld_rr_rr() {
    let mut cpu = CPU::new();

    // LD SP, HL
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11111001);

    cpu.registers.h = 1;
    cpu.registers.l = 2;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.sp, 0x102);
}

#[test]
fn test_op_push_rr() {
    let mut cpu = CPU::new();

    cpu.registers.bc_set(0x102);
    cpu.registers.de_set(0x304);
    cpu.registers.hl_set(0x506);
    cpu.registers.af_set(0x7FF);
    cpu.registers.sp = 0x908;

    let opcode_base = 0b11_00_0101;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i << 4;
        set_inst!(cpu, pc, opcode);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 1);
    }

    assert_eq!(cpu.mmu.byte_get(0x900), 0xF0);
    assert_eq!(cpu.mmu.byte_get(0x901), 7);
    assert_eq!(cpu.mmu.byte_get(0x902), 6);
    assert_eq!(cpu.mmu.byte_get(0x903), 5);
    assert_eq!(cpu.mmu.byte_get(0x904), 4);
    assert_eq!(cpu.mmu.byte_get(0x905), 3);
    assert_eq!(cpu.mmu.byte_get(0x906), 2);
    assert_eq!(cpu.mmu.byte_get(0x907), 1);
}

#[test]
fn test_op_pop_rr() {
    let mut cpu = CPU::new();

    cpu.mmu.word_set(0x200, 0x102);
    cpu.mmu.word_set(0x202, 0x304);
    cpu.mmu.word_set(0x204, 0x506);
    cpu.mmu.word_set(0x206, 0x7FF);
    cpu.registers.sp = 0x200;

    let opcode_base = 0b11_00_0001;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i << 4;
        set_inst!(cpu, pc, opcode);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 1);
    }

    assert_eq!(cpu.registers.b, 1);
    assert_eq!(cpu.registers.c, 2);
}

#[test]
fn test_op_add_r_r() {
    let mut cpu = CPU::new();

    let opcode_base = 0b10000_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let opcode = opcode_base | i;

        // 0x80 + 0x80
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0x80;
        match i {
            0b000 => cpu.registers.b = 0x80,
            0b001 => cpu.registers.c = 0x80,
            0b010 => cpu.registers.d = 0x80,
            0b011 => cpu.registers.e = 0x80,
            0b100 => cpu.registers.h = 0x80,
            0b101 => cpu.registers.l = 0x80,
            0b111 => cpu.registers.a = 0x80,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, true);

        // 0x08 + 0x08
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0x08;
        match i {
            0b000 => cpu.registers.b = 0x08,
            0b001 => cpu.registers.c = 0x08,
            0b010 => cpu.registers.d = 0x08,
            0b011 => cpu.registers.e = 0x08,
            0b100 => cpu.registers.h = 0x08,
            0b101 => cpu.registers.l = 0x08,
            0b111 => cpu.registers.a = 0x08,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, false);
    }
}

#[test]
fn test_op_add_r_rr() {
    let mut cpu = CPU::new();

    let opcode = 0b10000_110;

    // 0x80 + 0x80
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0x80;
    cpu.registers.h = 1;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x102, 0x80);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    // 0x08 + 0x08
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0x08;
    cpu.registers.h = 2;
    cpu.registers.l = 3;
    cpu.mmu.byte_set(0x203, 0x08);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn test_op_adc_r_r() {
    let mut cpu = CPU::new();

    let opcode_base = 0b10001_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let opcode = opcode_base | i;

        // 0x80 + 0x80 + carry(false)
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0x80;
        match i {
            0b000 => cpu.registers.b = 0x80,
            0b001 => cpu.registers.c = 0x80,
            0b010 => cpu.registers.d = 0x80,
            0b011 => cpu.registers.e = 0x80,
            0b100 => cpu.registers.h = 0x80,
            0b101 => cpu.registers.l = 0x80,
            0b111 => cpu.registers.a = 0x80,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = false;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, true);

        // 0x08 + 0x08 + carry(true)
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0x08;
        match i {
            0b000 => cpu.registers.b = 0x08,
            0b001 => cpu.registers.c = 0x08,
            0b010 => cpu.registers.d = 0x08,
            0b011 => cpu.registers.e = 0x08,
            0b100 => cpu.registers.h = 0x08,
            0b101 => cpu.registers.l = 0x08,
            0b111 => cpu.registers.a = 0x08,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = true;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, false);
    }
}

#[test]
fn test_op_adc_r_rr() {
    let mut cpu = CPU::new();

    let opcode = 0b10001_110;

    // 0x80 + 0x80 + carry(false)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0x80;
    cpu.registers.h = 1;
    cpu.registers.l = 2;
    cpu.registers.carry = false;
    cpu.mmu.byte_set(0x102, 0x80);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    // 0x08 + 0x08 + carry(true)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0x08;
    cpu.registers.h = 2;
    cpu.registers.l = 3;
    cpu.registers.carry = true;
    cpu.mmu.byte_set(0x203, 0x08);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0x11);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);
}
