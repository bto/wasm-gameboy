use super::*;

#[test]
fn test_new() {
    let bus = MemoryBus::new();
    assert_eq!(bus.memory[0], 0);
}

#[test]
fn test_byte() {
    let mut bus = MemoryBus::new();
    assert_eq!(bus.get_byte(0x100), 0);

    bus.set_byte(0x100, 10);
    assert_eq!(bus.get_byte(0x100), 10);
}
