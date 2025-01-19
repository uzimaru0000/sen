use std::io::{stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use lib::render::{utils::frame::Frame, Renderer};

pub struct CliRenderer {
    width: usize,
    height: usize,
}

impl CliRenderer {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;

    pub fn new(width: usize, height: usize) -> Self {
        execute!(stdout(), Hide, EnterAlternateScreen, Clear(ClearType::All)).unwrap();

        Self { width, height }
    }
}

impl Renderer for CliRenderer {
    fn render(&mut self, frame: &Frame) {
        let mut stdout = stdout();

        for y in 0..self.height {
            let src_y = (y as f32 / self.height as f32) * Self::HEIGHT as f32;
            let src_y = src_y.round() as usize;

            queue!(stdout, MoveTo(0, y as u16)).unwrap();
            for x in 0..self.width {
                let src_x = (x as f32 / self.width as f32) * Self::WIDTH as f32;
                let src_x = src_x.round() as usize;

                let i = (src_y * Self::WIDTH + src_x) * 3;
                let r = frame.data[i];
                let g = frame.data[i + 1];
                let b = frame.data[i + 2];

                queue!(
                    stdout,
                    SetBackgroundColor(Color::Rgb { r, g, b }),
                    Print(" ")
                )
                .unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}

impl Drop for CliRenderer {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, LeaveAlternateScreen);
    }
}
