use super::{Speaker, SpeakerEvent};

pub struct SilentSpeaker;

impl SilentSpeaker {
    pub fn new() -> Self {
        Self
    }
}

impl Speaker for SilentSpeaker {
    fn send(&self, ch: u8, event: SpeakerEvent) {
        println!("CH {} EVENT {:?}", ch, event)
    }
}
