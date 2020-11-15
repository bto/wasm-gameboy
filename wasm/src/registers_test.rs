use super::*;

#[test]
fn test_new() {
    let registers = Registers::new();
    assert_eq!(registers.a, 0);
    assert_eq!(registers.b, 0);
    assert_eq!(registers.c, 0);
    assert_eq!(registers.d, 0);
    assert_eq!(registers.e, 0);
    assert_eq!(registers.h, 0);
    assert_eq!(registers.l, 0);
    assert_eq!(registers.carry, false);
    assert_eq!(registers.half_carry, false);
    assert_eq!(registers.subtraction, false);
    assert_eq!(registers.zero, false);
}

#[test]
fn test_bc() {
    let mut registers = Registers::new();
    assert_eq!(registers.get_bc(), 0);

    registers.set_bc(0x0102);
    assert_eq!(registers.get_bc(), 0x0102);
    assert_eq!(registers.b, 1);
    assert_eq!(registers.c, 2);

    registers.b = 3;
    assert_eq!(registers.get_bc(), 0x0302);

    registers.c = 4;
    assert_eq!(registers.get_bc(), 0x0304);
}

#[test]
fn test_de() {
    let mut registers = Registers::new();
    assert_eq!(registers.get_de(), 0);

    registers.set_de(0x0102);
    assert_eq!(registers.get_de(), 0x0102);
    assert_eq!(registers.d, 1);
    assert_eq!(registers.e, 2);

    registers.d = 3;
    assert_eq!(registers.get_de(), 0x0302);

    registers.e = 4;
    assert_eq!(registers.get_de(), 0x0304);
}

#[test]
fn test_hl() {
    let mut registers = Registers::new();
    assert_eq!(registers.get_hl(), 0);

    registers.set_hl(0x0102);
    assert_eq!(registers.get_hl(), 0x0102);
    assert_eq!(registers.h, 1);
    assert_eq!(registers.l, 2);

    registers.h = 3;
    assert_eq!(registers.get_hl(), 0x0302);

    registers.l = 4;
    assert_eq!(registers.get_hl(), 0x0304);
}
