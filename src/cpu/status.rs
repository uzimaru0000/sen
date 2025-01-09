use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct ProcessorStatus: u8 {
        const NEGATIVE = 0b1000_0000;
        const OVERFLOW = 0b0100_0000;
        const BREAK2_COMMAND = 0b0010_0000;
        const BREAK_COMMAND = 0b0001_0000;
        const DECIMAL = 0b0000_1000;
        const INTERRUPT = 0b0000_0100;
        const ZERO = 0b0000_0010;
        const CARRY = 0b0000_0001;
    }
}

impl ProcessorStatus {
    pub fn new() -> Self {
        ProcessorStatus::from_bits_truncate(0b0010_0100)
    }

    pub fn set_zero(&mut self, value: bool) {
        self.set(ProcessorStatus::ZERO, value);
    }

    pub fn set_negative(&mut self, value: bool) {
        self.set(ProcessorStatus::NEGATIVE, value);
    }

    pub fn set_carry(&mut self, value: bool) {
        self.set(ProcessorStatus::CARRY, value);
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.set(ProcessorStatus::OVERFLOW, value);
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.set(ProcessorStatus::INTERRUPT, value);
    }

    pub fn set_break_command(&mut self, value: bool) {
        self.set(ProcessorStatus::BREAK_COMMAND, value);
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.set(ProcessorStatus::DECIMAL, value);
    }

    pub fn set_break2_command(&mut self, value: bool) {
        self.set(ProcessorStatus::BREAK2_COMMAND, value);
    }
}
