use super::{register::Joypad, JoypadHandler};

pub struct DummyHandler;

impl JoypadHandler for DummyHandler {
    fn handle(&mut self, _: &mut Joypad) {}
}
