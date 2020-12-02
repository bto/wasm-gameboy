use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_add_r_n() {
    let mut cpu = CPU::new();
    let opcode = 0b11_000_110;

    // 0x80 + 0x80
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x80);
    cpu.registers.a = 0x80;
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    // 0x08 + 0x08
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x08);
    cpu.registers.a = 0x08;
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_add_r_r() {
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
        cpu.registers.carry = false;
        cpu.registers.half_carry = true;
        cpu.registers.subtraction = true;
        cpu.registers.zero = false;
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
        cpu.registers.carry = true;
        cpu.registers.half_carry = false;
        cpu.registers.subtraction = false;
        cpu.registers.zero = true;
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
fn op_add_r_rrn() {
    let mut cpu = CPU::new();
    let opcode = 0b10000_110;

    // 0x80 + 0x80
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0x80;
    cpu.registers.h = 1;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x102, 0x80);
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
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
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);
}
