use register::Joypad;

pub mod button;
pub mod register;

pub trait JoypadHandler {
    fn handle(&mut self, joypad: &mut Joypad);
}
