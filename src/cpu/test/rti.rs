use super::{get_opecode, CPUTest, TestCPU};
use crate::bus::Mem;
use crate::cpu::addressing_mode::AddressingMode;
use test_case::test_case;

type TestResult = (u8, u16);

fn assert(cpu: &mut TestCPU) -> TestResult {
    (cpu.status.into(), cpu.program_counter)
}

#[test_case(
    vec![
        get_opecode("RTI", AddressingMode::NoneAddressing),
        0x00
    ],
    |cpu| {
        cpu.stack_pointer = 0xFC;
        cpu.mem_write(0x01FD, 0b00110000);
        cpu.mem_write_u16(0x01FE, 0x1234);
    },
    assert => (0b00110000, 0x1235);
    "rti"
)]
fn test_rti(
    code: Vec<u8>,
    initialize: fn(&mut TestCPU),
    assert: fn(&mut TestCPU) -> TestResult,
) -> TestResult {
    CPUTest::new(code, initialize, assert).run()
}
