use std::{
    collections::HashSet,
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crossterm::{
    event::{self, EventStream, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use lib::joypad::{button::JoypadButton, register::Joypad, JoypadHandler};

#[derive(Default, Clone)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

pub struct CliJoypadHandler {
    inputs: Arc<Mutex<InputState>>,
    _handle: JoinHandle<()>,
}

impl CliJoypadHandler {
    pub fn new(running: Arc<AtomicBool>) -> Self {
        let inputs = Arc::new(Mutex::new(InputState::default()));
        let handle = handle_event(running.clone(), inputs.clone());
        enable_raw_mode().unwrap();

        Self {
            inputs,
            _handle: handle,
        }
    }
}

impl JoypadHandler for CliJoypadHandler {
    fn handle(&mut self, joypad: &mut Joypad) {
        let inputs = self.inputs.lock().unwrap();

        joypad.set_button_pressed(JoypadButton::UP, inputs.up);
        joypad.set_button_pressed(JoypadButton::DOWN, inputs.down);
        joypad.set_button_pressed(JoypadButton::LEFT, inputs.left);
        joypad.set_button_pressed(JoypadButton::RIGHT, inputs.right);
        joypad.set_button_pressed(JoypadButton::BUTTON_A, inputs.a);
        joypad.set_button_pressed(JoypadButton::BUTTON_B, inputs.b);
        joypad.set_button_pressed(JoypadButton::SELECT, inputs.select);
        joypad.set_button_pressed(JoypadButton::START, inputs.start);
    }
}

impl Drop for CliJoypadHandler {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

fn handle_event(running: Arc<AtomicBool>, inputs: Arc<Mutex<InputState>>) -> JoinHandle<()> {
    let r = running.clone();
    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            let mut inputs = inputs.lock().unwrap();
            if let event::Event::Key(event) = event::read().unwrap() {
                let state = event.kind == event::KeyEventKind::Press
                    || event.kind == event::KeyEventKind::Repeat;

                match event.code {
                    KeyCode::Up => inputs.up = state,
                    KeyCode::Down => inputs.down = state,
                    KeyCode::Left => inputs.left = state,
                    KeyCode::Right => inputs.right = state,
                    KeyCode::Char('a') => inputs.a = state,
                    KeyCode::Char('s') => inputs.b = state,
                    KeyCode::Char('z') => inputs.start = state,
                    KeyCode::Char('x') => inputs.select = state,
                    KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                        r.store(false, Ordering::SeqCst);
                        break;
                    }
                    KeyCode::Esc => {
                        r.store(false, Ordering::SeqCst);
                        break;
                    }
                    _ => {}
                }
            }
        } else {
            let mut inputs = inputs.lock().unwrap();
            inputs.up = false;
            inputs.down = false;
            inputs.left = false;
            inputs.right = false;
            inputs.a = false;
            inputs.b = false;
            inputs.start = false;
            inputs.select = false;
        }
    })
}
