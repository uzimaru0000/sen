use crate::utils::noise::NoiseGenerator;

use lib::speaker::NoiseMode;
use sdl2::audio::AudioCallback;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Duration,
};

pub struct NoiseNote {
    hz: f32,
    volume: f32,
    mode: NoiseMode,
}

impl NoiseNote {
    pub fn new(hz: f32, volume: f32, mode: NoiseMode) -> Self {
        Self { hz, volume, mode }
    }
}

pub struct Noise {
    receiver: Receiver<NoiseNote>,
    freq: f32,
    phase: f32,
    value: bool,
    note: Option<NoiseNote>,
    generator: NoiseGenerator,
}

impl AudioCallback for Noise {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            match self.receiver.recv_timeout(Duration::from_millis(0)) {
                Ok(note) => {
                    self.generator = NoiseGenerator::new(note.mode.clone());
                    self.note = Some(note);
                }
                Err(_) => {}
            }

            if let Some(note) = &mut self.note {
                *x = if self.value { 0.0 } else { note.volume };

                let current_phase = self.phase;
                self.phase = (self.phase + note.hz / self.freq) % 1.0;
                if current_phase > self.phase {
                    self.value = self.generator.next();
                }
            } else {
                *x = 0.0;
            }
        }
    }
}

pub fn create_noise(ctx: &sdl2::Sdl) -> (sdl2::audio::AudioDevice<Noise>, Sender<NoiseNote>) {
    let audio_subsystem = ctx.audio().unwrap();
    let spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let (sender, receiver) = channel();

    let device = audio_subsystem
        .open_playback(None, &spec, |spec| Noise {
            receiver,
            freq: spec.freq as f32,
            phase: 0.0,
            value: false,
            note: None,
            generator: NoiseGenerator::new(NoiseMode::Long),
        })
        .unwrap();

    (device, sender)
}
