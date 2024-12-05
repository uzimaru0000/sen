use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

#[test_case(
    vec![get_opecode("ASL", AddressingMode::NoneAddressing), 0x00],
    |cpu| {
        cpu.register_a = 0x80;
    },
    |cpu| (cpu.register_a, cpu.status.zero, cpu.status.carry, cpu.status.negative) => (0x00, true, true, false);
    "accumulator"
)]
#[test_case(
    vec![get_opecode("ASL", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| {
        cpu.mem_write(0x10, 0x80);
    },
    |cpu| (cpu.mem_read(0x10), cpu.status.zero, cpu.status.carry, cpu.status.negative) => (0x00, true, true, false);
    "zero_page"
)]
#[test_case(
    vec![get_opecode("ASL", AddressingMode::ZeroPageX), 0x10, 0x00],
    |cpu| {
        cpu.register_x = 0x01;
        cpu.mem_write(0x11, 0x80);
    },
    |cpu| (cpu.mem_read(0x11), cpu.status.zero, cpu.status.carry, cpu.status.negative) => (0x00, true, true, false);
    "zero_page_x"
)]
#[test_case(
    vec![get_opecode("ASL", AddressingMode::Absolute), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.mem_write(0x0110, 0x80);
    },
    |cpu| (cpu.mem_read(0x0110), cpu.status.zero, cpu.status.carry, cpu.status.negative) => (0x00, true, true, false);
    "absolute"
)]
#[test_case(
    vec![get_opecode("ASL", AddressingMode::AbsoluteX), 0x10, 0x01, 0x00],
    |cpu| {
        cpu.register_x = 0x01;
        cpu.mem_write(0x0111, 0x80);
    },
    |cpu| (cpu.mem_read(0x10), cpu.status.zero, cpu.status.carry, cpu.status.negative) => (0x00, true, true, false);
    "absolute_x"
)]
fn test_asl(
    code: Vec<u8>,
    init: fn(cpu: &mut TestCPU) -> (),
    assert: fn(&TestCPU) -> (u8, bool, bool, bool),
) -> (u8, bool, bool, bool) {
    CPUTest::new(code, init, assert).run()
}
