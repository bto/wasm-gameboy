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
        let opcode = self.bus.byte_get(self.registers.pc);
        self.execute(opcode);
    }

    fn db_get(&self) -> u8 {
        self.bus.byte_get(self.registers.pc + 1)
    }

    fn lb_hb_get(&self) -> u16 {
        let lb = self.bus.byte_get(self.registers.pc + 1);
        let hb = self.bus.byte_get(self.registers.pc + 2);
        (hb as u16) << 8 | lb as u16
    }

    fn execute(&mut self, opcode: u8) -> u16 {
        let bits = (
            (opcode & 0b10000000) >> 7,
            (opcode & 0b01000000) >> 6,
            (opcode & 0b00100000) >> 5,
            (opcode & 0b00010000) >> 4,
            (opcode & 0b00001000) >> 3,
            (opcode & 0b00000100) >> 2,
            (opcode & 0b00000010) >> 1,
            opcode & 0b00000001,
        );

        let pc = match bits {
            (0, 0, 0, 0, 1, 0, 0, 0) => self.op_ld_nn_sp(),
            (0, 0, 1, 0, 0, 0, 1, 0) => self.op_ld_hl_inc_a(),
            (0, 0, 1, 0, 1, 0, 1, 0) => self.op_ld_a_hl_inc(),
            (0, 0, 1, 1, 0, 0, 1, 0) => self.op_ld_hl_dec_a(),
            (0, 0, 1, 1, 1, 0, 1, 0) => self.op_ld_a_hl_dec(),
            (0, 0, _, _, 0, 0, 0, 1) => self.op_ld_rr_nn(opcode),
            (0, 0, _, _, 0, 0, 1, 0) => self.op_ld_rp_a(opcode),
            (0, 0, _, _, 1, 0, 1, 0) => self.op_ld_a_rp(opcode),
            (0, 0, _, _, _, 1, 1, 0) => self.op_ld_r_n(opcode),
            (0, 1, _, _, _, _, _, _) => self.op_ld_r_r(opcode),
            (1, 0, 0, 0, 0, _, _, _) => self.op_add(opcode),
            (1, 1, 1, 0, 0, 0, 0, 0) => self.op_ldh_n_a(),
            (1, 1, 1, 0, 0, 0, 1, 0) => self.op_ldh_c_a(),
            (1, 1, 1, 0, 1, 0, 1, 0) => self.op_ld_nn_a(),
            (1, 1, 1, 1, 0, 0, 0, 0) => self.op_ldh_a_n(),
            (1, 1, 1, 1, 0, 0, 1, 0) => self.op_ldh_a_c(),
            (1, 1, 1, 1, 1, 0, 0, 1) => self.op_ld_sp_hl(),
            (1, 1, 1, 1, 1, 0, 1, 0) => self.op_ld_a_nn(),
            (1, 1, _, _, 0, 0, 0, 1) => self.op_pop_rr(opcode),
            (1, 1, _, _, 0, 1, 0, 1) => self.op_push_rr(opcode),
            _ => panic!("not implemented"),
        };

        pc
    }

    fn op_add(&mut self, opcode: u8) -> u16 {
        self.registers.a += self.register_8_get(opcode);
        self.registers.pc + 1
    }

    fn op_ld_a_hl_dec(&mut self) -> u16 {
        let hl = self.registers.hl_get();
        self.registers.a = self.bus.byte_get(hl);
        self.registers.hl_set(hl - 1);
        self.registers.pc + 1
    }

    fn op_ld_a_hl_inc(&mut self) -> u16 {
        let hl = self.registers.hl_get();
        self.registers.a = self.bus.byte_get(hl);
        self.registers.hl_set(hl + 1);
        self.registers.pc + 1
    }

    fn op_ld_a_nn(&mut self) -> u16 {
        self.registers.a = self.bus.byte_get(self.lb_hb_get());
        self.registers.pc + 3
    }

    fn op_ld_a_rp(&mut self, opcode: u8) -> u16 {
        self.registers.a = self.bus.byte_get(self.register_16_get(opcode));
        self.registers.pc + 1
    }

    fn op_ld_hl_dec_a(&mut self) -> u16 {
        let hl = self.registers.hl_get();
        self.bus.byte_set(hl, self.registers.a);
        self.registers.hl_set(hl - 1);
        self.registers.pc + 1
    }

    fn op_ld_hl_inc_a(&mut self) -> u16 {
        let hl = self.registers.hl_get();
        self.bus.byte_set(hl, self.registers.a);
        self.registers.hl_set(hl + 1);
        self.registers.pc + 1
    }

    fn op_ld_nn_a(&mut self) -> u16 {
        self.bus.byte_set(self.lb_hb_get(), self.registers.a);
        self.registers.pc + 3
    }

    fn op_ld_nn_sp(&mut self) -> u16 {
        self.bus.word_set(self.lb_hb_get(), self.registers.sp);
        self.registers.pc + 3
    }

    fn op_ld_r_n(&mut self, opcode: u8) -> u16 {
        self.register_8_set(opcode, self.db_get());
        self.registers.pc + 2
    }

    fn op_ld_r_r(&mut self, opcode: u8) -> u16 {
        self.register_8_set(opcode, self.register_8_get(opcode));
        self.registers.pc + 1
    }

    fn op_ld_rp_a(&mut self, opcode: u8) -> u16 {
        self.register_16_set_addr(opcode, self.registers.a);
        self.registers.pc + 1
    }

    fn op_ld_rr_nn(&mut self, opcode: u8) -> u16 {
        println!("op_ld_rr_nn");
        let value = self.lb_hb_get();
        println!("{}", value);
        self.register_16_set(opcode, value);
        self.registers.pc + 3
    }

    fn op_ld_sp_hl(&mut self) -> u16 {
        self.registers.sp = self.registers.hl_get();
        self.registers.pc + 1
    }

    fn op_ldh_a_c(&mut self) -> u16 {
        let addr = 0xFF00 | self.registers.c as u16;
        self.registers.a = self.bus.byte_get(addr);
        self.registers.pc + 1
    }

    fn op_ldh_a_n(&mut self) -> u16 {
        let addr = 0xFF00 | self.db_get() as u16;
        self.registers.a = self.bus.byte_get(addr);
        self.registers.pc + 2
    }

    fn op_ldh_c_a(&mut self) -> u16 {
        let addr = 0xFF00 | self.registers.c as u16;
        self.bus.byte_set(addr, self.registers.a);
        self.registers.pc + 1
    }

    fn op_ldh_n_a(&mut self) -> u16 {
        let addr = 0xFF00 | self.db_get() as u16;
        self.bus.byte_set(addr, self.registers.a);
        self.registers.pc + 2
    }

    fn op_pop_rr(&mut self, opcode: u8) -> u16 {
        let lb = self.bus.byte_get(self.registers.sp) as u16;
        self.registers.sp += 1;
        let hb = self.bus.byte_get(self.registers.sp) as u16;
        self.registers.sp += 1;

        self.register_16_set(opcode, hb << 8 | lb);

        self.registers.pc + 1
    }

    fn op_push_rr(&mut self, opcode: u8) -> u16 {
        let value = self.register_16_get(opcode);

        self.registers.sp -= 1;
        self.bus.byte_set(self.registers.sp, (value >> 8) as u8);
        self.registers.sp -= 1;
        self.bus.byte_set(self.registers.sp, (value & 0xFF) as u8);

        self.registers.pc + 1
    }

    fn register_16_get(&self, opcode: u8) -> u16 {
        match opcode & 0b110000 {
            0b000000 => self.registers.bc_get(),
            0b010000 => self.registers.de_get(),
            0b100000 => self.registers.hl_get(),
            0b110000 => self.registers.sp as u16,
            _ => panic!("never reach"),
        }
    }

    fn register_16_set(&mut self, opcode: u8, value: u16) {
        match opcode & 0b110000 {
            0b000000 => self.registers.bc_set(value),
            0b010000 => self.registers.de_set(value),
            0b100000 => self.registers.hl_set(value),
            0b110000 => self.registers.sp = value,
            _ => panic!("never reach"),
        }
    }

    fn register_16_set_addr(&mut self, opcode: u8, value: u8) {
        match opcode & 0b110000 {
            0b000000 => self.bus.byte_set(self.registers.bc_get(), value),
            0b010000 => self.bus.byte_set(self.registers.de_get(), value),
            0b100000 => self.bus.byte_set(self.registers.hl_get(), value),
            0b110000 => self.bus.byte_set(self.registers.sp, value),
            _ => panic!("never reach"),
        }
    }

    fn register_8_get(&self, opcode: u8) -> u8 {
        match opcode & 0b111 {
            0b000 => self.registers.b,
            0b001 => self.registers.c,
            0b010 => self.registers.d,
            0b011 => self.registers.e,
            0b100 => self.registers.h,
            0b101 => self.registers.l,
            0b110 => self.bus.byte_get(self.registers.hl_get()),
            0b111 => self.registers.a,
            _ => panic!("never reach"),
        }
    }

    fn register_8_set(&mut self, opcode: u8, value: u8) {
        match opcode & 0b111000 {
            0b000000 => self.registers.b = value,
            0b001000 => self.registers.c = value,
            0b010000 => self.registers.d = value,
            0b011000 => self.registers.e = value,
            0b100000 => self.registers.h = value,
            0b101000 => self.registers.l = value,
            0b110000 => self.bus.byte_set(self.registers.hl_get(), value),
            0b111000 => self.registers.a = value,
            _ => panic!("invalid destination register"),
        }
    }
}

#[path = "./cpu_test.rs"]
mod tests;
