use crate::utils::noise::NoiseMode;

pub mod sdl;
pub mod silent;

#[derive(Debug)]
pub enum SpeakerEvent {
    SquareNote {
        duty: f32,
        hz: f32,
        volume: f32,
    },
    NoiseNote {
        mode: NoiseMode,
        hz: f32,
        volume: f32,
    },
    TriangleNote {
        hz: f32,
    },
}

pub trait Speaker {
    fn send(&self, ch: u8, event: SpeakerEvent);
}
