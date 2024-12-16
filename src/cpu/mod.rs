use std::fmt::Display;

use addressing_mode::AddressingMode;
use opecode::{OpCode, OPCODE_MAP};
use status::ProcessorStatus;

use crate::bus::{Bus, Mem};

mod addressing_mode;
mod opecode;
mod status;
pub mod trace;

#[cfg(test)]
mod test;

pub struct CPU<M: Mem + Bus> {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: ProcessorStatus,
    pub bus: M,
}

impl<M: Mem + Bus> Mem for CPU<M> {
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
    }

    fn mem_read_u16(&mut self, addr: u16) -> u16 {
        self.bus.mem_read_u16(addr)
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.bus.mem_write_u16(addr, data);
    }
}

impl<M: Mem + Bus> CPU<M> {
    pub fn new(bus: M) -> Self {
        Self {
            program_counter: 0,
            stack_pointer: 0xFD,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0b0010_0100.into(),
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.stack_pointer = 0xFD;
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0b0010_0100.into();

        self.program_counter = self.mem_read_u16(0xFFFC);

        self.bus.tick(7);
    }

    pub fn reset_with_pc(&mut self, pc: u16) {
        self.stack_pointer = 0xFD;
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0b0010_0100.into();

        self.program_counter = pc;

        self.bus.tick(7);
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU<M>, &OpCode),
    {
        loop {
            if let Some(_nmi) = self.bus.poll_nmi_status() {
                self.interrupt_nmi();
            }

            let opcode = self.mem_read(self.program_counter);
            let op = OPCODE_MAP.get(&opcode).unwrap();

            callback(self, op);

            self.program_counter = self.program_counter.wrapping_add(1);

            let additional_cycle = match op.name {
                "ADC" => {
                    let is_crossed_page = self.adc(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "AND" => {
                    let is_crossed_page = self.and(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "ASL" => {
                    self.asl(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "BCC" => self.branch(!self.status.carry),
                "BCS" => self.branch(self.status.carry),
                "BEQ" => self.branch(self.status.zero),
                "BIT" => {
                    self.bit(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "BMI" => self.branch(self.status.negative),
                "BNE" => self.branch(!self.status.zero),
                "BPL" => self.branch(!self.status.negative),
                "BRK" => {
                    self.stack_push_u16(self.program_counter);
                    self.stack_push(self.status.into());

                    self.program_counter = self.mem_read_u16(0xFFFE);
                    self.status.set_break_command(true);

                    0
                }
                "BVC" => self.branch(!self.status.overflow),
                "BVS" => self.branch(self.status.overflow),
                "CLC" => {
                    self.clc();

                    0
                }
                "CLD" => {
                    self.cld();

                    0
                }
                "CLI" => {
                    self.cli();

                    0
                }
                "CLV" => {
                    self.clv();

                    0
                }
                "CMP" => {
                    let is_crossed_page = self.cmp(self.register_a, &op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "CPX" => {
                    self.cmp(self.register_x, &op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "CPY" => {
                    self.cmp(self.register_y, &op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "DCP" => {
                    self.dec(&op.addr_mode);
                    self.cmp(self.register_a, &op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "DEC" => {
                    self.dec(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "DEX" => {
                    self.dex();

                    0
                }
                "DEY" => {
                    self.dey();

                    0
                }
                "EOR" => {
                    let is_crossed_page = self.eor(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "INC" => {
                    self.inc(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "INX" => {
                    self.inx();

                    0
                }
                "INY" => {
                    self.iny();

                    0
                }
                "ISB" => {
                    self.inc(&op.addr_mode);
                    self.sbc(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "JMP" => {
                    self.jmp(&op.addr_mode);

                    0
                }
                "JSR" => {
                    self.jsr(&op.addr_mode);

                    0
                }
                "LAX" => {
                    let is_crossed_page = self.lda(&op.addr_mode);
                    self.tax();
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "LDA" => {
                    let is_crossed_page = self.lda(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "LDX" => {
                    let is_crossed_page = self.ldx(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "LDY" => {
                    let is_crossed_page = self.ldy(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "LSR" => {
                    self.lsr(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "NOP" => {
                    let (_, is_crossed_page) = if op.addr_mode == AddressingMode::NoneAddressing {
                        (0, false)
                    } else {
                        self.get_operand_address(&op.addr_mode)
                    };
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "ORA" => {
                    let is_crossed_page = self.ora(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "PHA" => {
                    self.pha();

                    0
                }
                "PHP" => {
                    self.php();

                    0
                }
                "PLA" => {
                    self.pla();

                    0
                }
                "PLP" => {
                    self.plp();

                    0
                }
                "RLA" => {
                    self.rol(&op.addr_mode);
                    self.and(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "ROL" => {
                    self.rol(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "ROR" => {
                    self.ror(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "RRA" => {
                    self.ror(&op.addr_mode);
                    self.adc(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "RTI" => {
                    self.rti();

                    0
                }
                "RTS" => {
                    self.rts();

                    0
                }
                "SAX" => {
                    self.sax(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "SBC" => {
                    let is_crossed_page = self.sbc(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    if is_crossed_page {
                        1
                    } else {
                        0
                    }
                }
                "SEC" => {
                    self.sec();

                    0
                }
                "SED" => {
                    self.sed();

                    0
                }
                "SEI" => {
                    self.sei();

                    0
                }
                "SLO" => {
                    self.asl(&op.addr_mode);
                    self.ora(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "STA" => {
                    self.sta(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "STX" => {
                    self.stx(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "STY" => {
                    self.sty(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "SRE" => {
                    self.lsr(&op.addr_mode);
                    self.eor(&op.addr_mode);
                    self.program_counter = self.program_counter.wrapping_add((op.size - 1) as u16);

                    0
                }
                "TAX" => {
                    self.tax();

                    0
                }
                "TAY" => {
                    self.tay();

                    0
                }
                "TSX" => {
                    self.tsx();

                    0
                }
                "TXA" => {
                    self.txa();

                    0
                }
                "TXS" => {
                    self.txs();

                    0
                }
                "TYA" => {
                    self.tya();

                    0
                }
                _ => todo!(),
            };

            self.bus.tick(op.cycles + additional_cycle);
        }
    }

    fn adc(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let (result, overflow1) = self.register_a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(if self.status.carry { 1 } else { 0 });
        let overflow = (self.register_a ^ result) & (value ^ result) & 0x80 != 0;
        let carry = overflow1 || overflow2;

        self.register_a = result;
        self.status.set_carry(carry);
        self.status.set_overflow(overflow);
        self.update_zero_and_negative_flags(result);

        is_crossed_page
    }

    fn and(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_cycle) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);

        is_crossed_cycle
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = value << 1;
        let carry = value & 0x80 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = self.register_a & value;
        self.status.set_zero(result == 0);
        self.status.set_overflow(value & 0x40 != 0);
        self.status.set_negative(value & 0x80 != 0);
    }

    fn clc(&mut self) {
        self.status.set_carry(false);
    }

    fn cld(&mut self) {
        self.status.set_decimal(false);
    }

    fn cli(&mut self) {
        self.status.set_interrupt(false);
    }

    fn clv(&mut self) {
        self.status.set_overflow(false);
    }

    fn cmp(&mut self, register: u8, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = register.wrapping_sub(value);
        self.status.set_carry(register >= value);
        self.update_zero_and_negative_flags(result);

        is_crossed_page
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_sub(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a ^= value;
        self.update_zero_and_negative_flags(self.register_a);

        is_crossed_page
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = value.wrapping_add(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::Absolute => {
                let (addr, _) = self.get_operand_address(mode);
                self.program_counter = addr;
            }
            AddressingMode::Indirect => {
                let ptr = self.mem_read_u16(self.program_counter);

                let lo = self.mem_read(ptr);
                let hi = self.mem_read(if ptr & 0xFF == 0xFF {
                    ptr & 0xFF00
                } else {
                    ptr + 1
                });
                let addr = (hi as u16) << 8 | lo as u16;

                self.program_counter = addr;
            }
            _ => panic!("mode {:?} is not supported", mode),
        }
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);

        self.stack_push_u16(self.program_counter + 2 - 1);
        self.program_counter = addr;
    }

    fn lda(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(value);

        is_crossed_page
    }

    fn ldx(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(value);

        is_crossed_page
    }

    fn ldy(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(value);

        is_crossed_page
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = value >> 1;
        let carry = value & 0x01 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn ora(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a |= value;
        self.update_zero_and_negative_flags(self.register_a);

        is_crossed_page
    }

    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    fn php(&mut self) {
        let mut flag = self.status.clone();
        flag.set_break_command(true);
        self.stack_push(flag.into());
    }

    fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        let mut flag = ProcessorStatus::from(self.stack_pop());
        flag.set_break_command(false);
        flag.set_break2_command(true);

        self.status = flag;
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = (value << 1) | if self.status.carry { 1 } else { 0 };
        let carry = value & 0x80 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = (value >> 1) | if self.status.carry { 0x80 } else { 0 };
        let carry = value & 0x01 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let (addr, _) = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn rti(&mut self) {
        let mut flag = ProcessorStatus::from(self.stack_pop());
        flag.set_break_command(false);
        flag.set_break2_command(true);

        self.status = flag;
        self.program_counter = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
    }

    fn sax(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        let value = self.register_a & self.register_x;
        self.mem_write(addr, value);
    }

    fn sbc(&mut self, mode: &AddressingMode) -> bool {
        let (addr, is_crossed_page) = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let (result, overflow1) = self.register_a.overflowing_sub(value);
        let (result, overflow2) = result.overflowing_sub(if self.status.carry { 0 } else { 1 });
        let overflow = (self.register_a & 0x80) != (value & 0x80)
            && (self.register_a & 0x80) != (result & 0x80);
        let carry = !(overflow1 || overflow2);

        self.register_a = result;
        self.status.set_carry(carry);
        self.status.set_overflow(overflow);
        self.update_zero_and_negative_flags(result);

        is_crossed_page
    }

    fn sec(&mut self) {
        self.status.set_carry(true);
    }

    fn sed(&mut self) {
        self.status.set_decimal(true);
    }

    fn sei(&mut self) {
        self.status.set_interrupt(true);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let (addr, _) = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.status.set_zero(value == 0);
        self.status.set_negative(value & 0x80 != 0);
    }

    fn stack_push(&mut self, value: u8) {
        let addr = 0x100 | self.stack_pointer as u16;
        self.mem_write(addr, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & 0xFF) as u8;
        self.stack_push(high_byte);
        self.stack_push(low_byte);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let addr = 0x100 | self.stack_pointer as u16;
        self.mem_read(addr)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let low_byte = self.stack_pop() as u16;
        let high_byte = self.stack_pop() as u16;
        (high_byte << 8) | low_byte
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> (u16, bool) {
        match mode {
            AddressingMode::Immediate => (self.program_counter, false),
            AddressingMode::ZeroPage => (self.mem_read(self.program_counter) as u16, false),
            AddressingMode::ZeroPageX => {
                let address = self.mem_read(self.program_counter) as u16;
                (address.wrapping_add(self.register_x as u16) & 0xFF, false)
            }
            AddressingMode::ZeroPageY => {
                let address = self.mem_read(self.program_counter) as u16;
                (address.wrapping_add(self.register_y as u16) & 0xFF, false)
            }
            AddressingMode::Absolute => (self.mem_read_u16(self.program_counter), false),
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);

                (addr, self.check_page_crossed(base, addr))
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);

                (addr, self.check_page_crossed(base, addr))
            }
            AddressingMode::Indirect => {
                let ptr = self.mem_read_u16(self.program_counter);
                (self.mem_read_u16(ptr), false)
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);

                let ptr = (base as u16).wrapping_add(self.register_x as u16) & 0xFF;

                let lo = self.mem_read(ptr);
                let hi = self.mem_read(ptr.wrapping_add(1) & 0xFF);
                ((hi as u16) << 8 | lo as u16, false)
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base.wrapping_add(1)) as u16);
                let deref_base = (hi as u16) << 8 | lo as u16;
                let addr = deref_base.wrapping_add(self.register_y as u16);

                (addr, self.check_page_crossed(deref_base, addr))
            }
            AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", mode),
        }
    }

    fn branch(&mut self, cond: bool) -> u8 {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if cond {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            let old_pc = self.program_counter;
            self.program_counter = new_pc;

            if self.check_page_crossed(old_pc, new_pc) {
                2
            } else {
                1
            }
        } else {
            0
        }
    }

    fn interrupt_nmi(&mut self) {
        self.stack_push_u16(self.program_counter);

        let mut flag = self.status.clone();
        flag.set_break_command(false);
        flag.set_break2_command(true);
        self.stack_push(flag.into());
        self.status.set_interrupt(true);

        self.bus.tick(2);
        self.program_counter = self.mem_read_u16(0xFFFA);
    }

    fn check_page_crossed(&self, addr1: u16, addr2: u16) -> bool {
        addr1 & 0xFF00 != addr2 & 0xFF00
    }
}

impl<M: Mem + Bus> Display for CPU<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status: u8 = self.status.into();

        write!(
            f,
            "PC:{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.program_counter,
            self.register_a,
            self.register_x,
            self.register_y,
            status,
            self.stack_pointer
        )
    }
}
