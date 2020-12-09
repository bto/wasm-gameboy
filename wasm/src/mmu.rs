use super::gpu::GPU;

#[derive(Debug, PartialEq)]
pub struct MMU {
    gpu: GPU,
    memory: [u8; 0xFFFF],
}

impl MMU {
    pub fn new() -> Self {
        Self {
            gpu: GPU::new(),
            memory: [0u8; 0xFFFF],
        }
    }

    pub fn byte_get(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.gpu.byte_get(addr),
            _ => self.memory[addr as usize],
        }
    }

    pub fn byte_set(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.gpu.byte_set(addr, value),
            _ => self.memory[addr as usize] = value,
        }
    }

    pub fn word_get(&self, addr: u16) -> u16 {
        let lb = self.byte_get(addr);
        let hb = self.byte_get(addr + 1);
        (hb as u16) << 8 | lb as u16
    }

    pub fn word_set(&mut self, addr: u16, value: u16) {
        self.byte_set(addr, value as u8);
        self.byte_set(addr + 1, (value >> 8) as u8);
    }
}

#[path = "./mmu_tests.rs"]
#[cfg(test)]
mod tests;
