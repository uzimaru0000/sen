use sdl2::{
    render::{Canvas, Texture},
    video::Window,
};

use crate::ppu::PPU;

use super::{frame::Frame, Renderer};

pub struct Sdl2Renderer<'a> {
    canvas: Canvas<Window>,
    texture: Texture<'a>,
    frame: Frame,
}

impl<'a> Sdl2Renderer<'a> {
    pub fn new(canvas: Canvas<Window>, texture: Texture<'a>) -> Self {
        Self {
            canvas,
            texture,
            frame: Frame::new(),
        }
    }
}

impl<'a> Renderer for Sdl2Renderer<'a> {
    fn render(&mut self, ppu: &PPU) {
        self.frame.render(ppu);
        self.texture
            .update(None, &self.frame.data, 256 * 3)
            .unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }
}
