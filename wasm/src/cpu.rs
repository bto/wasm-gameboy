use super::memory_bus::MemoryBus;
use super::registers::Registers;

struct CPU {
    bus: MemoryBus,
    registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            bus: MemoryBus::new(),
            registers: Registers::new(),
        }
    }

    pub fn step(&mut self) {
        let byte = self.bus.get_byte(self.registers.pc);
        self.execute(byte);
    }

    fn execute(&mut self, byte: u8) {
        let bits = (
            byte & 0b00000001,
            (byte & 0b00000010) >> 1,
            (byte & 0b00000100) >> 2,
            (byte & 0b00001000) >> 3,
            (byte & 0b00010000) >> 4,
            (byte & 0b00100000) >> 5,
            (byte & 0b01000000) >> 6,
            (byte & 0b10000000) >> 7,
        );

        match bits {
            (0, 0, 1, 1, 1, 0, 1, 0) => self.op_lda(),
            (1, 0, 0, 0, 0, _, _, _) => self.op_add(byte),
            _ => panic!("not implemented"),
        }
    }

    fn get_register_8(&self, byte: u8) -> u8 {
        match byte & 0b111 {
            0b111 => self.registers.a,
            0b000 => self.registers.b,
            0b001 => self.registers.c,
            0b010 => self.registers.d,
            0b011 => self.registers.e,
            0b100 => self.registers.h,
            0b101 => self.registers.l,
            0b110 => (self.bus.get_byte(self.registers.get_hl() as usize)),
            _ => panic!("never reach"),
        }
    }

    fn op_add(&mut self, byte: u8) {
        self.registers.a += self.get_register_8(byte);
    }

    fn op_lda(&self) {
        panic!("not implemented")
    }
}

#[path = "./cpu_test.rs"]
mod tests;
