use crate::ppu::PPU;

use super::Renderer;

pub struct DummyRenderer;

impl Renderer for DummyRenderer {
    fn render(&mut self, _: &PPU) {}
}
