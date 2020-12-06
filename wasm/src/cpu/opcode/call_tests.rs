use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_call_nn() {
    let mut cpu = CPU::new();
    let opcode = 0xCD;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.sp = 0x202;
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x302);
    assert_eq!(cpu.registers.sp, 0x200);
    assert_eq!(cpu.mmu.byte_get(0x201), 0x03);
    assert_eq!(cpu.mmu.byte_get(0x200), 0x02);
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
    assert_eq!(cpu.registers.sp, 0x1FE);
    assert_eq!(cpu.mmu.byte_get(0x1FF), 0x04);
    assert_eq!(cpu.mmu.byte_get(0x1FE), 0x03);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_call_nn_c() {
    let mut cpu = CPU::new();
    let opcode = 0xDC;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.sp = 0x202;
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x302);
    assert_eq!(cpu.registers.sp, 0x200);
    assert_eq!(cpu.mmu.byte_get(0x201), 0x03);
    assert_eq!(cpu.mmu.byte_get(0x200), 0x02);
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
    assert_eq!(cpu.registers.sp, 0x200);
    assert_eq!(cpu.mmu.byte_get(0x1FF), 0);
    assert_eq!(cpu.mmu.byte_get(0x1FE), 0);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_call_nn_nc() {
    let mut cpu = CPU::new();
    let opcode = 0xD4;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.sp = 0x202;
    cpu.registers.carry = true;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.registers.sp, 0x202);
    assert_eq!(cpu.mmu.byte_get(0x201), 0);
    assert_eq!(cpu.mmu.byte_get(0x200), 0);
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
    assert_eq!(cpu.registers.sp, 0x200);
    assert_eq!(cpu.mmu.byte_get(0x201), 0x04);
    assert_eq!(cpu.mmu.byte_get(0x200), 0x03);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_call_nn_z() {
    let mut cpu = CPU::new();
    let opcode = 0xCC;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.sp = 0x202;
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, 0x302);
    assert_eq!(cpu.registers.sp, 0x200);
    assert_eq!(cpu.mmu.byte_get(0x201), 0x03);
    assert_eq!(cpu.mmu.byte_get(0x200), 0x02);
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
    assert_eq!(cpu.registers.sp, 0x200);
    assert_eq!(cpu.mmu.byte_get(0x1FF), 0);
    assert_eq!(cpu.mmu.byte_get(0x1FE), 0);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_call_nn_nz() {
    let mut cpu = CPU::new();
    let opcode = 0xC4;

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x02, 0x03);
    cpu.registers.sp = 0x202;
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.registers.sp, 0x202);
    assert_eq!(cpu.mmu.byte_get(0x201), 0);
    assert_eq!(cpu.mmu.byte_get(0x200), 0);
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
    assert_eq!(cpu.registers.sp, 0x200);
    assert_eq!(cpu.mmu.byte_get(0x201), 0x04);
    assert_eq!(cpu.mmu.byte_get(0x200), 0x03);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}
