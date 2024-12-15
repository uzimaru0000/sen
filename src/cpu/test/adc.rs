use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type ResultType = (u8, bool, bool, bool, bool);

fn assert(cpu: &mut TestCPU) -> ResultType {
    (
        cpu.register_a,
        cpu.status.zero,
        cpu.status.negative,
        cpu.status.carry,
        cpu.status.overflow,
    )
}

#[test_case(
    vec![get_opecode("ADC", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| { cpu.register_a = 0x10 },
    assert => (0x20, false, false, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("ADC", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0xFF;
        cpu.status.set_carry(true);
    },
    assert => (0x10, false, false, true, false);
    "with_carry"
)]
#[test_case(
    vec![get_opecode("ADC", AddressingMode::Immediate), 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0x7F;
    },
    assert => (0x80, false, true, false, true);
    "negative"
)]
#[test_case(
    vec![get_opecode("ADC", AddressingMode::Immediate), 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0xFF;
    },
    assert => (0x00, true, false, true, false);
    "zero"
)]
#[test_case(
    vec![get_opecode("ADC", AddressingMode::Immediate), 0x50, 0x00],
    |cpu| {
        cpu.register_a = 0x50;
    },
    assert => (0xA0, false, true, false, true);
    "overflow"
)]
fn test_adc(
    code: Vec<u8>,
    init: fn(cpu: &mut TestCPU) -> (),
    assert: fn(&mut TestCPU) -> ResultType,
) -> ResultType {
    CPUTest::new(code, init, assert).run()
}
