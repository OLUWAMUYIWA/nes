use crate::bus::ByteAccess;

mod regs;
mod vram;

#[derive(Debug)]
pub(crate) struct Ppu {}

impl Default for Ppu {
    fn default() -> Self {
        todo!()
    }
}
impl Ppu {
    pub(crate) fn new() -> Self {
        Self {}
    }

  
}

impl ByteAccess for Ppu {
    fn load_u8(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn store_u8(&mut self, addr: u16, v: u8) {
        todo!()
    }
}
