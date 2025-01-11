use std::sync::mpsc::Sender;

use noise::{Noise, NoiseNote};
use sdl2::audio::AudioDevice;
use square_wave::{SquareNote, SquareWave};
use triangle_wave::{TriangleNote, TriangleWave};

use super::{Speaker, SpeakerEvent};

mod noise;
mod square_wave;
mod triangle_wave;

pub struct SdlSpeaker {
    ch1_device: AudioDevice<SquareWave>,
    ch1_sender: Sender<SquareNote>,
    ch2_device: AudioDevice<SquareWave>,
    ch2_sender: Sender<SquareNote>,
    ch3_device: AudioDevice<TriangleWave>,
    ch3_sender: Sender<TriangleNote>,
    ch4_device: AudioDevice<Noise>,
    ch4_sender: Sender<NoiseNote>,
}

impl SdlSpeaker {
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        let (ch1_device, ch1_sender) = square_wave::create_square_wave(ctx);
        let (ch2_device, ch2_sender) = square_wave::create_square_wave(ctx);
        let (ch3_device, ch3_sender) = triangle_wave::create_triangle_wave(ctx);
        let (ch4_device, ch4_sender) = noise::create_noise(ctx);

        ch1_device.resume();
        ch2_device.resume();
        ch3_device.resume();
        ch4_device.resume();

        Self {
            ch1_device,
            ch1_sender,
            ch2_device,
            ch2_sender,
            ch3_device,
            ch3_sender,
            ch4_device,
            ch4_sender,
        }
    }
}

impl Speaker for SdlSpeaker {
    fn send(&self, ch: u8, event: SpeakerEvent) {
        match (ch, event) {
            (1, SpeakerEvent::SquareNote { duty, hz, volume }) => {
                self.ch1_sender
                    .send(SquareNote::new(duty, hz, volume))
                    .unwrap();
            }
            (2, SpeakerEvent::SquareNote { duty, hz, volume }) => {
                self.ch2_sender
                    .send(SquareNote::new(duty, hz, volume))
                    .unwrap();
            }
            (3, SpeakerEvent::TriangleNote { hz }) => {
                self.ch3_sender.send(TriangleNote::new(hz)).unwrap();
            }
            (4, SpeakerEvent::NoiseNote { mode, hz, volume }) => {
                self.ch4_sender
                    .send(NoiseNote::new(hz, volume, mode))
                    .unwrap();
            }
            _ => {}
        }
    }
}
