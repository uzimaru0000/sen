use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::status::ProcessorStatus;
use test_case::test_case;

fn assert(cpu: &mut TestCPU) -> (u8, bool, bool) {
    (
        cpu.register_a,
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::CARRY),
    )
}

#[test_case(
    vec![get_opecode("EOR", AddressingMode::Immediate), 0xAA],
    |cpu| {
        cpu.register_a = 0xCC;
    },
    assert => (0x66, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("EOR", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0xCC;
        cpu.mem_write(0x10, 0xAA);
    },
    assert => (0x66, false, false);
    "zero_page"
)]
#[test_case(
    vec![get_opecode("EOR", AddressingMode::ZeroPageX), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0xCC;
        cpu.register_x = 0x01;
        cpu.mem_write(0x11, 0xAA);
    },
    assert => (0x66, false, false);
    "zero_page_x"
)]
#[test_case(
    vec![get_opecode("EOR", AddressingMode::Absolute), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0xCC;
        cpu.mem_write_u16(0x0110, 0xAA);
    },
    assert => (0x66, false, false);
    "absolute"
)]
#[test_case(
    vec![get_opecode("EOR", AddressingMode::AbsoluteX), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0xCC;
        cpu.register_x = 0x01;
        cpu.mem_write_u16(0x0111, 0xAA);
    },
    assert => (0x66, false, false);
    "absolute_x"
)]
#[test_case(
    vec![get_opecode("EOR", AddressingMode::AbsoluteY), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.register_a = 0xCC;
        cpu.register_y = 0x01;
        cpu.mem_write_u16(0x0111, 0xAA);
    },
    assert => (0x66, false, false);
    "absolute_y"
)]
#[test_case(
    vec![get_opecode("EOR", AddressingMode::IndirectX), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0xCC;
        cpu.register_x = 0x01;
        cpu.mem_write_u16(0x11, 0x1000);
        cpu.mem_write_u16(0x1000, 0xAA);
    },
    assert => (0x66, false, false);
    "indirect_x"
)]
#[test_case(
    vec![get_opecode("EOR", AddressingMode::IndirectY), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0xCC;
        cpu.register_y = 0x01;
        cpu.mem_write_u16(0x10, 0x1000);
        cpu.mem_write_u16(0x1001, 0xAA);
    },
    assert => (0x66, false, false);
    "indirect_y"
)]
fn test_asl(
    code: Vec<u8>,
    init: fn(cpu: &mut TestCPU) -> (),
    assert: fn(&mut TestCPU) -> (u8, bool, bool),
) -> (u8, bool, bool) {
    CPUTest::new(code, init, assert).run()
}
