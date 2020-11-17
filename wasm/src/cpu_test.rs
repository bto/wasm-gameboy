use super::*;

macro_rules! set_inst {
    ( $cpu:ident, $pc:ident, $v1:expr, $v2:expr ) => {
        $cpu.mmu.byte_set($pc + 0, $v1);
        $cpu.mmu.byte_set($pc + 1, $v2);
    };

    ( $cpu:ident, $pc:ident, $v1:expr ) => {
        $cpu.mmu.byte_set($pc + 0, $v1);
    };
}

#[test]
fn test_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.mmu, MMU::new());
    assert_eq!(cpu.registers, Registers::new());
}

#[test]
fn test_op_ld_hl_n() {
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
fn test_op_ld_hl_r() {
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
fn test_op_ld_r_hl() {
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
fn test_op_ld_r_n() {
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
fn test_op_ld_r_r() {
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
