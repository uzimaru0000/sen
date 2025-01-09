use crate::ppu::PPU;

pub mod dummy;
pub mod frame;
mod palette;
mod rect;
pub mod sdl2;

pub trait Renderer {
    fn render(&mut self, ppu: &PPU);
}
