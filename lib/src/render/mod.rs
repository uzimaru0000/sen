use utils::frame::Frame;

pub mod utils;

pub trait Renderer {
    fn render(&mut self, frame: &Frame);
}
