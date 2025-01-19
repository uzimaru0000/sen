use crate::{
    apu::APU,
    joypad::{register::Joypad, JoypadHandler},
    ppu::PPU,
    render::{utils::frame::Frame, Renderer},
    rom::Rom,
    speaker::Speaker,
};

pub(crate) trait Mem {
    fn mem_read(&mut self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_read_u16(&mut self, addr: u16) -> u16;
    fn mem_write_u16(&mut self, addr: u16, data: u16);
}

pub(crate) trait Bus {
    fn tick(&mut self, cycles: u8);
    fn poll_nmi_status(&mut self) -> Option<bool>;
    fn get_cycles(&self) -> (usize, usize);
    fn get_scanline(&self) -> u16;
}

pub struct NESBus<S, J, R>
where
    S: Speaker,
    J: JoypadHandler,
    R: Renderer,
{
    cpu_vram: [u8; 0x0800],
    prg_rom: Vec<u8>,
    ppu: PPU,
    apu: APU<S>,
    joypad: Joypad,
    cycles: usize,
    joypad_handler: J,
    renderer: R,
}

impl<S, J, R> NESBus<S, J, R>
where
    S: Speaker,
    J: JoypadHandler,
    R: Renderer,
{
    pub(crate) fn new(rom: Rom, speaker: S, joypad_handler: J, renderer: R) -> Self {
        let ppu = PPU::new(rom.chr_rom, rom.is_chr_ram, rom.screen_mirroring);
        let apu = APU::new(speaker);
        let joypad = Joypad::new();

        Self {
            cpu_vram: [0; 0x0800],
            prg_rom: rom.prg_rom,
            ppu,
            apu,
            joypad,
            joypad_handler,
            renderer,
            cycles: 0,
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

const APU: u16 = 0x4000;
const APU_END: u16 = 0x4017;

const JOYPAD1_READ_REGISTERS: u16 = 0x4016;
const JOYPAD2_READ_REGISTERS: u16 = 0x4017;

const ROM: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

impl<S, J, R> Mem for NESBus<S, J, R>
where
    S: Speaker,
    J: JoypadHandler,
    R: Renderer,
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
                eprintln!("Attempt to read from write-only PPU address {:#04X}", addr);
                0
            }
            PPU_STATUS_REGISTERS => self.ppu.read_status(),
            PPU_OAM_DATA_REGISTERS => self.ppu.read_oam_data(),
            PPU_DATA_REGISTERS => self.ppu.read_data(),
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0x2007;
                self.mem_read(mirror_down_addr)
            }
            JOYPAD1_READ_REGISTERS => self.joypad.read(),
            JOYPAD2_READ_REGISTERS => {
                todo!("Ignoring read from joypad 2");
            }
            APU..=APU_END => self.apu.read(addr),
            ROM..=ROM_END => self.read_prg_rom(addr),
            _ => {
                eprintln!("Ignoring mem access at {:#04X}", addr);
                0xFF
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
                eprintln!("Ignoring write to PPU status registers");
            }
            PPU_OAM_ADDRESS_REGISTERS => {
                self.ppu.write_to_oam_addr(data);
            }
            PPU_OAM_DATA_REGISTERS => {
                self.ppu.write_to_oam_data(data);
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
            PPU_OAM_DAM_REGISTERS => {
                let addr = (data as u16) << 8;
                let mut data = [0; 256];
                for i in 0..256 {
                    data[i] = self.mem_read(addr + i as u16);
                }

                self.ppu.write_to_oam_dma(&data);
            }
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0x2007;
                self.mem_write(mirror_down_addr, data);
            }
            ROM..=ROM_END => {
                eprintln!("Ignoring write to ROM at {:#04X}", addr);
            }
            JOYPAD1_READ_REGISTERS => {
                self.joypad.write(data);
            }
            APU..=APU_END => {
                self.apu.write(addr, data);
            }
            _ => {
                println!("Ignoring mem write-access at {:#04X}", addr);
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

impl<S, J, R> Bus for NESBus<S, J, R>
where
    S: Speaker,
    J: JoypadHandler,
    R: Renderer,
{
    fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;

        let nmi_before = self.ppu.get_nmi_interrupt().is_some();
        self.ppu.tick(cycles * 3);
        let nmi_after = self.ppu.get_nmi_interrupt().is_some();

        if !nmi_before && nmi_after {
            let mut frame = Frame::new();
            frame.render(&self.ppu);
            self.renderer.render(&frame);
            self.joypad_handler.handle(&mut self.joypad);
        }
    }

    fn poll_nmi_status(&mut self) -> Option<bool> {
        self.ppu.poll_nmi_interrupt()
    }

    fn get_cycles(&self) -> (usize, usize) {
        (self.cycles, self.ppu.get_cycles())
    }

    fn get_scanline(&self) -> u16 {
        self.ppu.get_scanline()
    }
}
