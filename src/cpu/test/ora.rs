use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = (u8, bool, bool);

fn assert(cpu: &mut TestCPU) -> TestResult {
    (cpu.register_a, cpu.status.zero, cpu.status.negative)
}

#[test_case(
    vec![get_opecode("ORA", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x00;
    },
    assert => (0x10, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("ORA", AddressingMode::Immediate), 0x00, 0x00],
    |cpu| {
        cpu.register_a = 0x00;
    },
    assert => (0x00, true, false);
    "update_zero_flags"
)]
#[test_case(
    vec![get_opecode("ORA", AddressingMode::Immediate), 0x80, 0x00],
    |cpu| {
        cpu.register_a = 0x00;
    },
    assert => (0x80, false, true);
    "update_negative_flags"
)]
fn test_ora(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
