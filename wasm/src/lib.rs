use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod cpu;
mod mmu;
mod utils;

use cpu::*;

#[wasm_bindgen]
pub struct GameBoy {
    cpu: CPU,
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl GameBoy {
    pub fn new() -> Self {
        utils::set_panic_hook();
        Self { cpu: CPU::new() }
    }

    pub fn step(&mut self) {
        self.cpu.execute();
    }
}
