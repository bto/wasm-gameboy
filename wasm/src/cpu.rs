use super::memory::Memory;
use super::registers::Registers;

struct CPU {
    memory: Memory,
    registers: Registers,
}

impl CPU {
    fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }
}

#[path = "./cpu_test.rs"]
mod tests;
