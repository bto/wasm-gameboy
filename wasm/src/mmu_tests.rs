use super::*;

#[test]
fn new() {
    let mmu = MMU::new();
    assert_eq!(mmu.ram[0], 0);
}

#[test]
fn byte() {
    let mut mmu = MMU::new();

    mmu.byte_set(0x7FFF, 12);
    assert_eq!(mmu.byte_get(0x7FFF), 12);
    assert_eq!(mmu.ram[0x7FFF], 12);

    // GPU start
    mmu.byte_set(0x8000, 23);
    assert_eq!(mmu.byte_get(0x8000), 23);
    assert_eq!(mmu.ram[0x8000], 0);
    assert_eq!(mmu.gpu.byte_get(0x8000), 23);

    // GPU end
    mmu.byte_set(0x9FFF, 34);
    assert_eq!(mmu.byte_get(0x9FFF), 34);
    assert_eq!(mmu.ram[0x9FFF], 0);
    assert_eq!(mmu.gpu.byte_get(0x9FFF), 34);

    mmu.byte_set(0xA000, 45);
    assert_eq!(mmu.byte_get(0xA000), 45);
    assert_eq!(mmu.ram[0xA000], 45);
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
