use crate::rom::Rom;

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&self, addr: u16) -> u16;
    fn mem_write_u16(&mut self, addr: u16, data: u16);
}

pub struct Bus {
    cpu_vram: [u8; 0x0800],
    rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu_vram: [0; 0x0800],
            rom,
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let prog_addr = addr - 0x8000;
        let prog_addr = if self.rom.prg_rom.len() == 0x4000 && prog_addr >= 0x4000 {
            prog_addr % 0x4000
        } else {
            prog_addr
        };
        self.rom.prg_rom[prog_addr as usize]
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const ROM: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0x07FF;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0x2007;
                todo!("PPU is not supported yet")
            }
            ROM..=ROM_END => self.read_prg_rom(addr),
            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0x07FF;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0x2007;
                todo!("PPU is not supported yet")
            }
            ROM..=ROM_END => {
                panic!("Attempt to write to ROM")
            }
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let low_byte = self.mem_read(addr) as u16;
                let high_byte = self.mem_read(addr + 1) as u16;
                (high_byte << 8) | low_byte
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0x2007;
                todo!("PPU is not supported yet")
            }
            ROM..=ROM_END => {
                let low_byte = self.mem_read(addr) as u16;
                let high_byte = self.mem_read(addr + 1) as u16;
                (high_byte << 8) | low_byte
            }
            _ => {
                println!("Ignoring mem access at {}", addr);
                0
            }
        }
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let high_byte = (data >> 8) as u8;
                let low_byte = (data & 0xFF) as u8;
                self.mem_write(addr, low_byte);
                self.mem_write(addr + 1, high_byte);
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0x2007;
                todo!("PPU is not supported yet")
            }
            _ => {
                println!("Ignoring mem write-access at {}", addr);
            }
        }
    }
}
