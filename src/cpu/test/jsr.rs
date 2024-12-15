use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = (u8, u8, u16);

fn assert(cpu: &mut TestCPU) -> TestResult {
    (cpu.register_x, cpu.stack_pointer, cpu.program_counter)
}

#[test_case(
    vec![
        get_opecode("JSR", AddressingMode::Absolute), 0x05, 0x80,
        0x00,
        0x00,
        get_opecode("INX", AddressingMode::NoneAddressing),
        0x00,
    ],
    |_cpu| {},
    assert => (0x01, 0xFB, 0x8007);
    "jsr"
)]
#[test_case(
    vec![
        get_opecode("JSR", AddressingMode::Absolute), 0x05, 0x80,
        0x00,
        0x00,
        get_opecode("INX", AddressingMode::NoneAddressing),
        get_opecode("RTS", AddressingMode::NoneAddressing),
        0x00,
    ],
    |_cpu| {},
    assert => (0x01, 0xFD, 0x8004);
    "rts"
)]
fn test_jsr(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
