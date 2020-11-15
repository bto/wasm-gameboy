#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlagRegister {
    pub carry: bool,
    pub half_carry: bool,
    pub subtraction: bool,
    pub zero: bool,
}

impl std::convert::From<FlagRegister> for u8 {
    fn from(flag: FlagRegister) -> Self {
        (flag.carry as u8) << 4
            | (flag.half_carry as u8) << 5
            | (flag.subtraction as u8) << 6
            | (flag.zero as u8) << 7
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(v: u8) -> Self {
        Self {
            carry: (v >> 4 & 1) != 0,
            half_carry: (v >> 5 & 1) != 0,
            subtraction: (v >> 6 & 1) != 0,
            zero: (v >> 7) != 0,
        }
    }
}

impl FlagRegister {
    pub fn new() -> Self {
        Self {
            carry: false,
            half_carry: false,
            subtraction: false,
            zero: false,
        }
    }
}

#[path = "./flag_register_test.rs"]
mod tests;
