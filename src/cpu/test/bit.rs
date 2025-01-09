use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::status::ProcessorStatus;
use test_case::test_case;

type TestResult = (bool, bool, bool);

fn assert(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.status.contains(ProcessorStatus::NEGATIVE),
        cpu.status.contains(ProcessorStatus::OVERFLOW),
        cpu.status.contains(ProcessorStatus::ZERO),
    )
}

#[test_case(
    vec![get_opecode("BIT", AddressingMode::ZeroPage), 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0x01;
        cpu.mem_write(0x01, 0x00);
    },
    assert => (false, false, true);
    "update_zero_flag"
)]
#[test_case(
    vec![get_opecode("BIT", AddressingMode::ZeroPage), 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0x60;
        cpu.mem_write(0x01, 0xFF);
    },
    assert => (true, true, false);
    "update_negative_and_overflow_flag"
)]
fn test_bit(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
