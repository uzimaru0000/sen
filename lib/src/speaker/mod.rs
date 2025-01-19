#[derive(Debug, Clone)]
pub enum NoiseMode {
    Short,
    Long,
}

impl Into<u8> for NoiseMode {
    fn into(self) -> u8 {
        match self {
            NoiseMode::Short => 6,
            NoiseMode::Long => 1,
        }
    }
}

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
