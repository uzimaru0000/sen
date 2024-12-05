use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = bool;

#[test_case(
    vec![get_opecode("CLC", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.status.set_carry(true);
    },
    |cpu| cpu.status.carry => false;
    "clc"
)]
#[test_case(
    vec![get_opecode("CLD", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.status.set_decimal(true);
    },
    |cpu| cpu.status.decimal => false;
    "cld"
)]
#[test_case(
    vec![get_opecode("CLI", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.status.set_interrupt(true);
    },
    |cpu| cpu.status.interrupt => false;
    "cli"
)]
#[test_case(
    vec![get_opecode("CLV", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.status.set_overflow(true);
    },
    |cpu| cpu.status.overflow => false;
    "clv"
)]
#[test_case(
    vec![get_opecode("SEC", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.status.set_carry(false);
    },
    |cpu| cpu.status.carry => true;
    "sec"
)]
#[test_case(
    vec![get_opecode("SED", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.status.set_decimal(false);
    },
    |cpu| cpu.status.decimal => true;
    "sed"
)]
#[test_case(
    vec![get_opecode("SEI", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.status.set_interrupt(false);
    },
    |cpu| cpu.status.interrupt => true;
    "sei"
)]
fn test_status(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
