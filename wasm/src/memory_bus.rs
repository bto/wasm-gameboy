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

    pub fn byte_get(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn byte_set(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}

#[path = "./memory_bus_test.rs"]
mod tests;
