use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_jp_nn() {
    let mut cpu = CPU::new();
    let opcode = 0xC3;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x302);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x03, 0x04);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x403);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_jp_nn_c() {
    let mut cpu = CPU::new();
    let opcode = 0xDA;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x302);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x03, 0x04);
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_jp_nn_nc() {
    let mut cpu = CPU::new();
    let opcode = 0xD2;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x03, 0x04);
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x403);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_jp_nn_z() {
    let mut cpu = CPU::new();
    let opcode = 0xCA;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x302);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x03, 0x04);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_jp_nn_nz() {
    let mut cpu = CPU::new();
    let opcode = 0xC2;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x03, 0x04);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x403);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_jp_rr() {
    let mut cpu = CPU::new();
    let opcode = 0xE9;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.h = 3;
    cpu.registers.l = 2;
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x302);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.h = 4;
    cpu.registers.l = 3;
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x403);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}
