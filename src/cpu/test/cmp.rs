use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = (bool, bool, bool);

fn assert(cpu: &mut TestCPU) -> TestResult {
    (cpu.status.zero, cpu.status.carry, cpu.status.negative)
}

#[test_case(
    vec![get_opecode("CMP", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x20;
    },
    assert => (false, true, false);
    "cmp_a_more_than"
)]
#[test_case(
    vec![get_opecode("CMP", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x10;
    },
    assert => (true, true, false);
    "cmp_a_equal"
)]
#[test_case(
    vec![get_opecode("CMP", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0xF0;
    },
    assert => (false, true, true);
    "cmp_a_update_negative"
)]
#[test_case(
    vec![get_opecode("CPX", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_x = 0x20;
    },
    assert => (false, true, false);
    "cmp_x_more_than"
)]
#[test_case(
    vec![get_opecode("CPX", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_x = 0x10;
    },
    assert => (true, true, false);
    "cmp_x_equal"
)]
#[test_case(
    vec![get_opecode("CPX", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_x = 0xF0;
    },
    assert => (false, true, true);
    "cmp_x_update_negative"
)]
#[test_case(
    vec![get_opecode("CPY", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_y = 0x20;
    },
    assert => (false, true, false);
    "cmp_y_more_than"
)]
#[test_case(
    vec![get_opecode("CPY", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_y = 0x10;
    },
    assert => (true, true, false);
    "cmp_y_equal"
)]
#[test_case(
    vec![get_opecode("CPY", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_y = 0xF0;
    },
    assert => (false, true, true);
    "cmp_y_update_negative"
)]
fn test_cmp(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
