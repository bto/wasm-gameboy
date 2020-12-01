use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_add_sp_n() {
    let mut cpu = CPU::new();

    let opcode = 0b11101000;

    // 0x88 + 0x88
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0x88);
    cpu.registers.sp = 0x88;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.sp, 0x110);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    // 0 + 0
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode, 0);
    cpu.registers.sp = 0;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.sp, 0);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);
}
