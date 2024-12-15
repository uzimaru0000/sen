use crate::{
    palette::{Frame, SYSTEM_PALETTE},
    ppu::PPU,
};

pub fn render(ppu: &PPU, frame: &mut Frame) {
    let bank = ppu.background_pattern_addr();

    for i in 0..0x03C0 {
        let tile = ppu.get_vram()[i] as u16;
        let tile_x = i % 32;
        let tile_y = i / 32;
        let tile =
            &ppu.get_chr_rom()[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
        let palette = bg_pallette(ppu, tile_x, tile_y);

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper = upper >> 1;
                lower = lower >> 1;
                let palette_idx = palette[value as usize] as usize;
                let rgb = SYSTEM_PALETTE[palette_idx];
                frame.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb);
            }
        }
    }
}

fn bg_pallette(ppu: &PPU, tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;
    // note: still using hardcoded first nametable
    let attr_byte = ppu.get_vram()[0x03C0 + attr_table_idx];

    let pallet_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
        (0, 0) => attr_byte & 0b11,
        (1, 0) => (attr_byte >> 2) & 0b11,
        (0, 1) => (attr_byte >> 4) & 0b11,
        (1, 1) => (attr_byte >> 6) & 0b11,
        (_, _) => panic!("should not happen"),
    };

    let pallette_start = 1 + (pallet_idx as usize) * 4;
    let palette_table = ppu.get_pallette_table();
    [
        palette_table[0],
        palette_table[pallette_start],
        palette_table[pallette_start + 1],
        palette_table[pallette_start + 2],
    ]
}
