use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct JoypadButton: u8 {
        const RIGHT = 0b1000_0000;
        const LEFT = 0b0100_0000;
        const DOWN = 0b0010_0000;
        const UP = 0b0001_0000;
        const START = 0b0000_1000;
        const SELECT = 0b0000_0100;
        const BUTTON_B = 0b0000_0010;
        const BUTTON_A = 0b0000_0001;
    }
}
