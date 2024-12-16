pub struct ScrollRegister {
    x: u8,
    y: u8,
    latch: bool,
}

impl ScrollRegister {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            latch: false,
        }
    }

    pub fn update(&mut self, value: u8) {
        if !self.latch {
            self.x = value;
        } else {
            self.y = value;
        }
        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }

    pub fn get_scroll(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}
