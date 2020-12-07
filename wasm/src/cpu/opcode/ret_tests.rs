use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_ret() {
    let mut cpu = CPU::new();
    let opcode = 0xC9;

    cpu.registers.pc = 0x123;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x234;
    cpu.mmu.byte_set(0x234, 0x45);
    cpu.mmu.byte_set(0x235, 0x03);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x345);
    assert_eq!(cpu.registers.sp, 0x236);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    cpu.registers.pc = 0x231;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x345;
    cpu.mmu.byte_set(0x345, 0x56);
    cpu.mmu.byte_set(0x346, 0x04);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x456);
    assert_eq!(cpu.registers.sp, 0x347);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_ret_c() {
    let mut cpu = CPU::new();
    let opcode = 0xD8;

    cpu.registers.pc = 0x123;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x234;
    cpu.mmu.byte_set(0x234, 0x45);
    cpu.mmu.byte_set(0x235, 0x03);
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x345);
    assert_eq!(cpu.registers.sp, 0x236);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    cpu.registers.pc = 0x231;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x345;
    cpu.mmu.byte_set(0x345, 0x56);
    cpu.mmu.byte_set(0x346, 0x04);
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x232);
    assert_eq!(cpu.registers.sp, 0x345);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_ret_nc() {
    let mut cpu = CPU::new();
    let opcode = 0xD0;

    cpu.registers.pc = 0x123;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x234;
    cpu.mmu.byte_set(0x234, 0x45);
    cpu.mmu.byte_set(0x235, 0x03);
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x124);
    assert_eq!(cpu.registers.sp, 0x234);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    cpu.registers.pc = 0x231;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x345;
    cpu.mmu.byte_set(0x345, 0x56);
    cpu.mmu.byte_set(0x346, 0x04);
    cpu.registers.carry = false;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x456);
    assert_eq!(cpu.registers.sp, 0x347);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_ret_z() {
    let mut cpu = CPU::new();
    let opcode = 0xC8;

    cpu.registers.pc = 0x123;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x234;
    cpu.mmu.byte_set(0x234, 0x45);
    cpu.mmu.byte_set(0x235, 0x03);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x345);
    assert_eq!(cpu.registers.sp, 0x236);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    cpu.registers.pc = 0x231;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x345;
    cpu.mmu.byte_set(0x345, 0x56);
    cpu.mmu.byte_set(0x346, 0x04);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x232);
    assert_eq!(cpu.registers.sp, 0x345);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_ret_nz() {
    let mut cpu = CPU::new();
    let opcode = 0xC0;

    cpu.registers.pc = 0x123;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x234;
    cpu.mmu.byte_set(0x234, 0x45);
    cpu.mmu.byte_set(0x235, 0x03);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x124);
    assert_eq!(cpu.registers.sp, 0x234);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    cpu.registers.pc = 0x231;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x345;
    cpu.mmu.byte_set(0x345, 0x56);
    cpu.mmu.byte_set(0x346, 0x04);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x456);
    assert_eq!(cpu.registers.sp, 0x347);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_reti() {
    let mut cpu = CPU::new();
    let opcode = 0xD9;

    cpu.registers.pc = 0x123;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x234;
    cpu.mmu.byte_set(0x234, 0x45);
    cpu.mmu.byte_set(0x235, 0x03);
    cpu.int_enable = false;
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x345);
    assert_eq!(cpu.registers.sp, 0x236);
    assert_eq!(cpu.int_enable, true);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    cpu.registers.pc = 0x231;
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.sp = 0x345;
    cpu.mmu.byte_set(0x345, 0x56);
    cpu.mmu.byte_set(0x346, 0x04);
    cpu.int_enable = true;
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x456);
    assert_eq!(cpu.registers.sp, 0x347);
    assert_eq!(cpu.int_enable, true);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}
