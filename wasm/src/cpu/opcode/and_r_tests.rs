use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_and_r_n() {
    let mut cpu = CPU::new();
    let opcode = 0b11_100_110;

    // 0b1100 & 0b1010
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0b1010);
    cpu.registers.a = 0b1100;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 0b1000);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    // 0b100 & 0b010
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0b010);
    cpu.registers.a = 0b100;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_and_r_r() {
    let mut cpu = CPU::new();

    let opcode_base = 0b10100_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101].iter() {
        let opcode = opcode_base | i;

        // 0b1100 & 0b1010
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0b1100;
        match i {
            0b000 => cpu.registers.b = 0b1010,
            0b001 => cpu.registers.c = 0b1010,
            0b010 => cpu.registers.d = 0b1010,
            0b011 => cpu.registers.e = 0b1010,
            0b100 => cpu.registers.h = 0b1010,
            0b101 => cpu.registers.l = 0b1010,
            0b111 => cpu.registers.a = 0b1010,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0b1000);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, false);

        // 0b100 & 0b010
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.a = 0b100;
        match i {
            0b000 => cpu.registers.b = 0b010,
            0b001 => cpu.registers.c = 0b010,
            0b010 => cpu.registers.d = 0b010,
            0b011 => cpu.registers.e = 0b010,
            0b100 => cpu.registers.h = 0b010,
            0b101 => cpu.registers.l = 0b010,
            0b111 => cpu.registers.a = 0b010,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, true);
    }

    // AND A(0xF), A(0xF)
    let opcode = 0b10100_111;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0xF;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0xF);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    // AND A(0), A(0)
    let opcode = 0b10100_111;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_and_r_rrn() {
    let mut cpu = CPU::new();
    let opcode = 0b10100_110;

    // 0b1100 & 0b1010
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0b1100;
    cpu.registers.h = 1;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x102, 0b1010);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0b1000);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    // 0b100 & 0b010
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0b100;
    cpu.registers.h = 2;
    cpu.registers.l = 3;
    cpu.mmu.byte_set(0x203, 0b010);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);
}
