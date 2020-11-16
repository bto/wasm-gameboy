#[derive(Debug, PartialEq)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    carry: bool,
    half_carry: bool,
    subtraction: bool,
    zero: bool,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
            carry: false,
            half_carry: false,
            subtraction: false,
            zero: false,
        }
    }

    pub fn bc_get(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn bc_set(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn de_get(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn de_set(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn hl_get(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn hl_set(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[path = "./registers_test.rs"]
mod tests;
