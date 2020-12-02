use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_dec_r() {
    let mut cpu = CPU::new();

    let opcode_base = 0b00_000_101;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let opcode = opcode_base | (i << 3);

        // not overflow
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        match i {
            0b000 => cpu.registers.b = 1,
            0b001 => cpu.registers.c = 1,
            0b010 => cpu.registers.d = 1,
            0b011 => cpu.registers.e = 1,
            0b100 => cpu.registers.h = 1,
            0b101 => cpu.registers.l = 1,
            0b111 => cpu.registers.a = 1,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = true;
        cpu.registers.half_carry = true;
        cpu.registers.subtraction = true;
        cpu.registers.zero = false;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, true);
        assert_eq!(cpu.registers.zero, true);
        match i {
            0b000 => assert_eq!(cpu.registers.b, 0),
            0b001 => assert_eq!(cpu.registers.c, 0),
            0b010 => assert_eq!(cpu.registers.d, 0),
            0b011 => assert_eq!(cpu.registers.e, 0),
            0b100 => assert_eq!(cpu.registers.h, 0),
            0b101 => assert_eq!(cpu.registers.l, 0),
            0b111 => assert_eq!(cpu.registers.a, 0),
            _ => panic!("never reach"),
        }

        // overflow
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        match i {
            0b000 => cpu.registers.b = 0,
            0b001 => cpu.registers.c = 0,
            0b010 => cpu.registers.d = 0,
            0b011 => cpu.registers.e = 0,
            0b100 => cpu.registers.h = 0,
            0b101 => cpu.registers.l = 0,
            0b111 => cpu.registers.a = 0,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = false;
        cpu.registers.half_carry = false;
        cpu.registers.subtraction = false;
        cpu.registers.zero = true;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, true);
        assert_eq!(cpu.registers.zero, false);
        match i {
            0b000 => assert_eq!(cpu.registers.b, 0xFF),
            0b001 => assert_eq!(cpu.registers.c, 0xFF),
            0b010 => assert_eq!(cpu.registers.d, 0xFF),
            0b011 => assert_eq!(cpu.registers.e, 0xFF),
            0b100 => assert_eq!(cpu.registers.h, 0xFF),
            0b101 => assert_eq!(cpu.registers.l, 0xFF),
            0b111 => assert_eq!(cpu.registers.a, 0xFF),
            _ => panic!("never reach"),
        }
    }
}

#[test]
fn op_dec_rr() {
    let mut cpu = CPU::new();

    let opcode_base = 0b00_00_1011;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let opcode = opcode_base | (i << 4);

        // not overfow
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        match i {
            0b00 => cpu.registers.bc_set(1),
            0b01 => cpu.registers.de_set(2),
            0b10 => cpu.registers.hl_set(3),
            0b11 => cpu.registers.sp = 4,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = true;
        cpu.registers.half_carry = true;
        cpu.registers.subtraction = true;
        cpu.registers.zero = true;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        match i {
            0b00 => assert_eq!(cpu.registers.bc_get(), 0),
            0b01 => assert_eq!(cpu.registers.de_get(), 1),
            0b10 => assert_eq!(cpu.registers.hl_get(), 2),
            0b11 => assert_eq!(cpu.registers.sp, 3),
            _ => panic!("never reach"),
        }
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, true);
        assert_eq!(cpu.registers.zero, true);

        // overfow
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        match i {
            0b00 => cpu.registers.bc_set(0),
            0b01 => cpu.registers.de_set(0),
            0b10 => cpu.registers.hl_set(0),
            0b11 => cpu.registers.sp = 0,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = false;
        cpu.registers.half_carry = false;
        cpu.registers.subtraction = false;
        cpu.registers.zero = false;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        match i {
            0b00 => assert_eq!(cpu.registers.bc_get(), 0xFFFF),
            0b01 => assert_eq!(cpu.registers.de_get(), 0xFFFF),
            0b10 => assert_eq!(cpu.registers.hl_get(), 0xFFFF),
            0b11 => assert_eq!(cpu.registers.sp, 0xFFFF),
            _ => panic!("never reach"),
        }
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, false);
    }
}

#[test]
fn op_dec_rrn() {
    let mut cpu = CPU::new();
    let opcode = 0b00_110_101;

    // not overflow
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.h = 1;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x102, 1);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
    assert_eq!(cpu.mmu.byte_get(0x102), 0);

    // overflow
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.h = 2;
    cpu.registers.l = 3;
    cpu.mmu.byte_set(0x203, 0);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
    assert_eq!(cpu.mmu.byte_get(0x203), 0xFF);
}
