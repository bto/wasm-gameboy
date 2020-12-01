use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_ld_r_rr() {
    let mut cpu = CPU::new();

    // LDI A, (HL)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_10_1010);
    cpu.registers.h = 4;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x402, 5);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 5);
    assert_eq!(cpu.registers.h, 4);
    assert_eq!(cpu.registers.l, 3);

    // LDD A, (HL)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_11_1010);
    cpu.registers.h = 5;
    cpu.registers.l = 4;
    cpu.mmu.byte_set(0x504, 6);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 6);
    assert_eq!(cpu.registers.h, 5);
    assert_eq!(cpu.registers.l, 3);
}
