use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = u8;

fn assert(cpu: &mut TestCPU) -> TestResult {
    cpu.register_a
}

#[test_case(
    vec![
        get_opecode("BPL", AddressingMode::NoneAddressing), 0x04,
        0x00,
        0x00,
        0x00,
        get_opecode("LDA", AddressingMode::Immediate), 0xFF,
        0x00
    ],
    |cpu| {
        cpu.status.set_negative(true);
        cpu.register_a = 0x10;
    },
    assert => 0x10;
    "non_branch"
)]
#[test_case(
    vec![
        get_opecode("BPL", AddressingMode::NoneAddressing), 0x03,
        0x00,
        0x00,
        0x00,
        get_opecode("LDA", AddressingMode::Immediate), 0xFF,
        0x00
    ],
    |cpu| {
        cpu.register_a = 0x10;
    },
    assert => 0xFF;
    "has_branch"
)]
fn test_bpl(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
