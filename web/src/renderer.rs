use lib::render::{utils::frame::Frame, Renderer};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    pub type JsRenderer;

    #[wasm_bindgen(method, js_name = render)]
    fn render(this: &JsRenderer, frame: &JsValue);
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
struct RenderBuffer {
    data: Vec<u8>,
}

pub struct WebRenderer {
    renderer: JsRenderer,
}

impl WebRenderer {
    pub fn new(renderer: JsRenderer) -> Self {
        Self { renderer }
    }
}

impl Renderer for WebRenderer {
    fn render(&mut self, frame: &Frame) {
        let buf = RenderBuffer {
            data: frame.data.clone(),
        };
        let frame_value = serde_wasm_bindgen::to_value(&buf).unwrap();
        self.renderer.render(&frame_value);
    }
}
