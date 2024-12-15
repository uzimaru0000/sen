use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};
use sen::{
    bus::{Mem, NESBus},
    cpu::{trace::trace, CPU},
    palette::Frame,
    render::render,
    rom::Rom,
};

fn main() {
    run_alter_ego();
}

fn run_alter_ego() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SEN", (256.0 * 2.0) as u32, (240.0 * 2.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let raw = include_bytes!("../fixtures/Alter_Ego.nes");
    let rom = Rom::new(raw).unwrap();

    let mut frame = Frame::new();

    let bus = NESBus::new(rom, move |ppu| {
        render(ppu, &mut frame);
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
                _ => { /* do nothing */ }
            }
        }
    });

    let mut cpu = CPU::new(bus);

    cpu.reset();
    cpu.run_with_callback(|cpu, op| {
        println!("{} {}", op, cpu);
    });
}
