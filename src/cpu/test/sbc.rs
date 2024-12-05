use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

fn assert(cpu: &TestCPU) -> (u8, bool, bool, bool, bool) {
    (
        cpu.register_a,
        cpu.status.zero,
        cpu.status.carry,
        cpu.status.negative,
        cpu.status.overflow,
    )
}

#[test_case(
    vec![get_opecode("SBC", AddressingMode::Immediate), 0x10, 0x00],
    |cpu| { cpu.register_a = 0x20; cpu.status.set_carry(true); },
    assert => (0x10, false, true, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| { cpu.register_a = 0x20; cpu.mem_write(0x10, 0x10); cpu.status.set_carry(true); },
    assert => (0x10, false, true, false, false);
    "zero_page"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::ZeroPageX), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x20;
        cpu.register_x = 0x01;
        cpu.mem_write(0x11, 0x10);
        cpu.status.set_carry(true);
    },
    assert => (0x10, false, true, false, false);
    "zero_page_x"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::Absolute), 0x01, 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x20;
        cpu.mem_write_u16(0x1001, 0x10);
        cpu.status.set_carry(true);
    },
    assert => (0x10, false, true, false, false);
    "absolute"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::AbsoluteX), 0x01, 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x20;
        cpu.register_x = 0x01;
        cpu.mem_write_u16(0x1002, 0x10);
        cpu.status.set_carry(true);
    },
    assert => (0x10, false, true, false, false);
    "absolute_x"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::AbsoluteY), 0x01, 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x20;
        cpu.register_y = 0x01;
        cpu.mem_write_u16(0x1002, 0x10);
        cpu.status.set_carry(true);
    },
    assert => (0x10, false, true, false, false);
    "absolute_y"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::IndirectX), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x20;
        cpu.register_x = 0x04;
        cpu.mem_write_u16(0x14, 0x2000);
        cpu.mem_write(0x2000, 0x30);
        cpu.status.set_carry(true);
    },
    assert => (0xF0, false, false, true, false);
    "indirect_x"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::IndirectY), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x20;
        cpu.register_y = 0x04;
        cpu.mem_write_u16(0x10, 0x2000);
        cpu.mem_write(0x2004, 0x30);
        cpu.status.set_carry(true);
    },
    assert => (0xF0, false, false, true, false);
    "indirect_y"
)]
#[test_case(
    vec![get_opecode("SBC", AddressingMode::Immediate), 0x81, 0x00],
    |cpu| { cpu.register_a = 0x7F; },
    assert => (0xFD, false, false, true, true);
    "overflow"
)]
fn test_sbc(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&TestCPU) -> (u8, bool, bool, bool, bool),
) -> (u8, bool, bool, bool, bool) {
    CPUTest::new(code, initialize, assert).run()
}
