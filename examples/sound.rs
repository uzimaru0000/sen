use std::{thread::sleep, time::Duration};

use sen::speaker::{self, Speaker};

fn main() {
    // let speaker = speaker::silent::SilentSpeaker::new();
    let speaker = speaker::sdl::SdlSpeaker::new(&sdl2::init().unwrap());

    speaker.send(
        1,
        speaker::SpeakerEvent::SquareNote {
            duty: 0.5,
            hz: 440.0,
            volume: 0.5,
        },
    );
    println!("Playing 440Hz square wave for 1 second");

    sleep(Duration::from_millis(1000));

    speaker.send(
        1,
        speaker::SpeakerEvent::SquareNote {
            duty: 0.25,
            hz: 380.0,
            volume: 0.5,
        },
    );
    println!("Playing 380Hz square wave for 1 second");

    sleep(Duration::from_millis(1000));

    speaker.send(
        1,
        speaker::SpeakerEvent::SquareNote {
            duty: 0.25,
            hz: 380.0,
            volume: 0.0,
        },
    );
    println!("Stopping square wave for 1 second");

    speaker.send(
        4,
        speaker::SpeakerEvent::NoiseNote {
            mode: sen::utils::noise::NoiseMode::Long,
            hz: 1789772.5 / (0x008 as f32),
            volume: 0.5,
        },
    );
    println!("Playing noise for 1 second");

    sleep(Duration::from_millis(1000));

    speaker.send(
        4,
        speaker::SpeakerEvent::NoiseNote {
            mode: sen::utils::noise::NoiseMode::Short,
            hz: 1789772.5 / (0x07d as f32),
            volume: 0.5,
        },
    );
    println!("Playing noise for 1 second");

    sleep(Duration::from_millis(1000));

    speaker.send(
        4,
        speaker::SpeakerEvent::NoiseNote {
            mode: sen::utils::noise::NoiseMode::Short,
            hz: 1789772.5 / (0x07d as f32),
            volume: 0.0,
        },
    );
    println!("Stopping noise for 1 second");

    speaker.send(3, speaker::SpeakerEvent::TriangleNote { hz: 440.0 });
    println!("Playing 440Hz triangle wave for 1 second");

    sleep(Duration::from_millis(1000));
}
