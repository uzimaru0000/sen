use wasm_bindgen::prelude::wasm_bindgen;

use lib::{bus::NESBus, cpu::CPU, rom::Rom};

use crate::{
    joypad::{JsJoypadHandler, WebJoypadHandler},
    renderer::{JsRenderer, WebRenderer},
    speaker::{JsSpeaker, WebSpeaker},
};

#[wasm_bindgen]
pub struct WebEmulator {
    cpu: CPU<NESBus<WebSpeaker, WebJoypadHandler, WebRenderer>>,
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
        let rom = Rom::new(&rom_data).unwrap();

        let bus = NESBus::new(
            rom,
            WebSpeaker::new(speaker),
            WebJoypadHandler::new(handler),
            WebRenderer::new(renderer),
        );
        let cpu = CPU::new(bus);

        Self { cpu }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        self.cpu.step();
    }
}
