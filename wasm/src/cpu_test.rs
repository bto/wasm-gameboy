use super::*;

#[test]
fn test_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.bus, MemoryBus::new());
    assert_eq!(cpu.registers, Registers::new());
}
