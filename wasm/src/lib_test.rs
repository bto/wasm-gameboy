use super::*;

#[test]
fn test_new() {
    let registers = Registers::new();
    assert_eq!(registers.a, 0);
}
