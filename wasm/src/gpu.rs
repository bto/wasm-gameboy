const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

struct GPU {
    vram: [u8; VRAM_SIZE],
}

impl GPU {
    fn byte_get(&self, addr: u16) -> u8 {
        let addr = addr as usize - VRAM_BEGIN;
        self.vram[addr]
    }

    fn byte_write(&mut self, addr: u16, value: u8) {
        let addr = addr as usize - VRAM_BEGIN;
        self.vram[addr] = value;
    }
}
