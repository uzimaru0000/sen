use register::Joypad;

pub mod button;
pub mod dummy;
pub mod register;
pub mod sdl2;

pub trait JoypadHandler {
    fn handle(&mut self, joypad: &mut Joypad);
}
