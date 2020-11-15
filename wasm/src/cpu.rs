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

    fn execute(&mut self, byte: u8) -> usize {
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

        let pc = match bits {
            (1, 0, 0, 0, 0, _, _, _) => self.op_add(byte),
            (0, 0, _, _, 1, 0, 1, 0) => self.op_ld_a_rp(byte),
            (0, 0, _, _, 0, 0, 1, 0) => self.op_ld_rp_a(byte),
            (0, 0, _, _, _, 1, 1, 0) => self.op_ld_r_n(byte),
            (0, 1, _, _, _, _, _, _) => self.op_ld_r_r(byte),
            _ => panic!("not implemented"),
        };

        pc
    }

    fn op_add(&mut self, byte: u8) -> usize {
        self.registers.a += self.register_8_get(byte);
        self.registers.pc + 1
    }

    fn op_ld_a_rp(&mut self, byte: u8) -> usize {
        self.registers.a = self.bus.get_byte(self.register_16_get(byte) as usize);
        self.registers.pc + 1
    }

    fn op_ld_r_n(&mut self, byte: u8) -> usize {
        self.register_8_set(byte, self.bus.get_byte(self.registers.pc + 1));
        self.registers.pc + 2
    }

    fn op_ld_r_r(&mut self, byte: u8) -> usize {
        self.register_8_set(byte, self.register_8_get(byte));
        self.registers.pc + 1
    }

    fn op_ld_rp_a(&mut self, byte: u8) -> usize {
        self.register_16_set(byte, self.registers.a);
        self.registers.pc + 1
    }

    fn register_16_get(&self, byte: u8) -> u16 {
        match byte & 0b110000 {
            0b000000 => self.registers.get_bc(),
            0b010000 => self.registers.get_de(),
            0b100000 => self.registers.get_hl(),
            0b110000 => self.registers.sp as u16,
            _ => panic!("never reach"),
        }
    }

    fn register_16_set(&mut self, byte: u8, value: u8) {
        match byte & 0b110000 {
            0b000000 => self.bus.set_byte(self.registers.get_bc() as usize, value),
            0b010000 => self.bus.set_byte(self.registers.get_de() as usize, value),
            0b100000 => self.bus.set_byte(self.registers.get_hl() as usize, value),
            0b110000 => self.bus.set_byte(self.registers.sp, value),
            _ => panic!("never reach"),
        }
    }

    fn register_8_get(&self, byte: u8) -> u8 {
        match byte & 0b111 {
            0b000 => self.registers.b,
            0b001 => self.registers.c,
            0b010 => self.registers.d,
            0b011 => self.registers.e,
            0b100 => self.registers.h,
            0b101 => self.registers.l,
            0b110 => (self.bus.get_byte(self.registers.get_hl() as usize)),
            0b111 => self.registers.a,
            _ => panic!("never reach"),
        }
    }

    fn register_8_set(&mut self, byte: u8, value: u8) {
        match byte & 0b111000 {
            0b000000 => self.registers.b = value,
            0b001000 => self.registers.c = value,
            0b010000 => self.registers.d = value,
            0b011000 => self.registers.e = value,
            0b100000 => self.registers.h = value,
            0b101000 => self.registers.l = value,
            0b110000 => self.bus.set_byte(self.registers.get_hl() as usize, value),
            0b111000 => self.registers.a = value,
            _ => panic!("invalid destination register"),
        }
    }
}

#[path = "./cpu_test.rs"]
mod tests;
