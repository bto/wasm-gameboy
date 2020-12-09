use super::*;

#[test]
fn new() {
    let gpu = GPU::new();
    assert_eq!(gpu.memory[0], 0);
}

#[test]
fn byte() {
    let mut gpu = GPU::new();

    gpu.byte_set(0x8000, 10);
    assert_eq!(gpu.byte_get(0x8000), 10);
    assert_eq!(gpu.memory[0], 10);

    gpu.byte_set(0x9FFF, 23);
    assert_eq!(gpu.byte_get(0x9FFF), 23);
    assert_eq!(gpu.memory[0x1FFF], 23);
}
