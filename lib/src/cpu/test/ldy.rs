use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::status::ProcessorStatus;
use test_case::test_case;

fn assert(cpu: &mut TestCPU) -> (u8, bool, bool) {
    (
        cpu.register_y,
        cpu.status.contains(ProcessorStatus::ZERO),
        cpu.status.contains(ProcessorStatus::NEGATIVE),
    )
}

#[test_case(
    vec![get_opecode("LDY", AddressingMode::Immediate), 0x10, 0x00],
    |_cpu| {},
    assert => (0x10, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("LDY", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| { cpu.mem_write(0x10, 0x55) },
    assert => (0x55, false, false);
    "zero_page"
)]
#[test_case(
    vec![get_opecode("LDY", AddressingMode::ZeroPageX), 0x10, 0x00],
    |cpu| { cpu.register_x = 0x05; cpu.mem_write(0x15, 0x55) },
    assert => (0x55, false, false);
    "zero_page_x"
)]
#[test_case(
    vec![get_opecode("LDY", AddressingMode::Absolute), 0x10, 0x00],
    |cpu| { cpu.mem_write_u16(0x10, 0x55) },
    assert => (0x55, false, false);
    "absolute"
)]
#[test_case(
    vec![get_opecode("LDY", AddressingMode::AbsoluteX), 0x10, 0x00],
    |cpu| { cpu.register_x = 0x05; cpu.mem_write_u16(0x15, 0x55) },
    assert => (0x55, false, false);
    "absolute_x"
)]
fn test_ldy(
    code: Vec<u8>,
    init: fn(cpu: &mut TestCPU) -> (),
    assert: fn(&mut TestCPU) -> (u8, bool, bool),
) -> (u8, bool, bool) {
    CPUTest::new(code, init, assert).run()
}
