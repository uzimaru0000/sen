use crate::{ppu::PPU, rom::Rom};

pub trait Mem {
    fn mem_read(&mut self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&mut self, addr: u16) -> u16;
    fn mem_write_u16(&mut self, addr: u16, data: u16);
}

pub trait Bus {
    fn tick(&mut self, cycles: u8);
    fn poll_nmi_status(&mut self) -> Option<bool>;
    fn get_cycles(&self) -> (usize, usize);
    fn get_scanline(&self) -> u16;
}

pub struct NESBus<F>
where
    F: FnMut(&PPU),
{
    cpu_vram: [u8; 0x0800],
    prg_rom: Vec<u8>,
    ppu: PPU,
    cycles: usize,
    gameloop_callback: F,
}

impl<F> NESBus<F>
where
    F: FnMut(&PPU),
{
    pub fn new(rom: Rom, gameloop_callback: F) -> Self {
        let ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);

        Self {
            cpu_vram: [0; 0x0800],
            prg_rom: rom.prg_rom,
            ppu,
            cycles: 0,
            gameloop_callback,
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let prog_addr = addr - 0x8000;
        let prog_addr = if self.prg_rom.len() == 0x4000 && prog_addr >= 0x4000 {
            prog_addr % 0x4000
        } else {
            prog_addr
        };
        self.prg_rom[prog_addr as usize]
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;

const PPU_CONTROL_REGISTERS: u16 = 0x2000;
const PPU_MASK_REGISTERS: u16 = 0x2001;
const PPU_STATUS_REGISTERS: u16 = 0x2002;
const PPU_OAM_ADDRESS_REGISTERS: u16 = 0x2003;
const PPU_OAM_DATA_REGISTERS: u16 = 0x2004;
const PPU_SCROLL_REGISTERS: u16 = 0x2005;
const PPU_ADDRESS_REGISTERS: u16 = 0x2006;
const PPU_DATA_REGISTERS: u16 = 0x2007;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;
const PPU_OAM_DAM_REGISTERS: u16 = 0x4014;

const APU_REGISTERS: u16 = 0x4000;
const APU_REGISTERS_MIRRORS_END: u16 = 0x4017;

const ROM: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

impl<F> Mem for NESBus<F>
where
    F: FnMut(&PPU),
{
    fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0x07FF;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_CONTROL_REGISTERS
            | PPU_MASK_REGISTERS
            | PPU_OAM_ADDRESS_REGISTERS
            | PPU_SCROLL_REGISTERS
            | PPU_ADDRESS_REGISTERS
            | PPU_OAM_DAM_REGISTERS => {
                panic!("Attempt to read from write-only PPU address 0x{:X}", addr);
            }
            PPU_STATUS_REGISTERS => self.ppu.read_to_status(),
            PPU_OAM_DATA_REGISTERS => {
                todo!("PPU OAM data registers are not supported yet")
            }
            PPU_DATA_REGISTERS => self.ppu.read_to_data(),
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0x2007;
                self.mem_read(mirror_down_addr)
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
            PPU_CONTROL_REGISTERS => {
                self.ppu.write_to_ctrl(data);
            }
            PPU_MASK_REGISTERS => {
                self.ppu.write_to_mask(data);
            }
            PPU_STATUS_REGISTERS => {
                panic!("Attempt to write to read-only PPU address 0x{:0X}", addr);
            }
            PPU_OAM_DATA_REGISTERS => {
                todo!("PPU OAM data registers are not supported yet")
            }
            PPU_SCROLL_REGISTERS => {
                self.ppu.write_to_scroll(data);
            }
            PPU_ADDRESS_REGISTERS => {
                self.ppu.write_to_addr(data);
            }
            PPU_DATA_REGISTERS => {
                self.ppu.write_to_data(data);
            }
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0x2007;
                self.mem_write(mirror_down_addr, data);
            }
            ROM..=ROM_END => {
                panic!("Attempt to write to ROM")
            }
            APU_REGISTERS..=APU_REGISTERS_MIRRORS_END => {
                println!("APU registers are not supported yet")
            }
            _ => {
                println!("Ignoring mem write-access at 0x{:0X}", addr);
            }
        }
    }

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        let low_byte = self.mem_read(addr) as u16;
        let high_byte = self.mem_read(addr + 1) as u16;
        (high_byte << 8) | low_byte
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let high_byte = (data >> 8) as u8;
        let low_byte = (data & 0xFF) as u8;
        self.mem_write(addr, low_byte);
        self.mem_write(addr + 1, high_byte);
    }
}

impl<F> Bus for NESBus<F>
where
    F: FnMut(&PPU),
{
    fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;

        let nmi_before = self.ppu.get_nmi_interrupt().is_some();
        self.ppu.tick(cycles * 3);
        let nmi_after = self.ppu.get_nmi_interrupt().is_some();

        if !nmi_before && nmi_after {
            (self.gameloop_callback)(&self.ppu);
        }
    }

    fn poll_nmi_status(&mut self) -> Option<bool> {
        let nmi = self.ppu.get_nmi_interrupt();
        self.ppu.clear_nmi_interrupt();

        nmi
    }

    fn get_cycles(&self) -> (usize, usize) {
        (self.cycles, self.ppu.get_cycles())
    }

    fn get_scanline(&self) -> u16 {
        self.ppu.get_scanline()
    }
}
