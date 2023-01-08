use anyhow::{Context as _, Result};

use self::opcode::{Code, Mode, OpCode};

mod opcode;
mod test;

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack: u8,
    pub status: u8,
    pub program_counter: u16,
    pub memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack: 0xFD,
            status: 0b0010_0100,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack = 0xFD;
        self.status = 0b0010_0100;

        self.program_counter = self.read_mem_u16(0xFFFC);
    }

    pub fn load(&mut self, prog: &[u8]) {
        // self.memory[0x8000..(0x8000 + prog.len())].clone_from_slice(&prog[..]);
        // self.write_mem_u16(0xFFFC, 0x8000);
        self.memory[0x0600..(0x0600 + prog.len())].clone_from_slice(&prog[..]);
        self.write_mem_u16(0xFFFC, 0x0600);
    }

    pub fn load_and_run(&mut self, prog: &[u8]) -> Result<()> {
        self.load(prog);
        self.reset();
        self.run()
    }

    pub fn run(&mut self) -> Result<()> {
        self.run_with_callback(|_| {})
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);

            let opcode = OpCode::from(self.read_mem(self.program_counter)).with_context(|| "")?;
            self.program_counter += 1;

            match opcode {
                OpCode(Code::ADC, mode, _) => {
                    self.adc(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::AND, mode, _) => {
                    self.and(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::ASL, mode, _) => {
                    self.asl(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::BCC, Mode::Relative, _) => {
                    self.bcc();
                }
                OpCode(Code::BCS, Mode::Relative, _) => {
                    self.bcs();
                }
                OpCode(Code::BEQ, Mode::Relative, _) => {
                    self.beq();
                }
                OpCode(Code::BIT, mode, _) => {
                    self.bit(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::BMI, Mode::Relative, _) => {
                    self.bmi();
                }
                OpCode(Code::BNE, Mode::Relative, _) => {
                    self.bne();
                }
                OpCode(Code::BPL, Mode::Relative, _) => {
                    self.bpl();
                }
                OpCode(Code::BVC, Mode::Relative, _) => {
                    self.bvc();
                }
                OpCode(Code::BVS, Mode::Relative, _) => {
                    self.bvs();
                }
                OpCode(Code::CLC, mode, _) => {
                    self.clc();
                    self.update_program_counter(&mode);
                }
                OpCode(Code::CLD, mode, _) => {
                    self.cld();
                    self.update_program_counter(&mode);
                }
                OpCode(Code::CLI, mode, _) => {
                    self.cli();
                    self.update_program_counter(&mode);
                }
                OpCode(Code::CLV, mode, _) => {
                    self.clv();
                    self.update_program_counter(&mode);
                }
                OpCode(Code::CMP, mode, _) => {
                    self.cmp(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::CPX, mode, _) => {
                    self.cpx(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::CPY, mode, _) => {
                    self.cpy(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::DEC, mode, _) => {
                    self.dec(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::DEX, Mode::Implied, _) => {
                    self.dex();
                }
                OpCode(Code::DEY, Mode::Implied, _) => {
                    self.dey();
                }
                OpCode(Code::EOR, mode, _) => {
                    self.eor(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::INC, mode, _) => {
                    self.inc(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::INX, mode, _) => {
                    self.inx();
                    self.update_program_counter(&mode);
                }
                OpCode(Code::INY, mode, _) => {
                    self.iny();
                    self.update_program_counter(&mode);
                }
                OpCode(Code::JMP, mode, _) => {
                    self.jmp(&mode);
                }
                OpCode(Code::JSR, Mode::Absolute, _) => {
                    self.jsr();
                }
                OpCode(Code::LDA, mode, _) => {
                    self.lda(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::LDX, mode, _) => {
                    self.ldx(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::LDY, mode, _) => {
                    self.ldy(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::LSR, mode, _) => {
                    self.lsr(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::ORA, mode, _) => {
                    self.ora(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::PHA, Mode::Implied, _) => {
                    self.pha();
                }
                OpCode(Code::PHP, Mode::Implied, _) => {
                    self.php();
                }
                OpCode(Code::PLA, Mode::Implied, _) => {
                    self.pla();
                }
                OpCode(Code::PLP, Mode::Implied, _) => {
                    self.plp();
                }
                OpCode(Code::ROL, mode, _) => {
                    self.rol(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::ROR, mode, _) => {
                    self.ror(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::RTI, Mode::Implied, _) => {
                    self.rti();
                }
                OpCode(Code::RTS, Mode::Implied, _) => {
                    self.rts();
                }
                OpCode(Code::SBC, mode, _) => {
                    self.sbc(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::SEC, Mode::Implied, _) => {
                    self.sec();
                }
                OpCode(Code::SED, Mode::Implied, _) => {
                    self.sed();
                }
                OpCode(Code::SEI, Mode::Implied, _) => {
                    self.sei();
                }
                OpCode(Code::STA, mode, _) => {
                    self.sta(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::STX, mode, _) => {
                    self.stx(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::STY, mode, _) => {
                    self.sty(&mode);
                    self.update_program_counter(&mode);
                }
                OpCode(Code::TAX, Mode::Implied, _) => {
                    self.tax();
                }
                OpCode(Code::TAY, Mode::Implied, _) => {
                    self.tay();
                }
                OpCode(Code::TSX, Mode::Implied, _) => {
                    self.tsx();
                }
                OpCode(Code::TXA, Mode::Implied, _) => {
                    self.txa();
                }
                OpCode(Code::TXS, Mode::Implied, _) => {
                    self.txs();
                }
                OpCode(Code::TYA, Mode::Implied, _) => {
                    self.tya();
                }
                OpCode(Code::NOP, _, _) => continue,
                OpCode(Code::BRK, _, _) => return Ok(()),
                _ => todo!(),
            }
        }
    }

    // OpCode impl

    fn adc(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_value = self.read_mem(addr);

        let a = self.register_a.clone();
        let c = self.status & 0b0000_0001;
        let sum = a as u16 + mem_value as u16 + c as u16;

        // carry flag
        if sum > 0xFF {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        // overflow flag
        let result = sum as u8;
        if (mem_value ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }

        // set accumulator
        self.register_a = result;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn and(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_val = self.read_mem(addr);

        let val = self.register_a & mem_val;

        self.register_a = val;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, mode: &Mode) {
        let mut val = {
            if *mode == Mode::Accumulator {
                self.register_a
            } else {
                let addr = self.get_address(mode);
                self.read_mem(addr)
            }
        };

        if val >> 7 == 1 {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        val = val << 1;

        if *mode == Mode::Accumulator {
            self.register_a = val;
        } else {
            let addr = self.get_address(mode);
            self.write_mem(addr, val);
        }

        self.update_zero_and_negative_flags(val);
    }

    fn bcc(&mut self) {
        self.branch(self.status & 0b0000_0001 == 0);
    }

    fn bcs(&mut self) {
        self.branch(self.status & 0b0000_0001 != 0);
    }

    fn beq(&mut self) {
        self.branch(self.status & 0b0000_0010 != 0);
    }

    fn bit(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_value = self.read_mem(addr);

        // V
        if (mem_value & 0b0100_0000) >> 6 == 1 {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }

        // N
        if mem_value >> 7 == 1 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }

        // Z = A & M
        if self.register_a & mem_value == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
    }

    fn bmi(&mut self) {
        self.branch(self.status & 0b1000_0000 != 0);
    }

    fn bne(&mut self) {
        self.branch(self.status & 0b0000_0010 == 0);
    }

    fn bpl(&mut self) {
        self.branch(self.status & 0b1000_0000 == 0);
    }

    fn bvc(&mut self) {
        self.branch(self.status & 0b0100_0000 == 0)
    }

    fn bvs(&mut self) {
        self.branch(self.status & 0b0100_0000 != 0)
    }

    fn clc(&mut self) {
        self.status = self.status & 0b1111_1110;
    }

    fn cld(&mut self) {
        self.status = self.status & 0b1111_0111;
    }

    fn cli(&mut self) {
        self.status = self.status & 0b1111_1011;
    }

    fn clv(&mut self) {
        self.status = self.status & 0b1011_1111;
    }

    fn cmp(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_val = self.read_mem(addr);

        if self.register_a >= mem_val {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        self.update_zero_and_negative_flags(self.register_a.wrapping_sub(mem_val));
    }

    fn cpx(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_val = self.read_mem(addr);

        if self.register_x >= mem_val {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        self.update_zero_and_negative_flags(self.register_x.wrapping_sub(mem_val));
    }

    fn cpy(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_val = self.read_mem(addr);

        if self.register_y >= mem_val {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        self.update_zero_and_negative_flags(self.register_y.wrapping_sub(mem_val));
    }

    fn dec(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_val = self.read_mem(addr);

        let value = mem_val.wrapping_sub(1);

        self.write_mem(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        if self.register_x == 0x00 {
            self.register_x = 0xff;
        } else {
            self.register_x -= 1;
        }
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        if self.register_y == 0x00 {
            self.register_y = 0xff;
        } else {
            self.register_y -= 1;
        }
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let value = self.read_mem(addr);
        self.register_a ^= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn inc(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_val = self.read_mem(addr);

        let value = mem_val.wrapping_add(1);

        self.write_mem(addr, value);
        self.update_zero_and_negative_flags(value);
    }

    fn inx(&mut self) {
        let value = self.register_x.wrapping_add(1);
        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        let value = self.register_y.wrapping_add(1);
        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn jmp(&mut self, mode: &Mode) {
        self.program_counter = self.get_address(mode);
    }

    fn jsr(&mut self) {
        self.push_stack_u16(self.program_counter + 2 - 1);
        self.program_counter = self.read_mem_u16(self.program_counter);
    }

    fn lda(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let val = self.read_mem(addr);
        self.register_a = val;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let val = self.read_mem(addr);
        self.register_x = val;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let val = self.read_mem(addr);
        self.register_y = val;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn lsr(&mut self, mode: &Mode) {
        let mut value = if mode == &Mode::Accumulator {
            self.register_a
        } else {
            let addr = self.get_address(mode);
            self.read_mem(addr)
        };

        if value & 1 == 1 {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
        value = value >> 1;

        if mode == &Mode::Accumulator {
            self.register_a = value;
        } else {
            let addr = self.get_address(mode);
            self.write_mem(addr, value);
        }

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ora(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let value = self.read_mem(addr);
        self.register_a = self.register_a | value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn pha(&mut self) {
        self.push_stack(self.register_a);
    }

    fn php(&mut self) {
        let flag = self.status | 0b0011_0000;
        self.push_stack(flag);
    }

    fn pla(&mut self) {
        self.register_a = self.pop_stack();
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        self.status = self.pop_stack();
        self.status = self.status & 0b1110_1111;
        self.status = self.status | 0b0010_0000;
    }

    fn rol(&mut self, mode: &Mode) {
        let mut value = if mode == &Mode::Accumulator {
            self.register_a
        } else {
            let addr = self.get_address(mode);
            self.read_mem(addr)
        };
        let current_carry = self.status & 0b0000_0001;

        if value >> 7 == 1 {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        value = value << 1;

        if current_carry == 1 {
            value = value | 0b0000_0001;
        }

        if mode == &Mode::Accumulator {
            self.register_a = value;
        } else {
            let addr = self.get_address(mode);
            self.write_mem(addr, value);
        }
        self.update_zero_and_negative_flags(value);
    }

    fn ror(&mut self, mode: &Mode) {
        let mut value = if mode == &Mode::Accumulator {
            self.register_a
        } else {
            let addr = self.get_address(mode);
            self.read_mem(addr)
        };
        let current_carry = self.status & 0b0000_0001;

        if value & 0b0000_0001 == 1 {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        value = value >> 1;

        if current_carry == 1 {
            value = value | 0b1000_0000;
        }

        if mode == &Mode::Accumulator {
            self.register_a = value;
        } else {
            let addr = self.get_address(mode);
            self.write_mem(addr, value);
        }
        self.update_zero_and_negative_flags(value);
    }

    fn rti(&mut self) {
        self.status = self.pop_stack();
        self.status = self.status & 0b1110_1111;
        self.status = self.status | 0b0010_0000;
        self.program_counter = self.pop_stack_u16();
    }

    fn rts(&mut self) {
        self.program_counter = self.pop_stack_u16() + 1;
    }

    fn sbc(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        let mem_value = self.read_mem(addr);

        let a = self.register_a;
        let b = (mem_value as i8).wrapping_neg().wrapping_sub(1) as u8;
        let c = self.status & 0b0000_0001;

        // A - B - (1 - C) = A + (-B) - 1 + C = A + (-B - 1) + C
        let sum = a as u16
            // (-B - 1)
            + b as u16
            + c as u16;

        // carry flag
        if sum > 0xFF {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }

        // overflow flag
        let result = sum as u8;
        if (b ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }

        // set accumulator
        self.register_a = result;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sec(&mut self) {
        self.status = self.status | 0b0000_0001
    }

    fn sed(&mut self) {
        self.status = self.status | 0b0000_1000
    }

    fn sei(&mut self) {
        self.status = self.status | 0b0000_0100
    }

    fn sta(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        self.write_mem(addr, self.register_a);
    }

    fn stx(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        self.write_mem(addr, self.register_x);
    }

    fn sty(&mut self, mode: &Mode) {
        let addr = self.get_address(mode);
        self.write_mem(addr, self.register_y);
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
        self.register_x = self.stack;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn txs(&mut self) {
        self.stack = self.register_x;
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    // utils

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_mem(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn read_mem_u16(&self, pos: u16) -> u16 {
        let lo = self.read_mem(pos);
        let hi = self.read_mem(pos + 1);
        u16::from_le_bytes([lo, hi])
    }

    pub fn write_mem_u16(&mut self, pos: u16, val: u16) {
        let [lo, hi] = val.to_le_bytes();
        self.write_mem(pos, lo);
        self.write_mem(pos + 1, hi);
    }

    fn push_stack(&mut self, data: u8) {
        let addr = 0x0100 + (self.stack as u16);
        self.write_mem(addr, data);
        self.stack = self.stack.wrapping_sub(1);
    }

    fn pop_stack(&mut self) -> u8 {
        self.stack = self.stack.wrapping_add(1);
        let addr = 0x0100 + (self.stack as u16);
        self.read_mem(addr)
    }

    fn push_stack_u16(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.push_stack(hi);
        self.push_stack(lo);
    }

    fn pop_stack_u16(&mut self) -> u16 {
        let lo = self.pop_stack();
        let hi = self.pop_stack();
        u16::from_le_bytes([lo, hi])
    }

    fn get_address(&self, mode: &Mode) -> u16 {
        match mode {
            Mode::Immediate => self.program_counter,
            Mode::ZeroPage => self.read_mem(self.program_counter) as u16,
            Mode::Absolute => self.read_mem_u16(self.program_counter),
            Mode::ZeroPageX => {
                let pos = self.read_mem(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;

                addr
            }
            Mode::ZeroPageY => {
                let pos = self.read_mem(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;

                addr
            }
            Mode::AbsoluteX => {
                let base = self.read_mem_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);

                addr
            }
            Mode::AbsoluteY => {
                let base = self.read_mem_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);

                addr
            }
            Mode::Indirect => {
                let addr = self.read_mem_u16(self.program_counter);
                if addr & 0x00ff == 0x00ff {
                    let lo = self.read_mem(addr);
                    let hi = self.read_mem(addr & 0xff00);
                    u16::from_le_bytes([lo, hi])
                } else {
                    self.read_mem_u16(addr)
                }
            }
            Mode::IndirectX => {
                let base = self.read_mem(self.program_counter);

                let ptr = base.wrapping_add(self.register_x);
                let lo = self.read_mem(ptr as u16);
                let hi = self.read_mem(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            }
            Mode::IndirectY => {
                let base = self.read_mem(self.program_counter);

                let lo = self.read_mem(base as u16);
                let hi = self.read_mem((base as u8).wrapping_add(1) as u16);
                let deref_base = u16::from_le_bytes([lo, hi]);
                let deref = deref_base.wrapping_add(self.register_y as u16);

                deref
            }
            _ => panic!("invalid mode"),
        }
    }

    fn update_program_counter(&mut self, mode: &Mode) {
        match mode {
            Mode::Accumulator => (),
            Mode::Implied => (),
            Mode::Relative => (),
            Mode::Immediate => self.program_counter += 1,
            Mode::ZeroPage => self.program_counter += 1,
            Mode::ZeroPageX => self.program_counter += 1,
            Mode::ZeroPageY => self.program_counter += 1,
            Mode::Absolute => self.program_counter += 2,
            Mode::AbsoluteX => self.program_counter += 2,
            Mode::AbsoluteY => self.program_counter += 2,
            Mode::Indirect => self.program_counter += 1,
            Mode::IndirectX => self.program_counter += 1,
            Mode::IndirectY => self.program_counter += 1,
        }
    }

    fn branch(&mut self, cond: bool) {
        if cond {
            // -128~127
            let jump = self.read_mem(self.program_counter) as i8;
            let jump_addr = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);
            self.program_counter = jump_addr;
        } else {
            self.program_counter += 1;
        }
    }
}
