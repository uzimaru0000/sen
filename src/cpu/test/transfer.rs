use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = (u8, bool, bool);

fn assert_x_register(cpu: &mut TestCPU) -> TestResult {
    (cpu.register_x, cpu.status.zero, cpu.status.negative)
}

fn assert_y_register(cpu: &mut TestCPU) -> TestResult {
    (cpu.register_y, cpu.status.zero, cpu.status.negative)
}

fn assert_a_register(cpu: &mut TestCPU) -> TestResult {
    (cpu.register_a, cpu.status.zero, cpu.status.negative)
}

fn assert_stack_pointer(cpu: &mut TestCPU) -> TestResult {
    (cpu.stack_pointer, cpu.status.zero, cpu.status.negative)
}

#[test_case(
    vec![get_opecode("TAX", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x10;
    },
    assert_x_register => (0x10, false, false);
    "tax"
)]
#[test_case(
    vec![get_opecode("TAY", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x10;
    },
    assert_y_register => (0x10, false, false);
    "tay"
)]
#[test_case(
    vec![get_opecode("TSX", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.stack_pointer = 0xF0;
    },
    assert_x_register => (0xF0, false, true);
    "tsx"
)]
#[test_case(
    vec![get_opecode("TXA", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_x = 0x10;
    },
    assert_a_register => (0x10, false, false);
    "txa"
)]
#[test_case(
    vec![get_opecode("TXS", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_x = 0xF0;
    },
    assert_stack_pointer => (0xF0, false, false);
    "txs"
)]
#[test_case(
    vec![get_opecode("TYA", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_y = 0x10;
    },
    assert_a_register => (0x10, false, false);
    "tya"
)]
fn test_tmp(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
