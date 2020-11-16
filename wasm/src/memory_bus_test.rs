use super::*;

#[test]
fn test_new() {
    let bus = MemoryBus::new();
    assert_eq!(bus.memory[0], 0);
}

#[test]
fn test_byte() {
    let mut bus = MemoryBus::new();
    assert_eq!(bus.byte_get(0x100), 0);

    bus.byte_set(0x100, 10);
    assert_eq!(bus.byte_get(0x100), 10);
}
