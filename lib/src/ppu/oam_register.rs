pub struct OAMRegister {
    pub data: [u8; 256],
    pub addr: u8,
}

impl OAMRegister {
    pub fn new() -> Self {
        Self {
            data: [0; 256],
            addr: 0,
        }
    }

    pub fn write_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.data[self.addr as usize] = *x;
            self.addr = self.addr.wrapping_add(1);
        }
    }

    pub fn write(&mut self, data: u8) {
        self.data[self.addr as usize] = data;
        self.addr = self.addr.wrapping_add(1);
    }

    pub fn read(&self) -> u8 {
        self.data[self.addr as usize]
    }

    pub fn write_addr(&mut self, addr: u8) {
        self.addr = addr;
    }

    pub fn reset_addr(&mut self) {
        self.addr = 0;
    }
}
