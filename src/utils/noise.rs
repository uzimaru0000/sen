#[derive(Debug, Clone)]
pub enum NoiseMode {
    Short,
    Long,
}

pub struct NoiseGenerator {
    shift_register: u16,
    bit: u8,
}

impl NoiseGenerator {
    pub fn new(mode: NoiseMode) -> Self {
        Self {
            shift_register: 1,
            bit: match mode {
                NoiseMode::Short => 6,
                NoiseMode::Long => 1,
            },
        }
    }

    pub fn next(&mut self) -> bool {
        let feedback = (self.shift_register & 0x01) ^ ((self.shift_register >> self.bit) & 0x01);
        self.shift_register = self.shift_register >> 1;
        self.shift_register = self.shift_register & 0x3FFF | feedback << 14;
        self.shift_register & 0x01 == 0
    }
}
