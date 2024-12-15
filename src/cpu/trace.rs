use crate::{
    bus::{Bus, Mem},
    cpu::{addressing_mode::AddressingMode, opecode::OPCODE_MAP, CPU},
};

use super::opecode::{OpCode, UNOFFICIAL_OPCODE};

pub fn trace<M: Mem + Bus>(cpu: &mut CPU<M>) -> String {
    let code = cpu.mem_read(cpu.program_counter);
    let op = OPCODE_MAP.get(&code).unwrap();

    let begin = cpu.program_counter;
    let mut hex_dump = vec![];
    hex_dump.push(code);

    let (mem_addr, stored_value) = match op.addr_mode {
        AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
        _ => {
            let pc = begin + 1;

            let addr = match op.addr_mode {
                AddressingMode::ZeroPage => cpu.mem_read(pc) as u16,
                AddressingMode::ZeroPageX => {
                    let address = cpu.mem_read(pc) as u16;
                    address.wrapping_add(cpu.register_x as u16) & 0xFF
                }
                AddressingMode::ZeroPageY => {
                    let address = cpu.mem_read(pc) as u16;
                    address.wrapping_add(cpu.register_y as u16) & 0xFF
                }
                AddressingMode::Absolute => cpu.mem_read_u16(pc),
                AddressingMode::AbsoluteX => {
                    let base = cpu.mem_read_u16(pc);
                    base.wrapping_add(cpu.register_x as u16)
                }
                AddressingMode::AbsoluteY => {
                    let base = cpu.mem_read_u16(pc);
                    base.wrapping_add(cpu.register_y as u16)
                }
                AddressingMode::Indirect => {
                    if op.name == "JMP" {
                        let ptr = cpu.mem_read_u16(pc);

                        let lo = cpu.mem_read(ptr);
                        let hi = cpu.mem_read(if ptr & 0xFF == 0xFF {
                            ptr & 0xFF00
                        } else {
                            ptr + 1
                        });
                        (hi as u16) << 8 | lo as u16
                    } else {
                        let ptr = cpu.mem_read_u16(pc);
                        cpu.mem_read_u16(ptr)
                    }
                }
                AddressingMode::IndirectX => {
                    let base = cpu.mem_read(pc);

                    let ptr = (base as u16).wrapping_add(cpu.register_x as u16) & 0xFF;
                    let lo = cpu.mem_read(ptr);
                    let hi = cpu.mem_read(ptr.wrapping_add(1) & 0xFF);

                    (hi as u16) << 8 | lo as u16
                }
                AddressingMode::IndirectY => {
                    let base = cpu.mem_read(pc);

                    let lo = cpu.mem_read(base as u16);
                    let hi = cpu.mem_read((base.wrapping_add(1) & 0xFF) as u16);
                    let deref_base = (hi as u16) << 8 | lo as u16;
                    deref_base.wrapping_add(cpu.register_y as u16)
                }
                _ => 0,
            };

            (addr, cpu.mem_read(addr))
        }
    };

    let tmp = match op.size {
        1 => match op.code {
            0x0A | 0x4A | 0x2A | 0x6A => format!("A "),
            _ => String::from(""),
        },
        2 => {
            let address = cpu.mem_read(begin + 1);
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
                    (address.wrapping_add(cpu.register_x)),
                    mem_addr,
                    stored_value
                ),
                AddressingMode::IndirectY => format!(
                    "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    address,
                    (mem_addr.wrapping_sub(cpu.register_y as u16)),
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
            let address_lo = cpu.mem_read(begin + 1);
            let address_hi = cpu.mem_read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.mem_read_u16(begin + 1);

            match op.addr_mode {
                AddressingMode::NoneAddressing => {
                    format!("${:04x}", address)
                }
                AddressingMode::Absolute => {
                    if op.name == "JMP" || op.name == "JSR" {
                        format!("${:04x}", address)
                    } else {
                        format!("${:04x} = {:02x}", address, stored_value)
                    }
                }
                AddressingMode::AbsoluteX => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::AbsoluteY => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::Indirect => {
                    format!("(${:04x}) = {:04x}", address, mem_addr)
                }
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                    op.addr_mode, op.code
                ),
            }
        }
        _ => String::from(""),
    };

    let name = if check_unofficial_opcode(op) {
        format!("*{}", op.name)
    } else {
        op.name.to_string()
    };
    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, name, tmp)
        .trim()
        .to_string();

    let status: u8 = cpu.status.into();
    let cpu_state = format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.register_a, cpu.register_x, cpu.register_y, status, cpu.stack_pointer
    )
    .to_ascii_uppercase();

    let (cpu_cycle, ppu_cycle) = cpu.bus.get_cycles();
    let scanline = cpu.bus.get_scanline();
    let ppu_state = format!("PPU:{:3},{:3} CYC:{}", scanline, ppu_cycle, cpu_cycle);

    format!("{} {}", cpu_state, ppu_state)
}

fn check_unofficial_opcode(op: &OpCode) -> bool {
    UNOFFICIAL_OPCODE.contains(op)
}
