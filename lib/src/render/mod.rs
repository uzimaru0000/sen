use crate::ppu::PPU;

pub mod utils;

pub trait Renderer {
    fn render(&mut self, frame: &PPU);
}
