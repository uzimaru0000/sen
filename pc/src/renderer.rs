use lib::render::{utils::frame::Frame, Renderer};
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

pub struct Sdl2Renderer {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Sdl2Renderer {
    pub fn new(canvas: Canvas<Window>, texture_creator: TextureCreator<WindowContext>) -> Self {
        Self {
            canvas,
            texture_creator,
        }
    }
}

impl Renderer for Sdl2Renderer {
    fn render(&mut self, frame: &Frame) {
        let mut texture = self
            .texture_creator
            .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
            .unwrap();
        texture.update(None, &frame.data, 256 * 3).unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}
