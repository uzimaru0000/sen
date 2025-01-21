use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use clap::Parser;
use crossterm::terminal::size;
use lib::emulator::Emulator;

use crate::{joypad::CliJoypadHandler, renderer::CliRenderer, speaker::CliSpeaker};

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

        let running = Arc::new(AtomicBool::new(true));
        let (width, height) = size().map_err(Error::Io)?;
        let mut emulator = Emulator::new(
            rom_data,
            CliSpeaker,
            CliJoypadHandler::new(running.clone()),
            CliRenderer::new(width as usize, (height - 2) as usize),
        );

        {
            let r = running.clone();
            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })
            .expect("Error setting Ctrl+C handler");
        }

        emulator.reset();
        while running.load(Ordering::SeqCst) {
            emulator.step();
        }

        Ok(())
    }
}
