use bitflags::bitflags;

bitflags! {
    pub struct MaskRegister: u8 {
        const GRAYSCALE = 0b0000_0001;
        const SHOW_BACKGROUND_LEFTMOST = 0b0000_0010;
        const SHOW_SPRITES_LEFTMOST = 0b0000_0100;
        const SHOW_BACKGROUND = 0b0000_1000;
        const SHOW_SPRITES = 0b0001_0000;
        const EMPHASIZE_RED = 0b0010_0000;
        const EMPHASIZE_GREEN = 0b0100_0000;
        const EMPHASIZE_BLUE = 0b1000_0000;
    }
}

impl MaskRegister {
    pub fn new() -> Self {
        Self::from_bits_truncate(0)
    }

    pub fn update(&mut self, value: u8) {
        *self.0.bits_mut() = value;
    }

    pub fn is_show_sprites(&self) -> bool {
        self.contains(Self::SHOW_SPRITES)
    }
}
