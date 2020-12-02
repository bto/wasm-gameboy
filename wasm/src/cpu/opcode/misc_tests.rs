use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_cpl_r() {
    let mut cpu = CPU::new();
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00101111);
    cpu.registers.a = 0b10010110;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0b01101001);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
}
