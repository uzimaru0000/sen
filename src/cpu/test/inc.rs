use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = (u8, bool, bool);

fn assert_inc(cpu: &TestCPU) -> TestResult {
    (cpu.mem_read(0x10), cpu.status.zero, cpu.status.negative)
}

fn assert_inx(cpu: &TestCPU) -> TestResult {
    (cpu.register_x, cpu.status.zero, cpu.status.negative)
}

fn assert_iny(cpu: &TestCPU) -> TestResult {
    (cpu.register_y, cpu.status.zero, cpu.status.negative)
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
    assert: fn(&TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
