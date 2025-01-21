use lib::emulator::Emulator;

use crate::{joypad::Sdl2JoypadHandler, renderer::Sdl2Renderer, speaker::SdlSpeaker};

pub struct Sdl2Emulator {
    emulator: Emulator<SdlSpeaker, Sdl2JoypadHandler, Sdl2Renderer>,
}

impl Sdl2Emulator {
    pub fn new(raw: Vec<u8>) -> Self {
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

        let speaker = SdlSpeaker::new(&sdl_context);
        let joypad_handler = Sdl2JoypadHandler::new(event_pump);
        let renderer = Sdl2Renderer::new(canvas, creator);

        let emulator = Emulator::new(raw, speaker, joypad_handler, renderer);

        Self { emulator }
    }

    pub fn step(&mut self) {
        self.emulator.step();
    }

    pub fn reset(&mut self) {
        self.emulator.reset();
    }
}
