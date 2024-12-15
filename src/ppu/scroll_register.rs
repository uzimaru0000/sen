enum ScrollDirection {
    Horizontal,
    Vertical,
}

pub struct ScrollRegister {
    fine_x: u8,
    fine_y: u8,
    x: u8,
    y: u8,
    dir: ScrollDirection,
}

impl ScrollRegister {
    pub fn new() -> Self {
        Self {
            fine_x: 0,
            fine_y: 0,
            x: 0,
            y: 0,
            dir: ScrollDirection::Horizontal,
        }
    }

    pub fn update(&mut self, value: u8) {
        match self.dir {
            ScrollDirection::Horizontal => {
                self.fine_x = value & 0x07;
                self.x = value >> 3;
            }
            ScrollDirection::Vertical => {
                self.fine_y = value & 0x07;
                self.y = value >> 3;
            }
        }

        self.dir = match self.dir {
            ScrollDirection::Horizontal => ScrollDirection::Vertical,
            ScrollDirection::Vertical => ScrollDirection::Horizontal,
        };
    }
}
