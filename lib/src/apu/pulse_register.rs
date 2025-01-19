use bitflags::bitflags;

bitflags! {
    struct ToneVolumeController: u8 {
        const DUTY_HI = 0b1000_0000;
        const DUTY_LO = 0b0100_0000;
        const LOOP = 0b0010_0000;
        const CONSTANT_VOLUME = 0b0001_0000;
        const VOLUME = 0b0000_1111;
    }

    struct SweepController: u8 {
        const SWEEP_ENABLE = 0b1000_0000;
        const SWEEP_TIMER = 0b0111_0000;
        const SWEEP_DIRECTION = 0b0000_1000;
        const SWEEP_FREQUENCY = 0b0000_0111;
    }

    struct LoFrequency: u8 {
        const LO_FREQUENCY = 0b1111_1111;
    }

    struct HiFrequency: u8 {
        const HI_FREQUENCY = 0b0000_0111;
        const KEY_ON = 0b1111_1000;
    }
}

enum SweepDirection {
    Increase,
    Decrease,
}

pub struct PulseRegister {
    tone_volume_controller: ToneVolumeController,
    sweep_controller: SweepController,
    lo_frequency: LoFrequency,
    hi_frequency: HiFrequency,
}

impl PulseRegister {
    pub fn new() -> Self {
        Self {
            tone_volume_controller: ToneVolumeController::empty(),
            sweep_controller: SweepController::empty(),
            lo_frequency: LoFrequency::empty(),
            hi_frequency: HiFrequency::empty(),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0 => self.tone_volume_controller = ToneVolumeController::from_bits_truncate(data),
            1 => self.sweep_controller = SweepController::from_bits_truncate(data),
            2 => self.lo_frequency = LoFrequency::from_bits_truncate(data),
            3 => self.hi_frequency = HiFrequency::from_bits_truncate(data),
            _ => panic!("Invalid address: {}", addr),
        }
    }

    pub fn get_duty(&self) -> f32 {
        match (
            self.tone_volume_controller
                .contains(ToneVolumeController::DUTY_HI),
            self.tone_volume_controller
                .contains(ToneVolumeController::DUTY_LO),
        ) {
            (false, false) => 0.125,
            (false, true) => 0.25,
            (true, false) => 0.5,
            (true, true) => 0.75,
        }
    }

    pub fn get_volume(&self) -> f32 {
        if self
            .tone_volume_controller
            .contains(ToneVolumeController::CONSTANT_VOLUME)
        {
            (self.tone_volume_controller.bits() & ToneVolumeController::VOLUME.bits()) as f32 / 15.0
        } else {
            // TODO: ちゃんと計算する
            (self.tone_volume_controller.bits() & ToneVolumeController::VOLUME.bits()) as f32 / 15.0
        }
    }

    pub fn get_sweep_enable(&self) -> bool {
        self.sweep_controller
            .contains(SweepController::SWEEP_ENABLE)
    }

    pub fn get_sweep_timer(&self) -> u8 {
        (self.sweep_controller.bits() & SweepController::SWEEP_TIMER.bits()) >> 4
    }

    pub fn get_sweep_direction(&self) -> SweepDirection {
        if self
            .sweep_controller
            .contains(SweepController::SWEEP_DIRECTION)
        {
            SweepDirection::Increase
        } else {
            SweepDirection::Decrease
        }
    }

    pub fn get_sweep_frequency(&self) -> u8 {
        self.sweep_controller.bits() & SweepController::SWEEP_FREQUENCY.bits()
    }

    pub fn get_frequency(&self) -> u16 {
        let lo = self.lo_frequency.bits() as u16;
        let hi = (self.hi_frequency.bits() & HiFrequency::HI_FREQUENCY.bits()) as u16;
        (hi << 8) | lo
    }
}
