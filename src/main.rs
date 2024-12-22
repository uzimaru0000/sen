use std::{collections::HashMap, fs, path::PathBuf};

use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use sen::{
    bus::NESBus,
    cpu::{trace::trace, CPU},
    joypad::button::JoypadButton,
    render::frame::Frame,
    rom::Rom,
    speaker::{sdl::SdlSpeaker, silent::SilentSpeaker},
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
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(5.0, 5.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let raw = fs::read(path).unwrap();
    let rom = Rom::new(&raw).unwrap();

    let speaker = SdlSpeaker::new(&sdl_context);

    let mut frame = Frame::new();

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, JoypadButton::DOWN);
    key_map.insert(Keycode::Up, JoypadButton::UP);
    key_map.insert(Keycode::Right, JoypadButton::RIGHT);
    key_map.insert(Keycode::Left, JoypadButton::LEFT);
    key_map.insert(Keycode::Space, JoypadButton::SELECT);
    key_map.insert(Keycode::Return, JoypadButton::START);
    key_map.insert(Keycode::A, JoypadButton::BUTTON_A);
    key_map.insert(Keycode::S, JoypadButton::BUTTON_B);

    let bus = NESBus::new(rom, speaker, move |ppu, joypad| {
        frame.render(ppu);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed(*key, true);
                    }
                }

                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed(*key, false);
                    }
                }

                _ => { /* do nothing */ }
            }
        }
    });

    let mut cpu = CPU::new(bus);

    cpu.reset();
    cpu.run_with_callback(|cpu, _| {
        // println!("{}", trace(cpu));
    });
}
