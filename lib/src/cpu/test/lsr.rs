use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::status::ProcessorStatus;
use test_case::test_case;

type TestResult = (u8, bool, bool, bool);

fn assert(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.register_a,
        cpu.status.contains(ProcessorStatus::CARRY),
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

#[test_case(
    vec![get_opecode("LSR", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0xFE;
    },
    assert => (0x7F, false, false, false);
    "shift_right"
)]
#[test_case(
    vec![get_opecode("LSR", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x01;
    },
    assert => (0x00, true, true, false);
    "update_zero"
)]
#[test_case(
    vec![get_opecode("LSR", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x03;
    },
    assert => (0x01, true, false, false);
    "update_carry"
)]
fn test_lsr(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
