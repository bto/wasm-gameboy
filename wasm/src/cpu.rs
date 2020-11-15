use super::registers::Registers;

struct CPU {
    registers: Registers,
}

impl CPU {
    fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }
}

#[path = "./cpu_test.rs"]
mod tests;
