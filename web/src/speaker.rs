use lib::{
    speaker::{Speaker, SpeakerEvent},
    utils::noise::NoiseMode,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::utils::log;

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
enum JsNoiseMode {
    Long,
    Short,
}

impl Into<NoiseMode> for JsNoiseMode {
    fn into(self) -> NoiseMode {
        match self {
            JsNoiseMode::Long => NoiseMode::Long,
            JsNoiseMode::Short => NoiseMode::Short,
        }
    }
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export)]
enum JsSpeakerEvent {
    SquareNote {
        duty: f32,
        hz: f32,
        volume: f32,
    },
    NoiseNote {
        mode: JsNoiseMode,
        hz: f32,
        volume: f32,
    },
    TriangleNote {
        hz: f32,
    },
}

impl From<SpeakerEvent> for JsSpeakerEvent {
    fn from(value: SpeakerEvent) -> Self {
        match value {
            SpeakerEvent::SquareNote { duty, hz, volume } => {
                JsSpeakerEvent::SquareNote { duty, hz, volume }
            }
            SpeakerEvent::NoiseNote { mode, hz, volume } => JsSpeakerEvent::NoiseNote {
                mode: match mode {
                    NoiseMode::Long => JsNoiseMode::Long,
                    NoiseMode::Short => JsNoiseMode::Short,
                },
                hz,
                volume,
            },
            SpeakerEvent::TriangleNote { hz } => JsSpeakerEvent::TriangleNote { hz },
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsSpeaker;

    #[wasm_bindgen(method, js_name = send)]
    fn send(this: &JsSpeaker, ch: u8, event: &JsValue);
}

pub struct WebSpeaker {
    speaker: JsSpeaker,
}

impl WebSpeaker {
    pub fn new(speaker: JsSpeaker) -> Self {
        Self { speaker }
    }
}

impl Speaker for WebSpeaker {
    fn send(&self, ch: u8, event: SpeakerEvent) {
        let js_event: JsSpeakerEvent = event.into();
        let event_value = serde_wasm_bindgen::to_value(&js_event).unwrap();
        self.speaker.send(ch, &event_value);
    }
}
