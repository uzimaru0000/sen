use lib::speaker::Speaker;

pub struct CliSpeaker;

impl Speaker for CliSpeaker {
    fn send(&self, _: u8, _: lib::speaker::SpeakerEvent) {}
}
