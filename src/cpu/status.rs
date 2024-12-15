#[derive(Debug, Clone, Copy)]
pub struct ProcessorStatus {
    pub negative: bool,
    pub overflow: bool,
    pub break2_command: bool,
    pub break_command: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool,
}

impl ProcessorStatus {
    pub fn new() -> ProcessorStatus {
        ProcessorStatus {
            negative: false,
            overflow: false,
            break2_command: true,
            break_command: false,
            decimal: false,
            interrupt: false,
            zero: false,
            carry: false,
        }
    }

    pub fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }

    pub fn set_negative(&mut self, value: bool) {
        self.negative = value;
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.overflow = value;
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.interrupt = value;
    }

    pub fn set_break_command(&mut self, value: bool) {
        self.break_command = value;
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.decimal = value;
    }

    pub fn set_break2_command(&mut self, value: bool) {
        self.break2_command = value;
    }
}

impl From<u8> for ProcessorStatus {
    fn from(value: u8) -> Self {
        Self {
            negative: value & 0x80 != 0,
            overflow: value & 0x40 != 0,
            break2_command: value & 0x20 != 0,
            break_command: value & 0x10 != 0,
            decimal: value & 0x08 != 0,
            interrupt: value & 0x04 != 0,
            zero: value & 0x02 != 0,
            carry: value & 0x01 != 0,
        }
    }
}

impl Into<u8> for ProcessorStatus {
    fn into(self) -> u8 {
        let mut value: u8 = 0;
        if self.negative {
            value |= 0x80;
        }
        if self.overflow {
            value |= 0x40;
        }
        if self.break2_command {
            value |= 0x20;
        }
        if self.break_command {
            value |= 0x10;
        }
        if self.decimal {
            value |= 0x08;
        }
        if self.interrupt {
            value |= 0x04;
        }
        if self.zero {
            value |= 0x02;
        }
        if self.carry {
            value |= 0x01;
        }
        value
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_processor_status_from_u8() {
        let value: u8 = 0b1100_0010;
        let status = super::ProcessorStatus::from(value);
        assert_eq!(status.negative, true, "Negative flag is not set");
        assert_eq!(status.overflow, true, "Overflow flag is not set");
        assert_eq!(status.break2_command, false, "Break2 command flag is set");
        assert_eq!(status.break_command, false, "Break command flag is set");
        assert_eq!(status.decimal, false, "Decimal flag is set");
        assert_eq!(status.interrupt, false, "Interrupt flag is set");
        assert_eq!(status.zero, true, "Zero flag is not set");
        assert_eq!(status.carry, false, "Carry flag is set");
    }

    #[test]
    fn test_processor_status_into_u8() {
        let status = super::ProcessorStatus {
            negative: true,
            overflow: true,
            break2_command: false,
            break_command: false,
            decimal: false,
            interrupt: false,
            zero: true,
            carry: false,
        };
        let value: u8 = status.into();
        assert_eq!(value, 0b1100_0010);
    }
}
