#[derive(Debug, PartialEq)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            memory: [0u8; 0xFFFF],
        }
    }

    pub fn get_byte(&self, addr: usize) -> u8 {
        self.memory[addr]
    }

    pub fn set_byte(&mut self, addr: usize, value: u8) {
        self.memory[addr] = value;
    }
}

#[path = "./memory_bus_test.rs"]
mod tests;
