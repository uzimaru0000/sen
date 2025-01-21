use std::io;

use crate::emulator::Sdl2Emulator;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct App {
    path: String,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FailedJoin,
}

impl App {
    pub fn run(&self) -> Result<(), Error> {
        let rom_data = std::fs::read(&self.path).map_err(Error::Io)?;

        let mut emulator = Sdl2Emulator::new(rom_data);

        emulator.reset();
        loop {
            emulator.step();
        }
    }
}
