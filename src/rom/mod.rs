const NES_TAG: &[u8] = &[0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16 * 1024;
const CHR_ROM_PAGE_SIZE: usize = 8 * 1024;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub is_chr_ram: bool,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &[u8]) -> Result<Self, String> {
        if &raw[0..4] != NES_TAG {
            return Err("Invalid NES file".to_string());
        }

        let mapper = (raw[7] & 0xF0) | (raw[6] >> 4);

        let ines_ver = (raw[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err("NES2.0 format is not supported".to_string());
        }

        let four_screen = raw[6] & 0x08 != 0;
        let vertical_mirroring = raw[6] & 0x01 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size: usize = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer = raw[6] & 0x04 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let chr_rom = if chr_rom_size > 0 {
            raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec()
        } else {
            vec![0; CHR_ROM_PAGE_SIZE]
        };

        Ok(Self {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom,
            is_chr_ram: chr_rom_size == 0,
            mapper,
            screen_mirroring,
        })
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::*;

    const TEST_ROM_DATA: Lazy<Vec<u8>> = Lazy::new(|| {
        let mut rom = vec![
            0x4E, 0x45, 0x53, 0x1A, // NES\x1A
            0x01, // PRG-ROM サイズ (1ページ = 16KB)
            0x00, // CHR-ROM サイズ (0ページ = キャラクターデータなし)
            0x00, // フラグ1
            0x00, // フラグ2
        ];
        rom.resize(16, 0x00);

        let mut program = vec![
            0xA9, 0x01, // LDA #$01
            0x8D, 0x00, 0x02, // STA $0200
            0x4C, 0x00, 0x80, // JMP $8000
        ];
        program.resize(16 * 1024, 0x00);

        rom.extend(program);

        rom
    });

    #[test]
    fn test_rom_new() {
        let rom = Rom::new(&TEST_ROM_DATA).unwrap();
        assert_eq!(rom.prg_rom.len(), 16 * 1024);
        assert_eq!(rom.chr_rom.len(), 0);
        assert_eq!(rom.mapper, 0);
        assert_eq!(rom.screen_mirroring, Mirroring::Horizontal);
    }
}
