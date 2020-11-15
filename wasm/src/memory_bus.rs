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
}
