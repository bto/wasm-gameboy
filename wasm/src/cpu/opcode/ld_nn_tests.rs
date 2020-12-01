use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_ld_nn_r() {
    let mut cpu = CPU::new();

    // LD (nn), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11101010, 2, 3);

    cpu.registers.a = 4;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.mmu.byte_get(0x302), 4);
}

#[test]
fn op_ld_nn_rr() {
    let mut cpu = CPU::new();

    // LD (nn), SP
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00001000, 2, 3);

    cpu.registers.sp = 0x405;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 3);
    assert_eq!(cpu.mmu.byte_get(0x302), 5);
    assert_eq!(cpu.mmu.byte_get(0x303), 4);
}
