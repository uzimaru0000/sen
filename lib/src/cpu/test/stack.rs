use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::status::ProcessorStatus;
use test_case::test_case;

type TestResult = u8;

fn assert(cpu: &mut TestCPU) -> TestResult {
    cpu.stack_pointer
}

#[test_case(
    vec![get_opecode("PHA", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x10;
    },
    assert => 0xFC;
    "push_register_a"
)]
#[test_case(
    vec![
        get_opecode("PHA", AddressingMode::NoneAddressing),
        get_opecode("LDA", AddressingMode::Immediate), 0x50,
        get_opecode("PLA", AddressingMode::NoneAddressing),
        0x00
    ],
    |cpu| {
        cpu.register_a = 0x10;
    },
    assert => 0xFD;
    "pull_register_a"
)]
#[test_case(
    vec![
        get_opecode("PHP", AddressingMode::NoneAddressing),
        0x00
    ],
    |cpu| {
        cpu.status = ProcessorStatus::from_bits_truncate(0b1100_0000);
    },
    assert => 0xFC;
    "push_processor_status"
)]
#[test_case(
    vec![
        get_opecode("PHP", AddressingMode::NoneAddressing),
        get_opecode("CLC", AddressingMode::NoneAddressing),
        get_opecode("PLP", AddressingMode::NoneAddressing),
        0x00
    ],
    |cpu| {
        cpu.status = ProcessorStatus::from_bits_truncate(0b1100_0000);
    },
    assert => 0xFD;
    "pull_processor_status"
)]
fn test_stack(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
