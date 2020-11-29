#[derive(Debug, PartialEq)]
pub struct MMU {
    memory: [u8; 0xFFFF],
}

impl MMU {
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

    pub fn word_get(&self, addr: u16) -> u16 {
        let lb = self.memory[addr as usize];
        let hb = self.memory[(addr + 1) as usize];
        (hb as u16) << 8 | lb as u16
    }

    pub fn word_set(&mut self, addr: u16, value: u16) {
        self.memory[addr as usize] = (value & 0xFF) as u8;
        self.memory[(addr + 1) as usize] = ((value & 0xFF00) >> 8) as u8;
    }
}

#[path = "./mmu_tests.rs"]
#[cfg(test)]
mod tests;
