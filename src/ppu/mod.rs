use addr_register::AddrRegister;
use ctrl_register::ControlRegister;
use mask_register::MaskRegister;
use oam_register::OAMRegister;
use scroll_register::ScrollRegister;
use status_register::StatusRegister;

use crate::rom::Mirroring;

mod addr_register;
mod ctrl_register;
mod mask_register;
mod oam_register;
mod scroll_register;
mod status_register;

pub struct PPU {
    chr_rom: Vec<u8>,
    palette_table: [u8; 32],
    vram: [u8; 2048],
    oam: OAMRegister,
    mirroring: Mirroring,
    ctrl: ControlRegister,
    addr: AddrRegister,
    status: StatusRegister,
    mask: MaskRegister,
    scroll: ScrollRegister,
    internal_data_buf: u8,
    scanline: u16,
    cycles: usize,
    nmi_interrupt: Option<bool>,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Self {
            chr_rom,
            palette_table: [0; 32],
            vram: [0; 2048],
            mirroring,
            oam: OAMRegister::new(),
            addr: AddrRegister::new(),
            ctrl: ControlRegister::new(),
            status: StatusRegister::new(),
            mask: MaskRegister::new(),
            scroll: ScrollRegister::new(),
            internal_data_buf: 0,
            scanline: 0,
            cycles: 0,
            nmi_interrupt: None,
        }
    }

    pub fn write_to_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status = self.ctrl.generate_vblank_nmi();

        self.ctrl.update(value);

        if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
            self.nmi_interrupt = Some(true);
        }
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask.update(value);
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.scroll.update(value);
    }

    pub fn write_to_oam_addr(&mut self, value: u8) {
        self.oam.write_addr(value);
    }

    pub fn write_to_oam_data(&mut self, value: u8) {
        self.oam.write(value);
    }

    pub fn write_to_oam_dma(&mut self, data: &[u8; 256]) {
        self.oam.write_dma(data);
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam.read()
    }

    pub fn read_status(&mut self) -> u8 {
        let status = self.status.bits();
        // self.status.set_vblank_status(false);
        self.addr.reset_latch();
        self.scroll.reset_latch();
        status
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3EFF => panic!(
                "addr space 0x3000..0x3EFF is not expected to be used, requested = {:#04X} ",
                addr
            ),
            0x3F00..=0x3FFF => self.palette_table[(addr - 0x3F00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        match addr {
            0..=0x1FFF => {
                // NOP
            }
            0x2000..=0x2FFF => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3EFF => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3F00) as usize] = value;
            }
            0x3F00..=0x3FFF => {
                self.palette_table[(addr - 0x3F00) as usize] = value;
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
        self.increment_vram_addr();
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        if self.cycles >= 341 {
            if self.is_sprite_zero_hit(self.cycles) {
                self.status.set_sprite_zero_hit(true);
            }

            self.cycles -= 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.status.set_vblank_status(true);
                self.status.set_sprite_zero_hit(false);

                if self.ctrl.generate_vblank_nmi() {
                    self.nmi_interrupt = Some(true);
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.status.set_sprite_zero_hit(false);
                self.status.set_vblank_status(false);
                self.nmi_interrupt = None;
                return true;
            }

            if self.scanline == 257 {
                self.oam.reset_addr();
            }
        }

        false
    }

    pub fn get_nmi_interrupt(&self) -> Option<bool> {
        self.nmi_interrupt
    }

    pub fn clear_nmi_interrupt(&mut self) {
        self.nmi_interrupt = None;
    }

    pub fn background_pattern_addr(&self) -> u16 {
        self.ctrl.background_pattern_addr()
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        self.ctrl.sprite_pattern_addr()
    }

    pub fn get_scanline(&self) -> u16 {
        self.scanline
    }

    pub fn get_cycles(&self) -> usize {
        self.cycles
    }

    pub fn get_oam_data(&self) -> &[u8; 256] {
        &self.oam.data
    }

    pub fn get_scroll(&self) -> (u8, u8) {
        self.scroll.get_scroll()
    }

    pub fn get_name_table(&self) -> (&[u8], &[u8]) {
        match (&self.mirroring, self.ctrl.name_table_addr()) {
            (Mirroring::Vertical, 0x2000) | (Mirroring::Vertical, 0x2800) => {
                (&self.vram[0x0000..0x0400], &self.vram[0x0400..0x0800])
            }
            (Mirroring::Vertical, 0x2400) | (Mirroring::Vertical, 0x2C00) => {
                (&self.vram[0x0400..0x0800], &self.vram[0x0000..0x0400])
            }
            (Mirroring::Horizontal, 0x2000) | (Mirroring::Horizontal, 0x2400) => {
                (&self.vram[0x0000..0x0400], &self.vram[0x0400..0x0800])
            }
            (Mirroring::Horizontal, 0x2800) | (Mirroring::Horizontal, 0x2C00) => {
                (&self.vram[0x0400..0x0800], &self.vram[0x0000..0x0400])
            }
            (_, _) => {
                panic!("Not supported mirroring type {:?}", self.mirroring);
            }
        }
    }

    pub fn get_bg_tile(&self, bank: u16, idx: usize) -> &[u8] {
        let tile = self.vram[idx] as u16;
        &self.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize]
    }

    pub fn get_sprite_tile(&self, bank: u16, idx: u16) -> &[u8] {
        &self.chr_rom[(bank + idx * 16) as usize..=(bank + idx * 16 + 15) as usize]
    }

    pub fn get_bg_palette(
        &self,
        attr_table: &[u8],
        tile_column: usize,
        tile_row: usize,
    ) -> [u8; 4] {
        let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;
        let attr_byte = attr_table[attr_table_idx];

        let pallet_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
            (0, 0) => attr_byte & 0b11,
            (1, 0) => (attr_byte >> 2) & 0b11,
            (0, 1) => (attr_byte >> 4) & 0b11,
            (1, 1) => (attr_byte >> 6) & 0b11,
            (_, _) => panic!("should not happen"),
        };

        let palette_start = 1 + (pallet_idx as usize) * 4;
        [
            self.palette_table[0],
            self.palette_table[palette_start],
            self.palette_table[palette_start + 1],
            self.palette_table[palette_start + 2],
        ]
    }

    pub fn get_sprite_palette(&self, palette_idx: u8) -> [u8; 4] {
        let palette_start = 0x11 + (palette_idx * 4) as usize;
        [
            0,
            self.palette_table[palette_start],
            self.palette_table[palette_start + 1],
            self.palette_table[palette_start + 2],
        ]
    }

    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0x2FFF;
        let vram_index = mirrored_vram - 0x2000;
        let name_table = vram_index / 0x400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) => vram_index - 0x800,
            (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn is_sprite_zero_hit(&self, cycle: usize) -> bool {
        let y = self.oam.data[0] as usize;
        let x = self.oam.data[3] as usize;

        (y == self.scanline as usize) && x <= cycle && self.mask.is_show_sprites()
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }
}

#[cfg(test)]
mod test {
    use crate::rom::Mirroring;

    use super::PPU;

    #[test]
    fn test_read_data() {
        let chr_rom = vec![0; 0x2000];
        let mut ppu = PPU::new(chr_rom, Mirroring::Horizontal);

        ppu.vram[0x0024] = 0x42;
        ppu.write_to_addr(0x20);
        ppu.write_to_addr(0x24);

        // NOTE: internal_data_buf に前回の結果が入るので、一度読み込んでおく
        ppu.read_data();
        let data = ppu.read_data();
        assert_eq!(data, 0x42);
    }
}
