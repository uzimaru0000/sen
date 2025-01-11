use bitflags::bitflags;

bitflags! {
    struct ToneControl: u8 {
        const LENGTH_COUNTER_HALT = 0b1000_0000;
        const LENGTH = 0b0111_1111;
    }
    struct LoFrequency: u8 {
        const LO_FREQUENCY = 0b1111_1111;
    }
    struct HiFrequency: u8 {
        const HI_FREQUENCY = 0b0000_0111;
        const KEY_ON = 0b1111_1000;
    }
}

pub struct TriangleRegister {
    tone_control: ToneControl,
    lo_frequency: LoFrequency,
    hi_frequency: HiFrequency,
}

impl TriangleRegister {
    pub fn new() -> Self {
        Self {
            tone_control: ToneControl::empty(),
            lo_frequency: LoFrequency::empty(),
            hi_frequency: HiFrequency::empty(),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0 => self.tone_control = ToneControl::from_bits_truncate(data),
            1 => eprintln!("unused register: {}", addr),
            2 => self.lo_frequency = LoFrequency::from_bits_truncate(data),
            3 => self.hi_frequency = HiFrequency::from_bits_truncate(data),
            _ => eprintln!("Not implemented: {:04X}", addr),
        }
    }

    pub fn get_frequency(&self) -> u16 {
        (self.hi_frequency.bits() as u16) << 8 | self.lo_frequency.bits() as u16
    }
}
