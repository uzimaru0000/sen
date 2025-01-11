pub enum InterruptType {
    NMI,
    BRK,
    IRQ,
    RESET,
}

pub struct Interrupt {
    _type: InterruptType,
    break_mask: u8,
    address: u16,
    cycles: u8,
}

impl Interrupt {
    pub fn get_break_mask(&self) -> u8 {
        self.break_mask
    }

    pub fn get_address(&self) -> u16 {
        self.address
    }

    pub fn get_cycles(&self) -> u8 {
        self.cycles
    }
}

pub const NMI: Interrupt = Interrupt {
    _type: InterruptType::NMI,
    break_mask: 0b1110_1111,
    address: 0xFFFA,
    cycles: 2,
};

pub const BRK: Interrupt = Interrupt {
    _type: InterruptType::BRK,
    break_mask: 0b1111_1111,
    address: 0xFFFE,
    cycles: 7,
};

pub const IRQ: Interrupt = Interrupt {
    _type: InterruptType::IRQ,
    break_mask: 0b1110_1111,
    address: 0xFFFE,
    cycles: 7,
};

pub const RESET: Interrupt = Interrupt {
    _type: InterruptType::RESET,
    break_mask: 0b1111_1111,
    address: 0xFFFC,
    cycles: 7,
};
