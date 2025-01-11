use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = u8;

fn assert(cpu: &mut TestCPU) -> TestResult {
    cpu.register_x
}

#[test_case(
    vec![
        get_opecode("JMP", AddressingMode::Absolute), 0x04, 0x80,
        0x00,
        get_opecode("INX", AddressingMode::NoneAddressing),
        0x00,
    ],
    |_cpu| {},
    assert => 0x01;
    "jump absolute"
)]
#[test_case(
    vec![
        get_opecode("JMP", AddressingMode::Indirect), 0x00, 0x10,
        0x00,
        get_opecode("INX", AddressingMode::NoneAddressing),
        0x00,
    ],
    |cpu| {
        cpu.mem_write_u16(0x1000, 0x2000);
        cpu.mem_write_u16(0x2000, 0x8004);
    },
    assert => 0x01;
    "jump indirect"
)]
fn test_jmp(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
