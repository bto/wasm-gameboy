use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_lddi_rr_r() {
    let mut cpu = CPU::new();

    // LDI (HL), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_10_0010);
    cpu.registers.h = 4;
    cpu.registers.l = 2;
    cpu.registers.a = 5;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x402), 5);
    assert_eq!(cpu.registers.l, 3);

    // LDD (HL), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_11_0010);
    cpu.registers.h = 5;
    cpu.registers.l = 4;
    cpu.registers.a = 6;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x504), 6);
    assert_eq!(cpu.registers.l, 3);
}
