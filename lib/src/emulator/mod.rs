pub trait Emulator {
    fn reset(&mut self);
    fn step(&mut self);
}
