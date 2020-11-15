use super::memory_bus::MemoryBus;
use super::registers::Registers;

struct CPU {
    bus: MemoryBus,
    registers: Registers,
}

impl CPU {
    fn new() -> Self {
        Self {
            bus: MemoryBus::new(),
            registers: Registers::new(),
        }
    }

    fn execute(&self, byte: u8) {
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
            _ => panic!("not implemented"),
        }
    }

    fn step(&self) {
        let byte = self.bus.get_byte(self.registers.pc);
        self.execute(byte);
    }

    fn op_lda(&self) {
        panic!("not implemented")
    }
}

#[path = "./cpu_test.rs"]
mod tests;
