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

    pub fn update(&mut self, data: u8) {
        *self.0.bits_mut() = data;
    }

    pub fn generate_vblank_nmi(&mut self) -> bool {
        self.contains(Self::GENERATE_NMI)
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            32
        } else {
            1
        }
    }

    pub fn name_table_addr(&self) -> u16 {
        match self.bits() & 0b11 {
            0b00 => 0x2000,
            0b01 => 0x2400,
            0b10 => 0x2800,
            0b11 => 0x2C00,
            _ => unreachable!(),
        }
    }

    pub fn background_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::BACKGROUND_PATTERN_ADDR) {
            0x1000
        } else {
            0x0000
        }
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            0x1000
        } else {
            0x0000
        }
    }
}
