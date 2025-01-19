use std::{fs, path::PathBuf};

use crate::{
    bus::NESBus, cpu::CPU, joypad::sdl2::Sdl2JoypadHandler, render::sdl2::Sdl2Renderer, rom::Rom,
    speaker::sdl::SdlSpeaker,
};

use super::Emulator;

pub struct Sdl2Emulator {
    cpu: CPU<NESBus<SdlSpeaker, Sdl2JoypadHandler, Sdl2Renderer>>,
}

impl Sdl2Emulator {
    pub fn new(path: PathBuf) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("SEN", (256.0 * 5.0) as u32, (240.0 * 5.0) as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        canvas.set_scale(5.0, 5.0).unwrap();
        let creator = canvas.texture_creator();

        let raw = fs::read(path).unwrap();
        let rom = Rom::new(&raw).unwrap();

        let speaker = SdlSpeaker::new(&sdl_context);
        let joypad_handler = Sdl2JoypadHandler::new(event_pump);
        let renderer = Sdl2Renderer::new(canvas, creator);
        let bus = NESBus::new(rom, speaker, joypad_handler, renderer);

        let cpu = CPU::new(bus);

        Self { cpu }
    }
}

impl Emulator for Sdl2Emulator {
    fn reset(&mut self) {
        self.cpu.reset();
    }

    fn step(&mut self) {
        self.cpu.step();
    }
}
