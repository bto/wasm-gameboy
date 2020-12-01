use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_cp_r_n() {
    let mut cpu = CPU::new();
    let opcode = 0b11_111_110;

    // 0x11 - 0x22
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x22);
    cpu.registers.a = 0x11;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 0x11);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);

    // 0xFF - 0xFF
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0xFF);
    cpu.registers.a = 0xFF;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 0xFF);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_cp_r_r() {
    let mut cpu = CPU::new();

    let opcode_base = 0b10111_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101].iter() {
        let opcode = opcode_base | i;

        // 0x11 - 0x22
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0x11;
        match i {
            0b000 => cpu.registers.b = 0x22,
            0b001 => cpu.registers.c = 0x22,
            0b010 => cpu.registers.d = 0x22,
            0b011 => cpu.registers.e = 0x22,
            0b100 => cpu.registers.h = 0x22,
            0b101 => cpu.registers.l = 0x22,
            0b111 => cpu.registers.a = 0x22,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, true);
        assert_eq!(cpu.registers.zero, false);

        // 0xFF - 0xFF
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0xFF;
        match i {
            0b000 => cpu.registers.b = 0xFF,
            0b001 => cpu.registers.c = 0xFF,
            0b010 => cpu.registers.d = 0xFF,
            0b011 => cpu.registers.e = 0xFF,
            0b100 => cpu.registers.h = 0xFF,
            0b101 => cpu.registers.l = 0xFF,
            0b111 => cpu.registers.a = 0xFF,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, true);
        assert_eq!(cpu.registers.zero, true);
    }

    // CP A, A
    let opcode = 0b10111_111;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0x10;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_cp_r_rrn() {
    let mut cpu = CPU::new();
    let opcode = 0b10111_110;

    // 0x11 - 0x22
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0x11;
    cpu.registers.h = 1;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x102, 0x22);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0x11);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);

    // 0xFF - 0xFF
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0xFF;
    cpu.registers.h = 2;
    cpu.registers.l = 3;
    cpu.mmu.byte_set(0x203, 0xFF);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0xFF);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}
