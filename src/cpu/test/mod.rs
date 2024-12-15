mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod bvc;
mod bvs;
mod cmp;
mod dec;
mod eor;
mod inc;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod ora;
mod rol;
mod ror;
mod rti;
mod sbc;
mod stack;
mod status;
mod store;
mod transfer;

use crate::{
    bus::{Bus, Mem},
    cpu::CPU,
};

use super::{addressing_mode::AddressingMode, opecode::CPU_OPCODE};

struct TestBus {
    mem: [u8; 0x10000],
}

impl TestBus {
    fn new(code: &[u8]) -> TestBus {
        let mut mem = [0u8; 0x10000];
        mem[0x8000..(0x8000 + code.len())].copy_from_slice(&code[..]);

        TestBus { mem }
    }
}

impl Mem for TestBus {
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        // NOTE: ProgramCounter の初期化のために決め打ちで 0x0000 を返す
        if addr == 0xFFFC {
            return 0x8000;
        }

        let low_byte = self.mem_read(addr) as u16;
        let high_byte = self.mem_read(addr + 1) as u16;
        (high_byte << 8) | low_byte
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let high_byte = (data >> 8) as u8;
        let low_byte = (data & 0xFF) as u8;
        self.mem_write(addr, low_byte);
        self.mem_write(addr + 1, high_byte);
    }
}

impl Bus for TestBus {
    fn tick(&mut self, _cycles: u8) {
        // NOTE: 何もしない
    }

    fn poll_nmi_status(&self) -> Option<bool> {
        // NOTE: 何もしない
        None
    }

    fn get_cycles(&self) -> (usize, usize) {
        (0, 0)
    }

    fn get_scanline(&self) -> u16 {
        0
    }
}

pub(self) type TestCPU = CPU<TestBus>;

pub(self) struct CPUTest<F, G, R>
where
    F: FnMut(&mut TestCPU) -> (),
    G: FnMut(&mut TestCPU) -> R,
{
    code: Vec<u8>,
    initialize: F,
    assert: G,
}

impl<F, G, R> CPUTest<F, G, R>
where
    F: FnMut(&mut TestCPU) -> (),
    G: FnMut(&mut TestCPU) -> R,
{
    pub(super) fn new(code: Vec<u8>, initialize: F, assert: G) -> Self {
        Self {
            code,
            initialize,
            assert,
        }
    }

    pub(super) fn run(&mut self) -> R {
        let bus = TestBus::new(&self.code);
        let mut cpu = CPU::new(bus);

        cpu.reset();

        (self.initialize)(&mut cpu);

        cpu.run_with_callback(|_| {});

        (self.assert)(&mut cpu)
    }
}

pub(super) fn get_opecode(name: &str, mode: AddressingMode) -> u8 {
    CPU_OPCODE
        .iter()
        .find(|x| x.name == name && x.addr_mode == mode)
        .take()
        .unwrap()
        .code
}
