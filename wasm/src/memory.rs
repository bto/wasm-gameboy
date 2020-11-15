#[derive(Debug, PartialEq)]
pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0u8; 0xFFFF],
        }
    }
}
