use std::{
    sync::mpsc::{Receiver, Sender},
    time::Duration,
};

use sdl2::audio::AudioCallback;

pub struct TriangleNote {
    hz: f32,
}

impl TriangleNote {
    pub fn new(hz: f32) -> Self {
        Self { hz }
    }
}

pub struct TriangleWave {
    receiver: Receiver<TriangleNote>,
    phase: f32,
    note: Option<TriangleNote>,
    freq: f32,
}

impl AudioCallback for TriangleWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            match self.receiver.recv_timeout(Duration::from_millis(0)) {
                Ok(note) => {
                    self.note = Some(note);
                }
                Err(_) => {}
            }

            if let Some(note) = &self.note {
                let base = if self.phase <= 0.5 {
                    self.phase
                } else {
                    1.0 - self.phase
                };

                *x = base * 4.0 - 1.0;
                self.phase = (self.phase + note.hz / self.freq) % 1.0;
            } else {
                *x = 0.0;
            }
        }
    }
}

pub fn create_triangle_wave(
    ctx: &sdl2::Sdl,
) -> (sdl2::audio::AudioDevice<TriangleWave>, Sender<TriangleNote>) {
    let audio_subsystem = ctx.audio().unwrap();
    let (sender, receiver) = std::sync::mpsc::channel::<TriangleNote>();
    let spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let device = audio_subsystem
        .open_playback(None, &spec, |spec| TriangleWave {
            receiver,
            phase: 0.0,
            note: None,
            freq: spec.freq as f32,
        })
        .unwrap();

    (device, sender)
}
