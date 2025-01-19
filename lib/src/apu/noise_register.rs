use crate::speaker::NoiseMode;
use bitflags::bitflags;

bitflags! {
    struct VolumeControl: u8 {
        const KEY_OFF_FLAG = 0b0010_0000;
        const ENVELOPE_FLAG = 0b0001_0000;
        const VOLUME = 0b0000_1111;
    }

    struct ModeControl: u8 {
        const MODE = 0b1000_0000;
        const FREQUENCY = 0b0000_1111;
    }

    struct KeyControl: u8 {
        const KEY_OF_COUNT = 0b1111_1000;
    }
}

const NOISE_MAP: [u16; 16] = [
    0x002, 0x004, 0x008, 0x010, 0x020, 0x030, 0x040, 0x050, 0x065, 0x07F, 0x0BE, 0x0FE, 0x17D,
    0x1FC, 0x3F9, 0x7F2,
];

pub struct NoiseRegister {
    volume_control: VolumeControl,
    mode_control: ModeControl,
    key_control: KeyControl,
}

impl NoiseRegister {
    pub fn new() -> Self {
        Self {
            volume_control: VolumeControl::empty(),
            mode_control: ModeControl::empty(),
            key_control: KeyControl::empty(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0 => self.volume_control = VolumeControl::from_bits_truncate(value),
            1 => eprintln!("unused register: {}", addr),
            2 => self.mode_control = ModeControl::from_bits_truncate(value),
            3 => self.key_control = KeyControl::from_bits_truncate(value),
            _ => panic!("Invalid noise register address: {}", addr),
        }
    }

    pub fn get_mode(&self) -> NoiseMode {
        if self.mode_control.contains(ModeControl::MODE) {
            NoiseMode::Short
        } else {
            NoiseMode::Long
        }
    }

    pub fn get_frequency(&self) -> u16 {
        let idx = self.mode_control.bits() as u16 & ModeControl::FREQUENCY.bits() as u16;
        NOISE_MAP[idx as usize]
    }

    pub fn get_volume(&self) -> f32 {
        (self.volume_control.bits() & VolumeControl::VOLUME.bits()) as f32 / 15.0
    }
}
