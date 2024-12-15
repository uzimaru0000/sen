use bitflags::bitflags;

bitflags! {
    pub struct ControlRegister: u8 {
        const NAME_TABLE1 = 0b00000001;
        const NAME_TABLE2 = 0b00000010;
        const VRAM_ADD_INCREMENT = 0b00000100;
        const SPRITE_PATTERN_ADDR = 0b00001000;
        const BACKGROUND_PATTERN_ADDR = 0b00010000;
        const SPRITE_SIZE = 0b00100000;
        const MASTER_SLAVE_SELECT = 0b01000000;
        const GENERATE_NMI = 0b10000000;
    }
}

impl ControlRegister {
    pub fn new() -> Self {
        Self::from_bits_truncate(0b0000_0000)
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            32
        } else {
            1
        }
    }

    pub fn update(&mut self, data: u8) {
        *self.0.bits_mut() = data;
    }

    pub fn generate_vblank_nmi(&mut self) -> bool {
        self.insert(Self::GENERATE_NMI);
        self.contains(Self::GENERATE_NMI)
    }

    pub fn background_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::BACKGROUND_PATTERN_ADDR) {
            1
        } else {
            0
        }
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            1
        } else {
            0
        }
    }
}
