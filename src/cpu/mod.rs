use core::fmt;
use std::fmt::Formatter;

use addressing_mode::AddressingMode;
use opecode::OPCODE_MAP;
use status::ProcessorStatus;

use crate::bus::Mem;

mod addressing_mode;
mod opecode;
mod status;

#[cfg(test)]
mod test;

pub struct CPU<M: Mem> {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: ProcessorStatus,
    pub bus: M,
}

impl<M: Mem> Mem for CPU<M> {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data);
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        self.bus.mem_read_u16(addr)
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        self.bus.mem_write_u16(addr, data);
    }
}

impl<M: Mem> CPU<M> {
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
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU<M>),
    {
        loop {
            callback(self);

            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let op = OPCODE_MAP.get(&opcode).unwrap();

            match op.name {
                "ADC" => {
                    self.adc(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "AND" => {
                    self.and(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "ASL" => {
                    self.asl(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "BCC" => {
                    self.bcc();
                }
                "BCS" => {
                    self.bcs();
                }
                "BEQ" => {
                    self.beq();
                }
                "BIT" => {
                    self.bit(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "BMI" => {
                    self.bmi();
                }
                "BNE" => {
                    self.bne();
                }
                "BPL" => {
                    self.bpl();
                }
                "BRK" => {
                    break;
                }
                "BVC" => {
                    self.bvc();
                }
                "BVS" => {
                    self.bvs();
                }
                "CLC" => {
                    self.clc();
                }
                "CLD" => {
                    self.cld();
                }
                "CLI" => {
                    self.cli();
                }
                "CLV" => {
                    self.clv();
                }
                "CMP" => {
                    self.cmp(self.register_a, &op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "CPX" => {
                    self.cmp(self.register_x, &op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "CPY" => {
                    self.cmp(self.register_y, &op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "DEC" => {
                    self.dec(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "DEX" => {
                    self.dex();
                }
                "DEY" => {
                    self.dey();
                }
                "EOR" => {
                    self.eor(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "INC" => {
                    self.inc(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "INX" => {
                    self.inx();
                }
                "INY" => {
                    self.iny();
                }
                "JMP" => {
                    self.jmp(&op.addr_mode);
                }
                "JSR" => {
                    self.jsr(&op.addr_mode);
                }
                "LDA" => {
                    self.lda(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "LDX" => {
                    self.ldx(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "LDY" => {
                    self.ldy(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "LSR" => {
                    self.lsr(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "NOP" => {
                    // do nothing
                }
                "ORA" => {
                    self.ora(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "PHA" => {
                    self.pha();
                }
                "PHP" => {
                    self.php();
                }
                "PLA" => {
                    self.pla();
                }
                "PLP" => {
                    self.plp();
                }
                "ROL" => {
                    self.rol(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "ROR" => {
                    self.ror(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "RTI" => {
                    self.rti();
                }
                "RTS" => {
                    self.rts();
                }
                "SBC" => {
                    self.sbc(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "SEC" => {
                    self.sec();
                }
                "SED" => {
                    self.sed();
                }
                "SEI" => {
                    self.sei();
                }
                "STA" => {
                    self.sta(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "STX" => {
                    self.stx(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "STY" => {
                    self.sty(&op.addr_mode);
                    self.program_counter += (op.size - 1) as u16;
                }
                "TAX" => {
                    self.tax();
                }
                "TAY" => {
                    self.tay();
                }
                "TSX" => {
                    self.tsx();
                }
                "TXA" => {
                    self.txa();
                }
                "TXS" => {
                    self.txs();
                }
                "TYA" => {
                    self.tya();
                }
                _ => todo!(),
            }
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let (result, overflow1) = self.register_a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(if self.status.carry { 1 } else { 0 });
        let overflow = (self.register_a ^ result) & (value ^ result) & 0x80 != 0;
        let carry = overflow1 || overflow2;

        self.register_a = result;
        self.status.set_carry(carry);
        self.status.set_overflow(overflow);
        self.update_zero_and_negative_flags(result);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = value << 1;
        let carry = value & 0x80 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn bcc(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if !self.status.carry {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
    }

    fn bcs(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status.carry {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
    }

    fn beq(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status.zero {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = self.register_a & value;
        self.status.set_zero(result == 0);
        self.status.set_overflow(value & 0x40 != 0);
        self.status.set_negative(value & 0x80 != 0);
    }

    fn bmi(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status.negative {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
    }

    fn bne(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if !self.status.zero {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
    }

    fn bpl(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if !self.status.negative {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
    }

    fn bvc(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if !self.status.overflow {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
    }

    fn bvs(&mut self) {
        let offset = self.mem_read(self.program_counter) as i8;
        self.program_counter += 1;

        if self.status.overflow {
            let new_pc = self.program_counter.wrapping_add(offset as u16);
            self.program_counter = new_pc;
        }
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

    fn cmp(&mut self, register: u8, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = register.wrapping_sub(value);
        self.status.set_carry(register >= value);
        self.update_zero_and_negative_flags(result);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
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

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a ^= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
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
                let addr = self.get_operand_address(mode);
                self.program_counter = addr;
            }
            AddressingMode::Indirect => {
                let ptr = self.get_operand_address(mode);
                let addr = self.mem_read_u16(ptr);

                self.program_counter = addr;
            }
            _ => panic!("mode {:?} is not supported", mode),
        }
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.stack_push_u16(self.program_counter + 2);
        self.program_counter = addr;
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(value);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(value);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(value);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = value >> 1;
        let carry = value & 0x01 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a |= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    fn php(&mut self) {
        self.stack_push(self.status.into());
    }

    fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        self.status = ProcessorStatus::from(self.stack_pop());
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = (value << 1) | if self.status.carry { 1 } else { 0 };
        let carry = value & 0x80 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let value = if *mode == AddressingMode::NoneAddressing {
            self.register_a
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_read(addr)
        };

        let result = (value >> 1) | if self.status.carry { 0x80 } else { 0 };
        let carry = value & 0x01 != 0;

        if *mode == AddressingMode::NoneAddressing {
            self.register_a = result;
        } else {
            let addr = self.get_operand_address(mode);
            self.mem_write(addr, result);
        }
        self.status.set_carry(carry);
        self.update_zero_and_negative_flags(result);
    }

    fn rti(&mut self) {
        self.status = self.stack_pop().into();
        self.program_counter = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16();
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
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
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
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

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => {
                let address = self.mem_read(self.program_counter) as u16;
                address.wrapping_add(self.register_x as u16)
            }
            AddressingMode::ZeroPageY => {
                let address = self.mem_read(self.program_counter) as u16;
                address.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::Indirect => {
                let ptr = self.mem_read_u16(self.program_counter);
                self.mem_read_u16(ptr)
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);

                let ptr = (base as u16).wrapping_add(self.register_x as u16);
                self.mem_read_u16(ptr)
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);

                let deref_base = self.mem_read_u16(base as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", mode),
        }
    }
}

impl<M: Mem> fmt::Display for CPU<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let code = self.mem_read(self.program_counter);
        let op = OPCODE_MAP.get(&code).unwrap();

        let begin = self.program_counter;
        let mut hex_dump = vec![];
        hex_dump.push(code);

        let (mem_addr, stored_value) = match op.addr_mode {
            AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
            _ => {
                let pc = begin + 1;

                let addr = match op.addr_mode {
                    AddressingMode::ZeroPage => self.mem_read(pc) as u16,
                    AddressingMode::ZeroPageX => {
                        let address = self.mem_read(pc) as u16;
                        address.wrapping_add(self.register_x as u16)
                    }
                    AddressingMode::ZeroPageY => {
                        let address = self.mem_read(pc) as u16;
                        address.wrapping_add(self.register_y as u16)
                    }
                    AddressingMode::Absolute => self.mem_read_u16(pc),
                    AddressingMode::AbsoluteX => {
                        let base = self.mem_read_u16(pc);
                        base.wrapping_add(self.register_x as u16)
                    }
                    AddressingMode::AbsoluteY => {
                        let base = self.mem_read_u16(pc);
                        base.wrapping_add(self.register_y as u16)
                    }
                    AddressingMode::Indirect => {
                        let ptr = self.mem_read_u16(pc);
                        self.mem_read_u16(ptr)
                    }
                    AddressingMode::IndirectX => {
                        let base = self.mem_read(pc);

                        let ptr = (base as u16).wrapping_add(self.register_x as u16);
                        self.mem_read_u16(ptr)
                    }
                    AddressingMode::IndirectY => {
                        let base = self.mem_read(pc);

                        let deref_base = self.mem_read_u16(base as u16);
                        deref_base.wrapping_add(self.register_y as u16)
                    }
                    _ => 0,
                };

                (addr, self.mem_read(addr))
            }
        };

        let tmp = match op.size {
            1 => match op.code {
                0x0A | 0x4A | 0x2A | 0x6A => format!("A "),
                _ => String::from(""),
            },
            2 => {
                let address = self.mem_read(begin + 1);
                hex_dump.push(address);

                match op.addr_mode {
                    AddressingMode::Immediate => format!("#${:02x}", address),
                    AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::ZeroPageX => format!(
                        "${:02x},X @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::ZeroPageY => format!(
                        "${:02x},Y @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::IndirectX => format!(
                        "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                        address,
                        (address.wrapping_add(self.register_x)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::IndirectY => format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        address,
                        (mem_addr.wrapping_sub(self.register_y as u16)),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::NoneAddressing => {
                        // assuming local jumps: BNE, BVS, etc....
                        let address: usize =
                            (begin as usize + 2).wrapping_add((address as i8) as usize);
                        format!("${:04x}", address)
                    }

                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                        op.addr_mode, op.code
                    ),
                }
            }
            3 => {
                let address_lo = self.mem_read(begin + 1);
                let address_hi = self.mem_read(begin + 2);
                hex_dump.push(address_lo);
                hex_dump.push(address_hi);

                let address = self.mem_read_u16(begin + 1);

                match op.addr_mode {
                    AddressingMode::NoneAddressing => {
                        if op.code == 0x6c {
                            //jmp indirect
                            let jmp_addr = if address & 0x00FF == 0x00FF {
                                let lo = self.mem_read(address);
                                let hi = self.mem_read(address & 0xFF00);
                                (hi as u16) << 8 | (lo as u16)
                            } else {
                                self.mem_read_u16(address)
                            };

                            // let jmp_addr = cpu.mem_read_u16(address);
                            format!("(${:04x}) = {:04x}", address, jmp_addr)
                        } else {
                            format!("${:04x}", address)
                        }
                    }
                    AddressingMode::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::AbsoluteX => format!(
                        "${:04x},X @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::AbsoluteY => format!(
                        "${:04x},Y @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                        op.addr_mode, op.code
                    ),
                }
            }
            _ => String::from(""),
        };

        let hex_str = hex_dump
            .iter()
            .map(|z| format!("{:02x}", z))
            .collect::<Vec<String>>()
            .join(" ");
        let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, op.name, tmp)
            .trim()
            .to_string();

        let status: u8 = self.status.into();
        let log = format!(
            "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
            asm_str, self.register_a, self.register_x, self.register_y, status, self.stack_pointer
        )
        .to_ascii_uppercase();

        write!(f, "{}", log)
    }
}
