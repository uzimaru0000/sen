use std::collections::HashMap;

use once_cell::sync::Lazy;
use sdl2::{event::Event, keyboard::Keycode, EventPump};

use super::{button::JoypadButton, register::Joypad, JoypadHandler};

const KEY_MAP: Lazy<HashMap<Keycode, JoypadButton>> = Lazy::new(|| {
    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, JoypadButton::DOWN);
    key_map.insert(Keycode::Up, JoypadButton::UP);
    key_map.insert(Keycode::Right, JoypadButton::RIGHT);
    key_map.insert(Keycode::Left, JoypadButton::LEFT);
    key_map.insert(Keycode::Space, JoypadButton::SELECT);
    key_map.insert(Keycode::Return, JoypadButton::START);
    key_map.insert(Keycode::A, JoypadButton::BUTTON_A);
    key_map.insert(Keycode::S, JoypadButton::BUTTON_B);

    key_map
});

pub struct Sdl2JoypadHandler {
    event_pump: EventPump,
}

impl Sdl2JoypadHandler {
    pub fn new(event_pump: EventPump) -> Self {
        Self { event_pump }
    }
}

impl JoypadHandler for Sdl2JoypadHandler {
    fn handle(&mut self, joypad: &mut Joypad) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = KEY_MAP.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed(*key, true);
                    }
                }

                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = KEY_MAP.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed(*key, false);
                    }
                }

                _ => { /* do nothing */ }
            }
        }
    }
}
