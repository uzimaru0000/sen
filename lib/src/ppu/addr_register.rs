pub struct AddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        Self {
            value: (0, 0),
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xFF) as u8;
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get() > 0x3FFF {
            self.set(self.get() & 0x3FFF);
        }
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }

        if self.get() > 0x3FFF {
            self.set(self.get() & 0x3FFF);
        }
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }
}

#[cfg(test)]
mod test {
    use super::AddrRegister;

    #[test]
    fn test_update_addr_register() {
        let mut addr_reg = AddrRegister::new();

        addr_reg.update(0x12);
        addr_reg.update(0x34);

        assert_eq!(addr_reg.value, (0x12, 0x34));
    }

    #[test]
    fn test_increment_addr_register() {
        let mut addr_reg = AddrRegister::new();

        addr_reg.set(0x12FF);
        addr_reg.increment(1);

        assert_eq!(addr_reg.get(), 0x1300);
    }
}
