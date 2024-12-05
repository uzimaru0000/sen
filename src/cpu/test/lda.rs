use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

fn assert(cpu: &TestCPU) -> (u8, bool, bool) {
    (cpu.register_a, cpu.status.zero, cpu.status.negative)
}

#[test_case(
    vec![get_opecode("LDA", AddressingMode::Immediate), 0x10, 0x00],
    |_cpu| {},
    assert => (0x10, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("LDA", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| { cpu.mem_write(0x10, 0x55) },
    assert => (0x55, false, false);
    "zero_page"
)]
#[test_case(
    vec![get_opecode("LDA", AddressingMode::ZeroPageX), 0x10, 0x00],
    |cpu| { cpu.register_x = 0x05; cpu.mem_write(0x15, 0x55) },
    assert => (0x55, false, false);
    "zero_page_x"
)]
#[test_case(
    vec![get_opecode("LDA", AddressingMode::Absolute), 0x10, 0x00],
    |cpu| { cpu.mem_write_u16(0x10, 0x55) },
    assert => (0x55, false, false);
    "absolute"
)]
#[test_case(
    vec![get_opecode("LDA", AddressingMode::AbsoluteX), 0x10, 0x00],
    |cpu| { cpu.register_x = 0x05; cpu.mem_write_u16(0x15, 0x55) },
    assert => (0x55, false, false);
    "absolute_x"
)]
#[test_case(
    vec![get_opecode("LDA", AddressingMode::AbsoluteY), 0x10, 0x00],
    |cpu| { cpu.register_y = 0x05; cpu.mem_write_u16(0x15, 0x55) },
    assert => (0x55, false, false);
    "absolute_y"
)]
#[test_case(
    vec![get_opecode("LDA", AddressingMode::IndirectX), 0x10, 0x00],
    |cpu| {
        cpu.register_x = 0x05;
        cpu.mem_write_u16(0x15, 0x55);
        cpu.mem_write(0x55, 0x15);
    },
    assert => (0x15, false, false);
    "indirect_x"
)]
#[test_case(
    vec![get_opecode("LDA", AddressingMode::IndirectY), 0x10, 0x00],
    |cpu| {
        cpu.register_y = 0x05;
        cpu.mem_write_u16(0x10, 0x50);
        cpu.mem_write(0x55, 0x55);
    },
    assert => (0x55, false, false);
    "indirect_y"
)]
fn test_lda(
    code: Vec<u8>,
    init: fn(cpu: &mut TestCPU) -> (),
    assert: fn(&TestCPU) -> (u8, bool, bool),
) -> (u8, bool, bool) {
    CPUTest::new(code, init, assert).run()
}
