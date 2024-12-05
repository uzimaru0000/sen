use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

fn assert(cpu: &TestCPU) -> (u8, bool, bool) {
    (cpu.register_x, cpu.status.zero, cpu.status.negative)
}

#[test_case(
    vec![get_opecode("LDX", AddressingMode::Immediate), 0x10, 0x00],
    |_cpu| {},
    assert => (0x10, false, false);
    "immediate"
)]
#[test_case(
    vec![get_opecode("LDX", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| { cpu.mem_write(0x10, 0x55) },
    assert => (0x55, false, false);
    "zero_page"
)]
#[test_case(
    vec![get_opecode("LDX", AddressingMode::ZeroPageY), 0x10, 0x00],
    |cpu| { cpu.register_y = 0x05; cpu.mem_write(0x15, 0x55) },
    assert => (0x55, false, false);
    "zero_page_y"
)]
#[test_case(
    vec![get_opecode("LDX", AddressingMode::Absolute), 0x10, 0x00],
    |cpu| { cpu.mem_write_u16(0x10, 0x55) },
    assert => (0x55, false, false);
    "absolute"
)]
#[test_case(
    vec![get_opecode("LDX", AddressingMode::AbsoluteY), 0x10, 0x00],
    |cpu| { cpu.register_y = 0x05; cpu.mem_write_u16(0x15, 0x55) },
    assert => (0x55, false, false);
    "absolute_y"
)]
fn test_ldx(
    code: Vec<u8>,
    init: fn(cpu: &mut TestCPU) -> (),
    assert: fn(&TestCPU) -> (u8, bool, bool),
) -> (u8, bool, bool) {
    CPUTest::new(code, init, assert).run()
}
