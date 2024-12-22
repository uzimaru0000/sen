use noise_register::NoiseRegister;
use pulse_register::PulseRegister;
use triangle_register::TriangleRegister;

use crate::speaker::{Speaker, SpeakerEvent};

mod noise_register;
mod pulse_register;
mod triangle_register;

const APU_PULSE1_REGISTERS: u16 = 0x4000;
const APU_PULSE1_REGISTERS_END: u16 = 0x4003;
const APU_PULSE2_REGISTERS: u16 = 0x4004;
const APU_PULSE2_REGISTERS_END: u16 = 0x4007;
const APU_TRIANGLE_REGISTERS: u16 = 0x4008;
const APU_TRIANGLE_REGISTERS_END: u16 = 0x400B;
const APU_NOISE_REGISTERS: u16 = 0x400C;
const APU_NOISE_REGISTERS_END: u16 = 0x400F;
const APU_DMC_REGISTERS: u16 = 0x4010;
const APU_DMC_REGISTERS_END: u16 = 0x4013;
const APU_STATUS_REGISTERS: u16 = 0x4015;
const APU_FRAME_COUNTER_REGISTERS: u16 = 0x4017;

// 1.789773 MHz
const CPU_CLOCK: f32 = 1_789_773.0;

fn calc_hz(frequency: u16) -> f32 {
    CPU_CLOCK / (16.0 * (frequency as f32 + 1.0))
}

pub struct APU<S: Speaker> {
    speaker: S,
    pulse1: PulseRegister,
    pulse2: PulseRegister,
    triangle: TriangleRegister,
    noise: NoiseRegister,
}

impl<S: Speaker> APU<S> {
    pub fn new(speaker: S) -> Self {
        Self {
            speaker,
            pulse1: PulseRegister::new(),
            pulse2: PulseRegister::new(),
            triangle: TriangleRegister::new(),
            noise: NoiseRegister::new(),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            APU_PULSE1_REGISTERS..=APU_PULSE1_REGISTERS_END => {
                self.pulse1.write(addr - APU_PULSE1_REGISTERS, data);
                self.speaker.send(
                    1,
                    SpeakerEvent::SquareNote {
                        duty: self.pulse1.get_duty(),
                        hz: calc_hz(self.pulse1.get_frequency()),
                        volume: self.pulse1.get_volume(),
                    },
                );
            }
            APU_PULSE2_REGISTERS..=APU_PULSE2_REGISTERS_END => {
                self.pulse2.write(addr - APU_PULSE2_REGISTERS, data);
                self.speaker.send(
                    2,
                    SpeakerEvent::SquareNote {
                        duty: self.pulse2.get_duty(),
                        hz: calc_hz(self.pulse2.get_frequency()),
                        volume: self.pulse2.get_volume(),
                    },
                );
            }
            APU_TRIANGLE_REGISTERS..=APU_TRIANGLE_REGISTERS_END => {
                self.triangle.write(addr - APU_TRIANGLE_REGISTERS, data);
                self.speaker.send(
                    3,
                    SpeakerEvent::TriangleNote {
                        hz: calc_hz(self.triangle.get_frequency()),
                    },
                );
            }
            APU_NOISE_REGISTERS..=APU_NOISE_REGISTERS_END => {
                self.noise.write(addr - APU_NOISE_REGISTERS, data);
                self.speaker.send(
                    4,
                    SpeakerEvent::NoiseNote {
                        mode: self.noise.get_mode(),
                        hz: calc_hz(self.noise.get_frequency()),
                        volume: self.noise.get_volume(),
                    },
                );
            }
            _ => {
                eprintln!("Not implemented: {:04X}", addr);
            }
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            APU_PULSE1_REGISTERS..=APU_PULSE1_REGISTERS_END
            | APU_PULSE2_REGISTERS..=APU_PULSE2_REGISTERS_END
            | APU_TRIANGLE_REGISTERS..=APU_TRIANGLE_REGISTERS_END
            | APU_NOISE_REGISTERS..=APU_NOISE_REGISTERS_END => 0x40,
            _ => {
                eprintln!("Not implemented: {:04X}", addr);
                0
            }
        }
    }
}
