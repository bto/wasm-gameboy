use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_bit_r() {
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
