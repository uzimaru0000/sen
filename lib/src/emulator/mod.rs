use crate::{
    bus::NESBus, cpu::CPU, joypad::JoypadHandler, render::Renderer, rom::Rom, speaker::Speaker,
};

pub struct Emulator<S, J, R>
where
    S: Speaker,
    J: JoypadHandler,
    R: Renderer,
{
    cpu: CPU<NESBus<S, J, R>>,
}

impl<S, J, R> Emulator<S, J, R>
where
    S: Speaker,
    J: JoypadHandler,
    R: Renderer,
{
    pub fn new(rom_data: Vec<u8>, speaker: S, handler: J, renderer: R) -> Self {
        let rom = Rom::new(&rom_data).unwrap();

        let bus = NESBus::new(rom, speaker, handler, renderer);
        let cpu = CPU::new(bus);

        Self { cpu }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }
}
