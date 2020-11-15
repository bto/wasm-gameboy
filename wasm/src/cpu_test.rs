use super::*;

#[test]
fn test_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.memory, Memory::new());
    assert_eq!(cpu.registers, Registers::new());
}
