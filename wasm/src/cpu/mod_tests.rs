use super::*;

#[test]
fn new() {
    let cpu = CPU::new();
    assert_eq!(cpu.mmu, MMU::new());
    assert_eq!(cpu.registers, Registers::new());
}
