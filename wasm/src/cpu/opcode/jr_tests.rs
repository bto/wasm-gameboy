use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_jr_n() {
    let mut cpu = CPU::new();
    let opcode = 0x18;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x80);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2 + 0x80);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0xFF);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2 + 0xFF);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_jr_n_c() {
    let mut cpu = CPU::new();
    let opcode = 0x38;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x80);
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2 + 0x80);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0xFF);
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_jr_n_nc() {
    let mut cpu = CPU::new();
    let opcode = 0x30;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x80);
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0xFF);
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2 + 0xFF);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_jr_n_z() {
    let mut cpu = CPU::new();
    let opcode = 0x28;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x80);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2 + 0x80);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0xFF);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_jr_n_nz() {
    let mut cpu = CPU::new();
    let opcode = 0x20;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x80);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0xFF);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2 + 0xFF);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}
