use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::status::ProcessorStatus;
use test_case::test_case;

type TestResult = (u8, bool, bool);

fn assert_dec(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.mem_read(0x10),
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

fn assert_dex(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.register_x,
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

fn assert_dey(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.register_y,
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

#[test_case(
    vec![get_opecode("DEC", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| {
        cpu.mem_write(0x10, 0x01);
    },
    assert_dec => (0x00, true, false);
    "dec"
)]
#[test_case(
    vec![get_opecode("DEX", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_x = 0x01;
    },
    assert_dex => (0x00, true, false);
    "dex"
)]
#[test_case(
    vec![get_opecode("DEY", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_y = 0x01;
    },
    assert_dey => (0x00, true, false);
    "dey"
)]
fn test_dec(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
