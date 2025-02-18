use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::status::ProcessorStatus;
use test_case::test_case;

type TestResult = (u8, bool, bool);

fn assert_inc(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.mem_read(0x10),
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

fn assert_inx(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.register_x,
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

fn assert_iny(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.register_y,
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

#[test_case(
    vec![get_opecode("INC", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| {
        cpu.mem_write(0x10, 0x01);
    },
    assert_inc => (0x02, false, false);
    "inc"
)]
#[test_case(
    vec![get_opecode("INX", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_x = 0x01;
    },
    assert_inx => (0x02, false, false);
    "inx"
)]
#[test_case(
    vec![get_opecode("INY", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_y = 0x01;
    },
    assert_iny => (0x02, false, false);
    "iny"
)]
fn test_inc(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
