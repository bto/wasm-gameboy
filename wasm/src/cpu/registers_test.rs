#[allow(unused_imports)]
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
    assert_eq!(registers.pc, 0);
    assert_eq!(registers.sp, 0);
    assert_eq!(registers.carry, false);
    assert_eq!(registers.half_carry, false);
    assert_eq!(registers.subtraction, false);
    assert_eq!(registers.zero, false);
}

#[test]
fn test_f() {
    let mut registers = Registers::new();
    assert_eq!(registers.f_get(), 0);

    registers.f_set(0b1111_1111);
    assert_eq!(registers.f_get(), 0b1111_0000);
    assert_eq!(registers.carry, true);
    assert_eq!(registers.half_carry, true);
    assert_eq!(registers.subtraction, true);
    assert_eq!(registers.zero, true);

    registers.carry = false;
    assert_eq!(registers.f_get(), 0b1110_0000);

    registers.half_carry = false;
    assert_eq!(registers.f_get(), 0b1100_0000);

    registers.subtraction = false;
    assert_eq!(registers.f_get(), 0b1000_0000);

    registers.zero = false;
    assert_eq!(registers.f_get(), 0b0000_0000);
}

#[test]
fn test_af() {
    let mut registers = Registers::new();
    assert_eq!(registers.af_get(), 0);

    registers.af_set(0x01FF);
    assert_eq!(registers.af_get(), 0x01F0);
    assert_eq!(registers.a, 1);
    assert_eq!(registers.carry, true);
    assert_eq!(registers.half_carry, true);
    assert_eq!(registers.subtraction, true);
    assert_eq!(registers.zero, true);

    registers.a = 2;
    assert_eq!(registers.af_get(), 0x02F0);

    registers.carry = false;
    assert_eq!(registers.af_get(), 0x02E0);
}

#[test]
fn test_bc() {
    let mut registers = Registers::new();
    assert_eq!(registers.bc_get(), 0);

    registers.bc_set(0x0102);
    assert_eq!(registers.bc_get(), 0x0102);
    assert_eq!(registers.b, 1);
    assert_eq!(registers.c, 2);

    registers.b = 3;
    assert_eq!(registers.bc_get(), 0x0302);

    registers.c = 4;
    assert_eq!(registers.bc_get(), 0x0304);
}

#[test]
fn test_de() {
    let mut registers = Registers::new();
    assert_eq!(registers.de_get(), 0);

    registers.de_set(0x0102);
    assert_eq!(registers.de_get(), 0x0102);
    assert_eq!(registers.d, 1);
    assert_eq!(registers.e, 2);

    registers.d = 3;
    assert_eq!(registers.de_get(), 0x0302);

    registers.e = 4;
    assert_eq!(registers.de_get(), 0x0304);
}

#[test]
fn test_hl() {
    let mut registers = Registers::new();
    assert_eq!(registers.hl_get(), 0);

    registers.hl_set(0x0102);
    assert_eq!(registers.hl_get(), 0x0102);
    assert_eq!(registers.h, 1);
    assert_eq!(registers.l, 2);

    registers.h = 3;
    assert_eq!(registers.hl_get(), 0x0302);

    registers.l = 4;
    assert_eq!(registers.hl_get(), 0x0304);
}
