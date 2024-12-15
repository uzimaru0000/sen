use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = (u8, bool, bool, bool);

fn assert(cpu: &mut TestCPU) -> TestResult {
    (
        cpu.register_a,
        cpu.status.carry,
        cpu.status.zero,
        cpu.status.negative,
    )
}

#[test_case(
    vec![get_opecode("ROL", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x3F;
    },
    assert => (0x7E, false, false, false);
    "rotate_left"
)]
#[test_case(
    vec![get_opecode("ROL", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x80;
    },
    assert => (0x00, true, true, false);
    "update_carry"
)]
#[test_case(
    vec![get_opecode("ROL", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x40;
    },
    assert => (0x80, false, false, true);
    "update_negative"
)]
#[test_case(
    vec![get_opecode("ROL", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x40;
        cpu.status.set_carry(true);
    },
    assert => (0x81, false, false, true);
    "with_carried"
)]
fn test_rol(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
