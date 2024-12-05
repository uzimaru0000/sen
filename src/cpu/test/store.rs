use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = u8;

fn assert(cpu: &TestCPU) -> TestResult {
    cpu.mem_read(0x10)
}

#[test_case(
    vec![get_opecode("STA", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| {
        cpu.register_a = 0x10;
    },
    assert => 0x10;
    "sta"
)]
#[test_case(
    vec![get_opecode("STX", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| {
        cpu.register_x = 0x10;
    },
    assert => 0x10;
    "stx"
)]
#[test_case(
    vec![get_opecode("STY", AddressingMode::ZeroPage), 0x10, 0x00],
    |cpu| {
        cpu.register_y = 0x10;
    },
    assert => 0x10;
    "sty"
)]
fn test_store(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
