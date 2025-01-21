use lib::speaker::NoiseMode;

pub struct NoiseGenerator {
    shift_register: u16,
    bit: u8,
}

impl NoiseGenerator {
    pub fn new(mode: NoiseMode) -> Self {
        Self {
            shift_register: 1,
            bit: mode.into(),
        }
    }

    pub fn next(&mut self) -> bool {
        let feedback = (self.shift_register & 0x01) ^ ((self.shift_register >> self.bit) & 0x01);
        self.shift_register = self.shift_register >> 1;
        self.shift_register = self.shift_register & 0x3FFF | feedback << 14;
        self.shift_register & 0x01 == 0
    }
}
