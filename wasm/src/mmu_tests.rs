use super::*;

#[test]
fn new() {
    let mmu = MMU::new();
    assert_eq!(mmu.memory[0], 0);
}

#[test]
fn byte() {
    let mut mmu = MMU::new();
    assert_eq!(mmu.byte_get(0x100), 0);

    mmu.byte_set(0x100, 10);
    assert_eq!(mmu.byte_get(0x100), 10);
}

#[test]
fn word() {
    let mut mmu = MMU::new();
    assert_eq!(mmu.word_get(0x100), 0);

    mmu.byte_set(0x100, 1);
    mmu.byte_set(0x101, 2);
    assert_eq!(mmu.word_get(0x100), 0x201);

    mmu.word_set(0x100, 0x304);
    assert_eq!(mmu.byte_get(0x100), 4);
    assert_eq!(mmu.byte_get(0x101), 3);
}
