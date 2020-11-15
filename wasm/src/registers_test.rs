use super::*;

#[test]
fn test_new() {
    let registers = Registers::new();
    assert_eq!(registers.a, 0);
    assert_eq!(registers.b, 0);
    assert_eq!(registers.c, 0);
    assert_eq!(registers.d, 0);
    assert_eq!(registers.e, 0);
    assert_eq!(registers.f, FlagRegister::new());
    assert_eq!(registers.h, 0);
    assert_eq!(registers.l, 0);
}

#[test]
fn test_af() {
    let mut registers = Registers::new();
    assert_eq!(registers.get_af(), 0);

    registers.set_af(0x0110);
    assert_eq!(registers.get_af(), 0x0110);
    assert_eq!(registers.a, 1);
    assert_eq!(registers.f.carry, true);

    registers.a = 3;
    assert_eq!(registers.get_af(), 0x0310);

    registers.f.subtraction = true;
    assert_eq!(registers.get_af(), 0x0350);
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
