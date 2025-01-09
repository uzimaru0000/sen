use std::{fs, path::PathBuf};

use sdl2::pixels::PixelFormatEnum;
use sen::{
    bus::NESBus, cpu::CPU, joypad::sdl2::Sdl2JoypadHandler, render::sdl2::Sdl2Renderer, rom::Rom,
    speaker::sdl::SdlSpeaker,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    run_from_file(args[1].clone().into());
}

fn run_from_file(path: PathBuf) {
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
    let texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let raw = fs::read(path).unwrap();
    let rom = Rom::new(&raw).unwrap();

    let speaker = SdlSpeaker::new(&sdl_context);
    let joypad_handler = Sdl2JoypadHandler::new(event_pump);
    let renderer = Sdl2Renderer::new(canvas, texture);
    let bus = NESBus::new(rom, speaker, joypad_handler, renderer);

    let mut cpu = CPU::new(bus);

    cpu.reset();
    cpu.run_with_callback(|cpu| {
        // println!("{}", trace(cpu));
    });
}
