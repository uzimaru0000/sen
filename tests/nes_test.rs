use sen::{
    bus::NESBus,
    cpu::{trace::trace, CPU},
    joypad::dummy::DummyHandler,
    render::dummy::DummyRenderer,
    rom::Rom,
    speaker::silent::SilentSpeaker,
};

#[test]
fn test_nestest() {
    let raw = include_bytes!("../fixtures/nestest.nes");
    let rom = Rom::new(raw).unwrap();
    let speaker = SilentSpeaker::new();
    let bus = NESBus::new(rom, speaker, DummyHandler, DummyRenderer);
    let mut cpu = CPU::new(bus);
    cpu.reset_with_pc(0xC000);

    let mut log = String::new();
    cpu.run_with_callback(|cpu| {
        let line = format!("{}\n", trace(cpu));
        log.push_str(&line);
        print!("{}", line);
    });

    let expected = include_str!("../fixtures/nestest.log");
    assert_eq!(log, expected);
}
