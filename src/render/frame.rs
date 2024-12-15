use crate::ppu::PPU;

use super::palette::SYSTEM_PALETTE;

pub struct Frame {
    pub data: Vec<u8>,
}

impl Frame {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;

    pub fn new() -> Self {
        Self {
            data: vec![0; Self::WIDTH * Self::HEIGHT * 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let base = y * 3 * Self::WIDTH + x * 3;
        if base + 2 < self.data.len() {
            self.data[base] = rgb.0;
            self.data[base + 1] = rgb.1;
            self.data[base + 2] = rgb.2;
        }
    }

    pub fn render(&mut self, ppu: &PPU) {
        self.render_bg(ppu);
        self.render_sprite(ppu);
    }

    fn render_bg(&mut self, ppu: &PPU) {
        let bank = ppu.background_pattern_addr();

        for i in 0..0x03C0 {
            let tile = ppu.get_bg_tile(bank, i);
            let tile_x = i % 32;
            let tile_y = i / 32;
            let palette = ppu.get_bg_palette(tile_x, tile_y);

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];

                for x in (0..=7).rev() {
                    let value = (1 & upper) << 1 | (1 & lower);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let palette_idx = palette[value as usize] as usize;
                    let rgb = SYSTEM_PALETTE[palette_idx];
                    self.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb);
                }
            }
        }
    }

    fn render_sprite(&mut self, ppu: &PPU) {
        let oam_data = ppu.get_oam_data();

        for i in (0..oam_data.len()).step_by(4).rev() {
            let idx = oam_data[i + 1] as u16;
            let tile_x = oam_data[i + 3] as usize;
            let tile_y = oam_data[i] as usize;
            let attr = oam_data[i + 2];

            let flip_vertical = attr & 0x80 != 0;
            let flip_horizontal = attr & 0x40 != 0;
            let palette_idx = attr & 0b11;
            let sprite_palette = ppu.get_sprite_palette(palette_idx);

            let bank = ppu.sprite_pattern_addr();
            let tile = ppu.get_sprite_tile(bank, idx);

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];

                for x in (0..=7).rev() {
                    let value = (1 & lower) << 1 | (1 & upper);
                    upper = upper >> 1;
                    lower = lower >> 1;

                    if value == 0 {
                        continue;
                    }

                    let palette_idx = sprite_palette[value as usize] as usize;
                    let rgb = SYSTEM_PALETTE[palette_idx];
                    match (flip_horizontal, flip_vertical) {
                        (false, false) => self.set_pixel(tile_x + x, tile_y + y, rgb),
                        (true, false) => self.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                        (false, true) => self.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                        (true, true) => self.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                    }
                }
            }
        }
    }
}
