use super::*;

#[test]
fn new() {
    let gb = GameBoy::new();
    assert_eq!(gb.cpu, CPU::new());
}
