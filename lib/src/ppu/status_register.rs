use bitflags::bitflags;

bitflags! {
    pub struct StatusRegister: u8 {
        const VBLANK = 0b1000_0000;
        const SPRITE_ZERO_HIT = 0b0100_0000;
        const SPRITE_OVERFLOW = 0b0010_0000;
    }
}

impl StatusRegister {
    pub fn new() -> Self {
        Self::from_bits_truncate(0)
    }

    pub fn set_vblank_status(&mut self, status: bool) {
        self.set(StatusRegister::VBLANK, status);
    }

    pub fn set_sprite_zero_hit(&mut self, status: bool) {
        self.set(StatusRegister::SPRITE_ZERO_HIT, status);
    }

    pub fn set_sprite_overflow(&mut self, status: bool) {
        self.set(StatusRegister::SPRITE_OVERFLOW, status);
    }

    pub fn is_in_vblank(&self) -> bool {
        self.contains(StatusRegister::VBLANK)
    }
}
