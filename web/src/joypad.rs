use lib::joypad::{button::JoypadButton, JoypadHandler};
use serde::Deserialize;
use ts_rs::TS;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(TS, Deserialize)]
#[ts(export)]
struct InputState {
    a: bool,
    b: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    start: bool,
    select: bool,
}

#[wasm_bindgen]
extern "C" {
    pub type JsJoypadHandler;

    #[wasm_bindgen(method, js_name = handle)]
    fn handle(this: &JsJoypadHandler) -> JsValue;
}

pub struct WebJoypadHandler {
    handler: JsJoypadHandler,
}

impl WebJoypadHandler {
    pub fn new(handler: JsJoypadHandler) -> Self {
        Self { handler }
    }
}

impl JoypadHandler for WebJoypadHandler {
    fn handle(&mut self, joypad: &mut lib::joypad::register::Joypad) {
        let state: InputState = serde_wasm_bindgen::from_value(self.handler.handle()).unwrap();

        joypad.set_button_pressed(JoypadButton::BUTTON_A, state.a);
        joypad.set_button_pressed(JoypadButton::BUTTON_B, state.b);
        joypad.set_button_pressed(JoypadButton::UP, state.up);
        joypad.set_button_pressed(JoypadButton::DOWN, state.down);
        joypad.set_button_pressed(JoypadButton::LEFT, state.left);
        joypad.set_button_pressed(JoypadButton::RIGHT, state.right);
        joypad.set_button_pressed(JoypadButton::START, state.start);
        joypad.set_button_pressed(JoypadButton::SELECT, state.select);
    }
}
