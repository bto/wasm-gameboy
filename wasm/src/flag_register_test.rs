use super::*;

#[test]
fn test_new() {
    let flag = FlagRegister::new();
    assert_eq!(flag.carry, false);
    assert_eq!(flag.half_carry, false);
    assert_eq!(flag.subtraction, false);
    assert_eq!(flag.zero, false);

}

#[test]
fn test_convert_flag() {
    let f = 0u8;
    let flag = FlagRegister::from(f);
    assert_eq!(flag.carry, false);
    assert_eq!(flag.half_carry, false);
    assert_eq!(flag.subtraction, false);
    assert_eq!(flag.zero, false);

    let f = 0b00010000;
    let flag = FlagRegister::from(f);
    assert_eq!(flag.carry, true);
    assert_eq!(flag.half_carry, false);
    assert_eq!(flag.subtraction, false);
    assert_eq!(flag.zero, false);

    let f = 0b00100000;
    let flag = FlagRegister::from(f);
    assert_eq!(flag.carry, false);
    assert_eq!(flag.half_carry, true);
    assert_eq!(flag.subtraction, false);
    assert_eq!(flag.zero, false);

    let f = 0b01000000;
    let flag = FlagRegister::from(f);
    assert_eq!(flag.carry, false);
    assert_eq!(flag.half_carry, false);
    assert_eq!(flag.subtraction, true);
    assert_eq!(flag.zero, false);

    let f = 0b10000000;
    let flag = FlagRegister::from(f);
    assert_eq!(flag.carry, false);
    assert_eq!(flag.half_carry, false);
    assert_eq!(flag.subtraction, false);
    assert_eq!(flag.zero, true);

    let f = 0xFFu8;
    let flag = FlagRegister::from(f);
    assert_eq!(flag.carry, true);
    assert_eq!(flag.half_carry, true);
    assert_eq!(flag.subtraction, true);
    assert_eq!(flag.zero, true);
}

#[test]
fn test_convert_u8() {
    let flag = FlagRegister::new();
    let f = u8::from(flag);
    assert_eq!(f, 0);

    let mut flag = FlagRegister::new();
    flag.carry = true;
    let f = u8::from(flag);
    assert_eq!(f, 0b00010000);

    let mut flag = FlagRegister::new();
    flag.half_carry = true;
    let f = u8::from(flag);
    assert_eq!(f, 0b00100000);

    let mut flag = FlagRegister::new();
    flag.subtraction = true;
    let f = u8::from(flag);
    assert_eq!(f, 0b01000000);

    let mut flag = FlagRegister::new();
    flag.zero = true;
    let f = u8::from(flag);
    assert_eq!(f, 0b10000000);

    let mut flag = FlagRegister::new();
    flag.carry = true;
    flag.half_carry = true;
    flag.subtraction = true;
    flag.zero = true;
    let f = u8::from(flag);
    assert_eq!(f, 0b11110000);
}
