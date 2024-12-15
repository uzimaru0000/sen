use super::{get_opecode, CPUTest, TestCPU};
use crate::cpu::addressing_mode::AddressingMode;
use crate::bus::Mem;
use test_case::test_case;

#[test_case(
    vec![get_opecode("AND", AddressingMode::Immediate), 0x01, 0x00],
    |cpu| { cpu.register_a = 0x01; },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x01, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("AND", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| { 
        cpu.register_a = 0x01; 
        cpu.mem_write(0x10, 0x01);
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x01, false, false);
    "zero_page"
)]
#[test_case(
    vec![get_opecode("AND", AddressingMode::ZeroPageX), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x11;
        cpu.register_x = 0x01;
        cpu.mem_write(0x11, 0x01);
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x01, false, false);
    "zero_page_x"
)]
#[test_case(
    vec![get_opecode("AND", AddressingMode::Absolute), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0x11;
        cpu.mem_write_u16(0x0110, 0x01);
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x01, false, false);
    "absolute"
)]
#[test_case(
    vec![get_opecode("AND", AddressingMode::AbsoluteX), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0x11;
        cpu.register_x = 0x01;
        cpu.mem_write_u16(0x0111, 0x01);
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x01, false, false);
    "absolute_x"
)]
#[test_case(
    vec![get_opecode("AND", AddressingMode::AbsoluteY), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0x11;
        cpu.register_y = 0x01;
        cpu.mem_write_u16(0x0111, 0x01);
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x01, false, false);
    "absolute_y"
)]
#[test_case(
    vec![get_opecode("AND", AddressingMode::IndirectX), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x11;
        cpu.register_x = 0x01;
        cpu.mem_write(0x10, 0x10);
        cpu.mem_write_u16(0x11, 0x0100);
        cpu.mem_write_u16(0x0100, 0x01);
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x01, false, false);
    "indirect_x"
)]
#[test_case(
    vec![get_opecode("AND", AddressingMode::IndirectY), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0xAA;
        cpu.register_y = 0x01;
        cpu.mem_write_u16(0x0010, 0x2000);
        cpu.mem_write(0x2001, 0xCC);
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x88, false, true);
    "indirect_y"
)]
#[test_case(
    vec![
        get_opecode("PHP", AddressingMode::NoneAddressing),
        get_opecode("PLA", AddressingMode::NoneAddressing),
        get_opecode("AND", AddressingMode::Immediate), 0xEF,
        get_opecode("BRK", AddressingMode::NoneAddressing),
    ],
    |cpu| {
        cpu.register_a = 0x00;
        cpu.register_x = 0x00;
        cpu.register_y = 0x00;
        cpu.status = 0x6F.into();
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.negative) => (0x6F, false, false);
    "pull_status_from_stack"
)]
fn test_and(
    code: Vec<u8>,
    init: fn(cpu: &mut TestCPU) -> (),
    assert: fn(&mut TestCPU) -> (u8, bool, bool),
) -> (u8, bool, bool) {
    CPUTest::new(code, init, assert).run()
}
