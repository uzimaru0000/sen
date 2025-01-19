use wasm_bindgen::prelude::wasm_bindgen;

use lib::emulator::Emulator;

use crate::{
    joypad::{JsJoypadHandler, WebJoypadHandler},
    renderer::{JsRenderer, WebRenderer},
    speaker::{JsSpeaker, WebSpeaker},
};

#[wasm_bindgen]
pub struct WebEmulator {
    emulator: Emulator<WebSpeaker, WebJoypadHandler, WebRenderer>,
}

#[wasm_bindgen]
impl WebEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new(
        rom_data: Vec<u8>,
        speaker: JsSpeaker,
        handler: JsJoypadHandler,
        renderer: JsRenderer,
    ) -> Self {
        Self {
            emulator: Emulator::new(
                rom_data,
                WebSpeaker::new(speaker),
                WebJoypadHandler::new(handler),
                WebRenderer::new(renderer),
            ),
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.emulator.reset();
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        self.emulator.step();
    }
}
