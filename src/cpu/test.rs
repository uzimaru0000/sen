#[cfg(test)]
mod test {
    use crate::cpu::{
        opcode::{Code, Mode, OpCode},
        CPU,
    };

    #[derive(Default)]
    struct OpCodeTestCase {
        name: String,
        reg_a: Option<u8>,
        reg_x: Option<u8>,
        reg_y: Option<u8>,
        mem: Option<(u16, u8)>,
        prog_cnt: Option<u16>,
        st: Option<u8>,
        prog: Vec<u8>,
        exp_st: Option<u8>,
        exp_a: Option<u8>,
        exp_x: Option<u8>,
        exp_y: Option<u8>,
        exp_mem: Option<(u16, u8)>,
    }
    fn opcode_test(test_cases: &[OpCodeTestCase]) {
        for test_case in test_cases {
            let mut cpu = CPU::new();
            cpu.load(&test_case.prog);
            cpu.reset();

            cpu.register_a = test_case.reg_a.unwrap_or(cpu.register_a);
            cpu.register_x = test_case.reg_x.unwrap_or(cpu.register_x);
            cpu.register_y = test_case.reg_y.unwrap_or(cpu.register_y);
            cpu.program_counter = test_case.prog_cnt.unwrap_or(cpu.program_counter);
            cpu.status = test_case.st.unwrap_or(cpu.status);
            if let Some((addr, val)) = test_case.mem {
                cpu.write_mem(addr, val);
            }

            cpu.run().unwrap();

            if let Some(exp_a) = test_case.exp_a {
                assert_eq!(cpu.register_a, exp_a, "{} is failed", test_case.name);
            }
            if let Some(exp_x) = test_case.exp_x {
                assert_eq!(cpu.register_x, exp_x, "{} is failed", test_case.name);
            }
            if let Some(exp_y) = test_case.exp_y {
                assert_eq!(cpu.register_y, exp_y, "{} is failed", test_case.name);
            }
            if let Some(exp_st) = test_case.exp_st {
                assert_eq!(cpu.status, exp_st, "{} is failed", test_case.name);
            }
            if let Some((addr, mem_val)) = test_case.exp_mem {
                assert_eq!(cpu.read_mem(addr), mem_val, "{} is failed", test_case.name);
            }
        }
    }

    #[test]
    fn test_get_address() {
        let test_case = vec![
            (Mode::Immediate, 0x8000),
            (Mode::ZeroPage, 0x00A9),
            (Mode::Absolute, 0x05A9),
            (Mode::ZeroPageX, 0x00B3),
            (Mode::ZeroPageY, 0x00B3),
            (Mode::AbsoluteX, 0x05B3),
            (Mode::AbsoluteY, 0x05B3),
            (Mode::IndirectX, 0x0000),
            (Mode::IndirectY, 0x000a),
        ];

        for (mode, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&[0xA9, 0x05, 0x00]);
            cpu.reset();

            cpu.register_x = 10;
            cpu.register_y = 10;

            let addr = cpu.get_address(&mode);
            assert_eq!(addr, exp, "Failed in the {:?}.", mode);
        }
    }

    #[test]
    fn test_tax() {
        let test_case = vec![
            (10, vec![0xAA, 0x00], 10, 0b00),
            (0, vec![0xAA, 0x00], 0, 0b10),
        ];

        for (val_a, prog, exp, zero_flag) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();
            cpu.register_a = val_a;
            cpu.run().unwrap();
            assert_eq!(cpu.register_x, exp);
            assert!(cpu.status & 0b0000_0010 == zero_flag);
        }
    }

    #[test]
    fn test_sta() {
        let test_case = vec![(
            0x50,
            vec![
                OpCode(Code::STA, Mode::ZeroPage, 3).to_bytes(),
                0x10,
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            0x0010,
            0x50,
        )];

        for (val_a, prog, addr, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.register_a = val_a;

            cpu.run().unwrap();

            assert_eq!(cpu.read_mem(addr), exp);
        }
    }

    #[test]
    fn test_adc() {
        let test_case = vec![(
            0b0000_0000,
            0x10,
            vec![
                OpCode(Code::ADC, Mode::Immediate, 2).to_bytes(),
                0x10,
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            0x20,
            0b00,
        )];

        for (st, val_a, prog, exp, zero_flag) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;
            cpu.register_a = val_a;

            cpu.run().unwrap();
            assert_eq!(cpu.register_a, exp);
            assert_eq!(cpu.status & 0b0000_0010, zero_flag);
        }
    }

    #[test]
    fn test_and() {
        let test_case = vec![
            (
                0b1111_1111,
                vec![
                    OpCode(Code::AND, Mode::Immediate, 2).to_bytes(),
                    0b0101_0101,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                0b0101_0101,
                0b00,
            ),
            (
                0b1010_1010,
                vec![
                    OpCode(Code::AND, Mode::Immediate, 2).to_bytes(),
                    0b0101_0101,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                0b0000_0000,
                0b10,
            ),
        ];

        for (val_a, prog, exp, zero_flag) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.register_a = val_a;

            cpu.run().unwrap();
            assert_eq!(cpu.register_a, exp);
            assert_eq!(cpu.status & 0b0000_0010, zero_flag);
        }
    }

    #[test]
    fn test_asl() {
        let test_case = vec![
            (
                0b0000_0001,
                vec![
                    OpCode(Code::ASL, Mode::Accumulator, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                0b0000_0010,
                0b00,
            ),
            (
                0b1000_0000,
                vec![
                    OpCode(Code::ASL, Mode::Accumulator, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                0b0000_0000,
                0b10,
            ),
        ];

        for (val_a, prog, exp, zero_flag) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.register_a = val_a;

            cpu.run().unwrap();
            assert_eq!(cpu.register_a, exp);
            assert_eq!(cpu.status & 0b0000_0010, zero_flag);
        }
    }

    #[test]
    fn test_bcc() {
        let test_case = vec![
            (0b0000_0000, vec![0x10], 0x8011),
            (0b0000_0001, vec![0x10], 0x8001),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.bcc();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_bcs() {
        let test_case = vec![
            (0b0000_0000, vec![0x10], 0x8001),
            (0b0000_0001, vec![0x10], 0x8011),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.bcs();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_beq() {
        let test_case = vec![
            (0b0000_0010, vec![0x10], 0x8011),
            (0b0000_0000, vec![0x10], 0x8001),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.beq();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_bit() {
        opcode_test(&[
            OpCodeTestCase {
                name: "V, N, Z がすべて 1".to_string(),
                st: Some(0b0000_0000),
                reg_a: Some(0b0000_0000),
                prog: vec![
                    OpCode(Code::BIT, Mode::Absolute, 4).to_bytes(),
                    0x04,
                    0x80,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                    0b1111_1111,
                ],
                exp_st: Some(0b1100_0010),
                ..Default::default()
            },
            OpCodeTestCase {
                name: "V, N, Z がすべて 0".to_string(),
                st: Some(0b0000_0000),
                reg_a: Some(0b0000_1000),
                prog: vec![
                    OpCode(Code::BIT, Mode::Absolute, 4).to_bytes(),
                    0x04,
                    0x80,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                    0b0000_1000,
                ],
                exp_st: Some(0b0000_0000),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_bmi() {
        let test_case = vec![
            (0b1000_0000, vec![0x10], 0x8011),
            (0b0000_0000, vec![0x10], 0x8001),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.bmi();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_bne() {
        let test_case = vec![
            (0b0000_0000, vec![0x10], 0x8011),
            (0b0000_0010, vec![0x10], 0x8001),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.bne();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_bpl() {
        let test_case = vec![
            (0b0000_0000, vec![0x10], 0x8011),
            (0b1000_0010, vec![0x10], 0x8001),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.bpl();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_bvc() {
        let test_case = vec![
            (0b0000_0000, vec![0x10], 0x8011),
            (0b0100_0010, vec![0x10], 0x8001),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.bvc();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_bvs() {
        let test_case = vec![
            (0b0100_0000, vec![0x10], 0x8011),
            (0b0000_0010, vec![0x10], 0x8001),
        ];

        for (st, prog, exp) in test_case {
            let mut cpu = CPU::new();
            cpu.load(&prog);
            cpu.reset();

            cpu.status = st;

            cpu.bvs();
            assert_eq!(cpu.program_counter, exp);
        }
    }

    #[test]
    fn test_clc() {
        opcode_test(&[OpCodeTestCase {
            name: "carry flag がクリアされる".to_string(),
            st: Some(0b0000_0001),
            prog: vec![
                OpCode(Code::CLC, Mode::Implied, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            exp_st: Some(0b0000_0000),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_cld() {
        opcode_test(&[OpCodeTestCase {
            name: "Decimal Mode がクリアされる".to_string(),
            st: Some(0b0000_1000),
            prog: vec![
                OpCode(Code::CLD, Mode::Implied, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            exp_st: Some(0b0000_0000),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_cli() {
        opcode_test(&[OpCodeTestCase {
            name: "interrupt flag がクリアされる".to_string(),
            st: Some(0b0000_0100),
            prog: vec![
                OpCode(Code::CLI, Mode::Implied, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            exp_st: Some(0b0000_0000),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_clv() {
        opcode_test(&[OpCodeTestCase {
            name: "overflow flag がクリアされる".to_string(),
            st: Some(0b0100_0000),
            prog: vec![
                OpCode(Code::CLV, Mode::Implied, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            exp_st: Some(0b0000_0000),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_cmp() {
        opcode_test(&[
            OpCodeTestCase {
                name: "A >= M のとき zero flag と carry flag が立つ".to_string(),
                reg_a: Some(0x50),
                prog: vec![
                    OpCode(Code::CMP, Mode::Immediate, 2).to_bytes(),
                    0x50,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_st: Some(0b0010_0111),
                ..Default::default()
            },
            OpCodeTestCase {
                name: "A > M のとき carry flag が立つ".to_string(),
                reg_a: Some(0x51),
                prog: vec![
                    OpCode(Code::CMP, Mode::Immediate, 2).to_bytes(),
                    0x50,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_st: Some(0b0010_0101),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_cpx() {
        opcode_test(&[
            OpCodeTestCase {
                name: "X >= M のとき zero flag と carry flag が立つ".to_string(),
                reg_x: Some(0x50),
                prog: vec![
                    OpCode(Code::CPX, Mode::Immediate, 2).to_bytes(),
                    0x50,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_st: Some(0b0010_0111),
                ..Default::default()
            },
            OpCodeTestCase {
                name: "X > M のとき carry flag が立つ".to_string(),
                reg_x: Some(0x51),
                prog: vec![
                    OpCode(Code::CPX, Mode::Immediate, 2).to_bytes(),
                    0x50,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_st: Some(0b0010_0101),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_cpy() {
        opcode_test(&[
            OpCodeTestCase {
                name: "Y >= M のとき zero flag と carry flag が立つ".to_string(),
                reg_y: Some(0x50),
                prog: vec![
                    OpCode(Code::CPY, Mode::Immediate, 2).to_bytes(),
                    0x50,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_st: Some(0b0010_0111),
                ..Default::default()
            },
            OpCodeTestCase {
                name: "Y > M のとき carry flag が立つ".to_string(),
                reg_y: Some(0x51),
                prog: vec![
                    OpCode(Code::CPY, Mode::Immediate, 2).to_bytes(),
                    0x50,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_st: Some(0b0010_0101),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_dec() {
        opcode_test(&[OpCodeTestCase {
            name: "メモリの値がデクリメントされる".to_string(),
            prog: vec![
                OpCode(Code::DEC, Mode::Absolute, 2).to_bytes(),
                0x04,
                0x80,
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                0x01,
            ],
            exp_mem: Some((0x8004, 0x00)),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_dex() {
        opcode_test(&[OpCodeTestCase {
            name: "reg_x の値がデクリメントされる".to_string(),
            reg_x: Some(0x01),
            prog: vec![
                OpCode(Code::DEX, Mode::Implied, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            exp_x: Some(0x00),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_dey() {
        opcode_test(&[OpCodeTestCase {
            name: "reg_y の値がデクリメントされる".to_string(),
            reg_y: Some(0x01),
            prog: vec![
                OpCode(Code::DEY, Mode::Implied, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            exp_y: Some(0x00),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_eor() {
        opcode_test(&[OpCodeTestCase {
            name: "reg_a の値が ExOR される".to_string(),
            reg_a: Some(0b1111_1010),
            prog: vec![
                OpCode(Code::EOR, Mode::Immediate, 2).to_bytes(),
                0b1111_0101,
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            exp_a: Some(0b0000_1111),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_inc() {
        opcode_test(&[
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::INC, Mode::Absolute, 2).to_bytes(),
                    0x04,
                    0x80,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                    0x10,
                ],
                exp_mem: Some((0x8004, 0x11)),
                ..Default::default()
            },
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::INC, Mode::Absolute, 2).to_bytes(),
                    0x04,
                    0x80,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                    0xFF,
                ],
                exp_mem: Some((0x8004, 0x00)),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_inx() {
        opcode_test(&[
            OpCodeTestCase {
                reg_x: Some(10),
                prog: vec![
                    OpCode(Code::INX, Mode::Implied, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_x: Some(11),
                ..Default::default()
            },
            OpCodeTestCase {
                reg_x: Some(0),
                prog: vec![
                    OpCode(Code::INX, Mode::Implied, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_x: Some(1),
                ..Default::default()
            },
            OpCodeTestCase {
                reg_x: Some(0xFF),
                prog: vec![
                    OpCode(Code::INX, Mode::Implied, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_x: Some(0),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_iny() {
        opcode_test(&[
            OpCodeTestCase {
                reg_y: Some(10),
                prog: vec![
                    OpCode(Code::INY, Mode::Implied, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_y: Some(11),
                ..Default::default()
            },
            OpCodeTestCase {
                reg_y: Some(0),
                prog: vec![
                    OpCode(Code::INY, Mode::Implied, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_y: Some(1),
                ..Default::default()
            },
            OpCodeTestCase {
                reg_y: Some(0xFF),
                prog: vec![
                    OpCode(Code::INY, Mode::Implied, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_y: Some(0),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_jmp() {
        let mut cpu = CPU::new();
        cpu.load(&[0x50, 0x80]);
        cpu.reset();

        cpu.jmp(&Mode::Absolute);

        assert_eq!(cpu.program_counter, 0x8050);
    }

    #[test]
    fn test_lda() {
        opcode_test(&[
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LDA, Mode::Immediate, 2).to_bytes(),
                    0x05,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_a: Some(0x05),
                ..Default::default()
            },
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LDA, Mode::ZeroPage, 2).to_bytes(),
                    0x10,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                mem: Some((0x0010, 0x55)),
                exp_a: Some(0x55),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_ldx() {
        opcode_test(&[
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LDX, Mode::Immediate, 2).to_bytes(),
                    0x05,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_x: Some(0x05),
                ..Default::default()
            },
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LDX, Mode::ZeroPage, 2).to_bytes(),
                    0x10,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                mem: Some((0x0010, 0x55)),
                exp_x: Some(0x55),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_ldy() {
        opcode_test(&[
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LDY, Mode::Immediate, 2).to_bytes(),
                    0x05,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                exp_y: Some(0x05),
                ..Default::default()
            },
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LDY, Mode::ZeroPage, 2).to_bytes(),
                    0x10,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                mem: Some((0x0010, 0x55)),
                exp_y: Some(0x55),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_lsr() {
        opcode_test(&[
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LSR, Mode::Accumulator, 2).to_bytes(),
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                reg_a: Some(0b0000_0010),
                exp_a: Some(0b0000_0001),
                ..Default::default()
            },
            OpCodeTestCase {
                prog: vec![
                    OpCode(Code::LSR, Mode::ZeroPage, 2).to_bytes(),
                    0x10,
                    OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
                ],
                mem: Some((0x0010, 0b0000_0010)),
                exp_mem: Some((0x0010, 0b0000_0001)),
                ..Default::default()
            },
        ]);
    }

    #[test]
    fn test_ora() {
        opcode_test(&[OpCodeTestCase {
            prog: vec![
                OpCode(Code::ORA, Mode::Immediate, 2).to_bytes(),
                0b1010_1010,
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            reg_a: Some(0b0101_0101),
            exp_a: Some(0b1111_1111),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_pha() {
        opcode_test(&[OpCodeTestCase {
            prog: vec![
                OpCode(Code::PHA, Mode::Implied, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            reg_a: Some(0x50),
            exp_mem: Some((0x01FD, 0x50)),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_rol() {
        opcode_test(&[OpCodeTestCase {
            prog: vec![
                OpCode(Code::ROL, Mode::Accumulator, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            st: Some(0b0000_0001),
            reg_a: Some(0b1000_0000),
            exp_a: Some(0b0000_0001),
            ..Default::default()
        }]);
    }

    #[test]
    fn test_ror() {
        opcode_test(&[OpCodeTestCase {
            prog: vec![
                OpCode(Code::ROR, Mode::Accumulator, 2).to_bytes(),
                OpCode(Code::BRK, Mode::Implied, 7).to_bytes(),
            ],
            st: Some(0b0000_0001),
            reg_a: Some(0b0000_0001),
            exp_a: Some(0b1000_0000),
            ..Default::default()
        }]);
    }
}
