use std::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Duration,
};

use sdl2::audio::{AudioCallback, AudioDevice};

pub struct SquareNote {
    duty: f32,
    hz: f32,
    volume: f32,
}

impl SquareNote {
    pub fn new(duty: f32, hz: f32, volume: f32) -> Self {
        Self { duty, hz, volume }
    }
}

pub struct SquareWave {
    receiver: Receiver<SquareNote>,
    phase: f32,
    note: Option<SquareNote>,
    freq: f32,
}

impl AudioCallback for SquareWave {
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
                *x = if self.phase <= note.duty {
                    note.volume
                } else {
                    -note.volume
                };
                self.phase = (self.phase + note.hz / self.freq) % 1.0;
            } else {
                *x = 0.0;
            }
        }
    }
}

pub fn create_square_wave(ctx: &sdl2::Sdl) -> (AudioDevice<SquareWave>, Sender<SquareNote>) {
    let audio_subsystem = ctx.audio().unwrap();
    let (sender, receiver) = channel::<SquareNote>();
    let spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    let device = audio_subsystem
        .open_playback(None, &spec, move |spec| SquareWave {
            receiver,
            phase: 0.0,
            note: None,
            freq: spec.freq as f32,
        })
        .unwrap();

    (device, sender)
}
