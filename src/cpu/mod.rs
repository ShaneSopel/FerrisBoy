pub mod alu;
pub mod logging;
pub mod microops;
pub mod registers;

use crate::cpu::alu::Alu;
use crate::cpu::logging::opcode_info;
use crate::cpu::microops::MicroOp;
use crate::cpu::registers::{Flags, Reg16, Reg8, Registers};
use crate::interconnect::Interconnect;

pub struct Cpu {
    regs: Registers,

    flags: Flags,

    alu: Alu,

    interrupt: bool,

    interrupt_enable_next: bool,

    //halted: bool,
    inter: Interconnect,

    cycles: u8,
}

impl Cpu {
    pub fn new(inter: Interconnect) -> Cpu {
        let regs = Registers {
            pc: 0x100,
            sp: 0,
            a: 0,
            b: 0,
            f: Flags::new(),
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            // ie: 0,
            // ir: 0,
        };

        Cpu {
            regs,
            flags: Flags {
                z: false,
                h: false,
                n: false,
                c: false,
            },

            alu: Alu::new(),
            inter,
            interrupt: true,
            interrupt_enable_next: true,
            //halted: false,
            cycles: 0,
        }
    }

    pub fn step(&mut self) {
        let pc_before_op = self.regs.pc;

        let opcode = self.fetch8();

        let micro_ops = if opcode == 0xCB {
            let _cb_opcode = self.fetch8();
            //decode CB
            self.cb_decode(opcode)
        } else {
            let (mnemonic, bytes, cycles) = opcode_info(opcode);

            let mut instr_bytes = vec![opcode];
            if bytes > 1 {
                for i in 1..bytes {
                    instr_bytes.push(self.inter.read_byte(pc_before_op + i as u16));
                }
            }

            println!(
                "PC: {:#06X} | Opcode: {:#04X} | Mnemonic: {:<10} | Bytes: {:?} | Cycles: {}",
                pc_before_op, opcode, mnemonic, instr_bytes, cycles
            );

            self.decode(opcode)
        };

        for op in micro_ops {
            self.execute_microop(op);
        }
    }

    fn fetch8(&mut self) -> u8 {
        let byte = self.inter.read_byte(self.regs.get16(Reg16::PC));
        self.regs
            .set16(Reg16::PC, self.regs.get16(Reg16::PC).wrapping_add(1));
        self.cycles += 1; // 1 machine cycle for fetch
        byte
    }

    fn fetch16(&mut self) -> u16 {
        let lo = self.inter.read_byte(self.regs.get16(Reg16::PC)) as u16;
        self.regs
            .set16(Reg16::PC, self.regs.get16(Reg16::PC).wrapping_add(1));

        let hi = self.inter.read_byte(self.regs.get16(Reg16::PC)) as u16;
        self.regs
            .set16(Reg16::PC, self.regs.get16(Reg16::PC).wrapping_add(1));

        (hi << 8) | lo
    }

    fn push(&mut self, value: u8) {
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        self.inter.write_byte(self.regs.sp, value);
    }

    fn pop(&mut self) -> u8 {
        let value = self.inter.read_byte(self.regs.sp);
        self.regs.sp = self.regs.sp.wrapping_add(1);

        value
    }

    pub fn cb_decode(&mut self, opcode: u8) -> Vec<MicroOp> {
        match opcode {
            0x00 => vec![MicroOp::RlcReg8 { dst: (Reg8::B) }],
            0x01 => vec![MicroOp::RlcReg8 { dst: (Reg8::C) }],
            0x02 => vec![MicroOp::RlcReg8 { dst: (Reg8::D) }],
            0x03 => vec![MicroOp::RlcReg8 { dst: (Reg8::E) }],
            0x04 => vec![MicroOp::RlcReg8 { dst: (Reg8::H) }],
            0x05 => vec![MicroOp::RlcReg8 { dst: (Reg8::L) }],
            //0x06 Rlc Hl
            0x07 => vec![MicroOp::RlcReg8 { dst: (Reg8::A) }],

            _ => panic!("Unimplemented opcode: {:02X}", opcode),
        }
    }

    pub fn decode(&mut self, opcode: u8) -> Vec<MicroOp> {
        match opcode {
            0x00 => vec![MicroOp::Nop],
            0x01 => vec![MicroOp::LdReg16FromMem {
                dst: Reg16::BC,
                src: Reg16::PC,
            }],
            0x02 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::BC),
                src: (Reg8::A),
            }],
            0x03 => vec![MicroOp::IncReg16 { reg: (Reg16::BC) }],
            0x04 => vec![MicroOp::IncReg8 { reg: (Reg8::B) }],
            0x05 => vec![MicroOp::DecReg8 { reg: (Reg8::B) }],
            0x06 => vec![MicroOp::LdReg8FromImm { dst: Reg8::B }],
            0x07 => vec![MicroOp::Rlca],
            0x08 => vec![MicroOp::LdMemImm16FromReg16 { src: (Reg16::SP) }],
            0x09 => vec![MicroOp::AddReg16 {
                dst: (Reg16::HL),
                src: (Reg16::BC),
            }],
            0x0A => vec![MicroOp::LdReg8FromMem {
                dst: (Reg8::A),
                src: (Reg16::BC),
            }],
            0x0B => vec![MicroOp::DecReg16 { reg: (Reg16::BC) }],
            0x0C => vec![MicroOp::IncReg8 { reg: (Reg8::C) }],
            0x0D => vec![MicroOp::DecReg8 { reg: (Reg8::C) }],
            0x0E => vec![MicroOp::LdReg8FromImm { dst: (Reg8::C) }],
            0x0F => vec![MicroOp::Rrca],
            0x10 => vec![MicroOp::Stop],
            0x11 => vec![MicroOp::LdReg16FromMem {
                dst: (Reg16::DE),
                src: (Reg16::PC),
            }],
            0x12 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::DE),
                src: (Reg8::A),
            }],
            0x13 => vec![MicroOp::IncReg16 { reg: (Reg16::DE) }],
            0x14 => vec![MicroOp::IncReg8 { reg: (Reg8::D) }],
            0x15 => vec![MicroOp::DecReg8 { reg: (Reg8::D) }],
            0x16 => vec![MicroOp::LdReg8FromImm { dst: (Reg8::D) }],
            0x17 => vec![MicroOp::Rla],
            0x18 => vec![MicroOp::JumpRelative { offset: (8) }],
            0x19 => vec![MicroOp::AddReg16 {
                dst: (Reg16::HL),
                src: (Reg16::DE),
            }],
            0x1A => vec![MicroOp::LdReg8FromMem {
                dst: (Reg8::A),
                src: (Reg16::DE),
            }],
            0x1B => vec![MicroOp::DecReg16 { reg: (Reg16::DE) }],
            0x1C => vec![MicroOp::IncReg8 { reg: (Reg8::E) }],
            0x1D => vec![MicroOp::DecReg8 { reg: (Reg8::E) }],
            0x1E => vec![MicroOp::LdReg8FromImm { dst: (Reg8::E) }],
            0x1F => vec![MicroOp::Rra],
            0x20 => vec![MicroOp::JumpRelativeIf {
                offset: (8),
                flag: ('z'),
                expected: (false),
            }],
            0x21 => vec![MicroOp::LdReg16FromMem {
                dst: Reg16::HL,
                src: Reg16::PC,
            }],
            0x22 => vec![MicroOp::LdMemFromReg8IncHL { src: (Reg8::A) }],
            0x23 => vec![MicroOp::IncReg16 { reg: (Reg16::HL) }],
            0x24 => vec![MicroOp::IncReg8 { reg: (Reg8::H) }],
            0x25 => vec![MicroOp::DecReg8 { reg: (Reg8::H) }],
            0x26 => vec![MicroOp::LdReg8FromImm { dst: (Reg8::H) }],
            0x27 => vec![MicroOp::Daa],
            0x28 => vec![MicroOp::JumpRelativeIf {
                offset: (8),
                flag: ('z'),
                expected: (true),
            }],
            0x29 => vec![MicroOp::AddReg16 {
                dst: (Reg16::HL),
                src: (Reg16::HL),
            }],
            0x2A => vec![MicroOp::LdReg8FromMemIncHL { dst: (Reg8::A) }],
            0x2B => vec![MicroOp::DecReg16 { reg: (Reg16::HL) }],
            0x2C => vec![MicroOp::IncReg8 { reg: (Reg8::L) }],
            0x2D => vec![MicroOp::DecReg8 { reg: (Reg8::L) }],
            0x2E => vec![MicroOp::LdReg8FromImm { dst: (Reg8::L) }],
            0x2F => vec![MicroOp::Cpl],
            0x30 => vec![MicroOp::JumpRelativeIf {
                offset: (8),
                flag: ('c'),
                expected: (false),
            }],
            0x31 => vec![MicroOp::LdReg16FromMem {
                dst: Reg16::SP,
                src: Reg16::PC,
            }],
            0x32 => vec![MicroOp::LdMemFromReg8DecHL { src: (Reg8::A) }],
            0x33 => vec![MicroOp::IncReg16 { reg: (Reg16::SP) }],
            0x34 => vec![MicroOp::IncReg16 { reg: (Reg16::HL) }],
            0x35 => vec![MicroOp::DecReg16 { reg: (Reg16::HL) }],
            0x36 => vec![MicroOp::LdMemFromImm8 { addr: (Reg16::HL) }],
            0x37 => vec![MicroOp::Scf],
            0x38 => vec![MicroOp::JumpRelativeIf {
                offset: (8),
                flag: ('c'),
                expected: (true),
            }],
            0x39 => vec![MicroOp::AddReg16 {
                dst: (Reg16::HL),
                src: (Reg16::SP),
            }],
            0x3A => vec![MicroOp::LdReg8FromMemDecHL { dst: (Reg8::A) }],
            0x3B => vec![MicroOp::DecReg16 { reg: (Reg16::SP) }],
            0x3C => vec![MicroOp::IncReg8 { reg: (Reg8::A) }],
            0x3D => vec![MicroOp::DecReg8 { reg: (Reg8::A) }],
            0x3E => vec![MicroOp::LdReg8FromImm { dst: (Reg8::A) }],
            0x3F => vec![MicroOp::Ccf],
            0x40 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::B),
                src: (Reg8::B),
            }],
            0x41 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::B),
                src: (Reg8::C),
            }],
            0x42 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::B),
                src: (Reg8::D),
            }],
            0x43 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::B),
                src: (Reg8::E),
            }],
            0x44 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::B),
                src: (Reg8::H),
            }],
            0x45 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::B),
                src: (Reg8::L),
            }],
            0x46 => vec![MicroOp::LdReg8FromReg16 {
                dst: (Reg8::B),
                src: (Reg16::HL),
            }],
            0x47 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::B),
                src: (Reg8::A),
            }],
            0x48 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::C),
                src: (Reg8::B),
            }],
            0x49 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::C),
                src: (Reg8::C),
            }],
            0x4A => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::C),
                src: (Reg8::D),
            }],
            0x4B => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::C),
                src: (Reg8::E),
            }],
            0x4C => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::C),
                src: (Reg8::H),
            }],
            0x4D => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::C),
                src: (Reg8::L),
            }],
            0x4E => vec![MicroOp::LdReg8FromReg16 {
                dst: (Reg8::C),
                src: (Reg16::HL),
            }],
            0x4F => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::C),
                src: (Reg8::A),
            }],
            0x50 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::D),
                src: (Reg8::B),
            }],
            0x51 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::D),
                src: (Reg8::C),
            }],
            0x52 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::D),
                src: (Reg8::D),
            }],
            0x53 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::D),
                src: (Reg8::E),
            }],
            0x54 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::D),
                src: (Reg8::H),
            }],
            0x55 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::D),
                src: (Reg8::L),
            }],
            0x56 => vec![MicroOp::LdReg8FromReg16 {
                dst: (Reg8::D),
                src: (Reg16::HL),
            }],
            0x57 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::D),
                src: (Reg8::A),
            }],
            0x58 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::E),
                src: (Reg8::B),
            }],
            0x59 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::E),
                src: (Reg8::C),
            }],
            0x5A => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::E),
                src: (Reg8::D),
            }],

            0x5B => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::E),
                src: (Reg8::E),
            }],
            0x5C => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::E),
                src: (Reg8::H),
            }],
            0x5D => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::E),
                src: (Reg8::L),
            }],
            0x5E => vec![MicroOp::LdReg8FromReg16 {
                dst: (Reg8::E),
                src: (Reg16::HL),
            }],
            0x5F => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::E),
                src: (Reg8::A),
            }],
            0x60 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::H),
                src: (Reg8::B),
            }],
            0x61 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::H),
                src: (Reg8::C),
            }],
            0x62 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::H),
                src: (Reg8::D),
            }],
            0x63 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::H),
                src: (Reg8::E),
            }],
            0x64 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::H),
                src: (Reg8::H),
            }],
            0x65 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::H),
                src: (Reg8::L),
            }],
            0x66 => vec![MicroOp::LdReg8FromReg16 {
                dst: (Reg8::H),
                src: (Reg16::HL),
            }],
            0x67 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::H),
                src: (Reg8::A),
            }],
            0x68 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::B),
            }],
            0x69 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::C),
            }],
            0x6A => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::D),
            }],
            0x6B => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::E),
            }],
            0x6C => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::H),
            }],
            0x6D => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::L),
            }],
            0x6E => vec![MicroOp::LdReg8FromReg16 {
                dst: (Reg8::L),
                src: (Reg16::HL),
            }],
            0x6F => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::A),
            }],
            0x70 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::HL),
                src: (Reg8::B),
            }],
            0x71 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::HL),
                src: (Reg8::C),
            }],
            0x72 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::HL),
                src: (Reg8::D),
            }],
            0x73 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::HL),
                src: (Reg8::E),
            }],
            0x74 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::HL),
                src: (Reg8::H),
            }],
            0x75 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::HL),
                src: (Reg8::L),
            }],
            0x76 => vec![MicroOp::Halt],
            0x77 => vec![MicroOp::LdMemFromReg8 {
                addr: (Reg16::HL),
                src: (Reg8::A),
            }],

            0x78 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],

            0x79 => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],

            0x7A => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],

            0x7B => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],

            0x7C => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],

            0x7D => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],

            0x7E => vec![MicroOp::LdReg8FromReg16 {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],

            0x7F => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],

            0x80 => vec![MicroOp::AddReg8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],
            0x81 => vec![MicroOp::AddReg8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],
            0x82 => vec![MicroOp::AddReg8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],
            0x83 => vec![MicroOp::AddReg8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],
            0x84 => vec![MicroOp::AddReg8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],
            0x85 => vec![MicroOp::AddReg8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],
            0x86 => vec![MicroOp::AddReg8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],
            0x87 => vec![MicroOp::AddReg8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],
            0x88 => vec![MicroOp::AddCarry8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],
            0x89 => vec![MicroOp::AddCarry8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],
            0x8A => vec![MicroOp::AddCarry8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],
            0x8B => vec![MicroOp::AddCarry8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],
            0x8C => vec![MicroOp::AddCarry8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],
            0x8D => vec![MicroOp::AddCarry8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],

            0x8E => vec![MicroOp::AddCarry8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],

            0x8F => vec![MicroOp::AddCarry8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],

            0x90 => vec![MicroOp::SubReg8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],

            0x91 => vec![MicroOp::SubReg8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],

            0x92 => vec![MicroOp::SubReg8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],

            0x93 => vec![MicroOp::SubReg8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],

            0x94 => vec![MicroOp::SubReg8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],

            0x95 => vec![MicroOp::SubReg8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],

            0x96 => vec![MicroOp::SubCarry8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],

            0x97 => vec![MicroOp::SubReg8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],

            0x98 => vec![MicroOp::SubCarry8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],
            0x99 => vec![MicroOp::SubCarry8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],
            0x9A => vec![MicroOp::SubCarry8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],
            0x9B => vec![MicroOp::SubCarry8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],
            0x9C => vec![MicroOp::SubCarry8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],
            0x9D => vec![MicroOp::SubCarry8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],
            0x9E => vec![MicroOp::SubCarry8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],
            0x9F => vec![MicroOp::SubCarry8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],
            0xA0 => vec![MicroOp::AndReg8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],

            0xA1 => vec![MicroOp::AndReg8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],
            0xA2 => vec![MicroOp::AndReg8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],
            0xA3 => vec![MicroOp::AndReg8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],
            0xA4 => vec![MicroOp::AndReg8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],
            0xA5 => vec![MicroOp::AndReg8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],
            0xA6 => vec![MicroOp::AndReg8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],
            0xA7 => vec![MicroOp::AndReg8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],
            0xA8 => vec![MicroOp::XorReg8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],
            0xA9 => vec![MicroOp::XorReg8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],
            0xAA => vec![MicroOp::XorReg8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],
            0xAB => vec![MicroOp::XorReg8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],
            0xAC => vec![MicroOp::XorReg8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],
            0xAD => vec![MicroOp::XorReg8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],
            0xAE => vec![MicroOp::XorReg8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],

            0xAF => vec![MicroOp::XorReg8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],

            0xB0 => vec![MicroOp::OrReg8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],
            0xB1 => vec![MicroOp::OrReg8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],
            0xB2 => vec![MicroOp::OrReg8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],
            0xB3 => vec![MicroOp::OrReg8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],
            0xB4 => vec![MicroOp::OrReg8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],
            0xB5 => vec![MicroOp::OrReg8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],

            0xB6 => vec![MicroOp::OrReg8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],
            0xB7 => vec![MicroOp::OrReg8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],
            0xB8 => vec![MicroOp::CpReg8 {
                dst: (Reg8::A),
                src: (Reg8::B),
            }],
            0xB9 => vec![MicroOp::CpReg8 {
                dst: (Reg8::A),
                src: (Reg8::C),
            }],
            0xBA => vec![MicroOp::CpReg8 {
                dst: (Reg8::A),
                src: (Reg8::D),
            }],
            0xBB => vec![MicroOp::CpReg8 {
                dst: (Reg8::A),
                src: (Reg8::E),
            }],
            0xBC => vec![MicroOp::CpReg8 {
                dst: (Reg8::A),
                src: (Reg8::H),
            }],
            0xBD => vec![MicroOp::CpReg8 {
                dst: (Reg8::A),
                src: (Reg8::L),
            }],
            0xBE => vec![MicroOp::CpReg8Mem {
                dst: (Reg8::A),
                src: (Reg16::HL),
            }],
            0xBF => vec![MicroOp::CpReg8 {
                dst: (Reg8::A),
                src: (Reg8::A),
            }],
            0xC0 => vec![MicroOp::ReturnIf {
                flag: ('f'),
                expected: (false),
            }],
            0xC1 => vec![MicroOp::PopReg16 { reg: (Reg16::BC) }],
            0xC2 => {
                let addr = self.fetch16();
                vec![MicroOp::JumpAbsoluteIf {
                    addr: (addr),
                    flag: ('z'),
                    expected: (false),
                }]
            }
            0xC3 => {
                let addr = self.fetch16();
                vec![MicroOp::JumpAbsolute { addr: (addr) }]
            }
            0xC4 => {
                let addr = self.fetch16();
                vec![MicroOp::CallAbsoluteIf {
                    addr,
                    flag: ('z'),
                    expected: (false),
                }]
            }
            0xC5 => vec![MicroOp::PushReg16 { reg: (Reg16::BC) }],
            0xC6 => {
                let addr = self.fetch8();
                vec![MicroOp::AddReg8Imm {
                    dst: (Reg8::A),
                    addr: (addr),
                }]
            }
            0xC7 => vec![MicroOp::Restart { vector: (0x0000) }],
            0xC8 => vec![MicroOp::ReturnIf {
                flag: ('z'),
                expected: (true),
            }],
            0xC9 => vec![MicroOp::Return {}],
            0xCA => {
                let addr = self.fetch16();
                vec![MicroOp::JumpAbsoluteIf {
                    addr,
                    flag: ('z'),
                    expected: (true),
                }]
            }
            0xCC => {
                let addr: u16 = self.fetch16();
                vec![MicroOp::CallAbsoluteIf {
                    addr,
                    flag: ('z'),
                    expected: (true),
                }]
            }
            0xCD => {
                let addr: u16 = self.fetch16();
                vec![MicroOp::CallAbsolute { addr }]
            }
            0xCE => {
                let addr: u8 = self.fetch8();
                vec![MicroOp::AddCarry8Imm {
                    dst: (Reg8::A),
                    addr: (addr),
                }]
            }
            0xCF => vec![MicroOp::Restart { vector: (0x0008) }],
            0xD0 => vec![MicroOp::ReturnIf {
                flag: ('c'),
                expected: (false),
            }],
            0xD1 => vec![MicroOp::PopReg16 { reg: (Reg16::DE) }],
            0xD2 => {
                let addr = self.fetch16();
                vec![MicroOp::JumpAbsoluteIf {
                    addr: (addr),
                    flag: ('c'),
                    expected: (false),
                }]
            }
            0xD4 => {
                let addr = self.fetch16();
                vec![MicroOp::CallAbsoluteIf {
                    addr,
                    flag: ('c'),
                    expected: (false),
                }]
            }
            0xD5 => vec![MicroOp::PushReg16 { reg: (Reg16::DE) }],
            0xD6 => {
                let addr = self.fetch8();
                vec![MicroOp::SubReg8Imm {
                    dst: (Reg8::A),
                    addr,
                }]
            }
            0xD7 => vec![MicroOp::Restart { vector: (0x0010) }],
            0xD8 => vec![MicroOp::ReturnIf {
                flag: ('c'),
                expected: (true),
            }],
            0xD9 => vec![MicroOp::Reti {}],
            0xDA => {
                let addr = self.fetch16();
                vec![MicroOp::JumpAbsoluteIf {
                    addr,
                    flag: ('c'),
                    expected: (true),
                }]
            }
            0xDC => {
                let addr: u16 = self.fetch16();
                vec![MicroOp::CallAbsoluteIf {
                    addr,
                    flag: ('C'),
                    expected: (true),
                }]
            }

            0xDE => {
                let addr = self.fetch8();
                vec![MicroOp::SubCarry8Imm {
                    dst: (Reg8::A),
                    addr: (addr),
                }]
            }
            0xDF => vec![MicroOp::Restart { vector: (0x0018) }],
            0xE0 => {
                let addr = self.fetch8();
                vec![MicroOp::LdA8FromA { offset: (addr) }]
            }
            0xE1 => vec![MicroOp::PopReg16 { reg: (Reg16::HL) }],
            0xE2 => vec![MicroOp::LdCFromA],
            0xE5 => vec![MicroOp::PushReg16 { reg: (Reg16::HL) }],
            0xE6 => {
                let addr = self.fetch8();
                vec![MicroOp::AndReg8Imm {
                    dst: (Reg8::A),
                    addr,
                }]
            }
            0xE7 => vec![MicroOp::Restart { vector: (0x0020) }],
            0xE8 => {
                let addr = self.fetch8() as i8;
                vec![MicroOp::AddImmToSP { imm: (addr) }]
            }
            0xE9 => vec![MicroOp::JumpHL],
            0xEA => {
                let addr = self.fetch16();
                vec![MicroOp::LdMemFromA { addr }]
            }
            0xEE => {
                let addr = self.fetch8();
                vec![MicroOp::XorReg8Imm {
                    dst: (Reg8::A),
                    addr,
                }]
            }
            0xEF => vec![MicroOp::Restart { vector: (0x0028) }],
            0xF0 => {
                let addr = self.fetch8();
                vec![MicroOp::LdAFromA8 { offset: (addr) }]
            }
            0xF1 => vec![MicroOp::PopReg16 { reg: (Reg16::AF) }],
            0xF2 => vec![MicroOp::LdAFromC],
            0xF3 => vec![MicroOp::Di],
            0xF5 => vec![MicroOp::PushReg16 { reg: (Reg16::AF) }],
            0xF6 => {
                let addr = self.fetch8();
                vec![MicroOp::OrReg8Imm {
                    dst: (Reg8::A),
                    addr: (addr),
                }]
            }
            0xF7 => vec![MicroOp::Restart { vector: (0x0030) }],
            0xF8 => vec![MicroOp::LdHLSPPlusR8],
            0xF9 => vec![MicroOp::LdReg16FromMem {
                dst: (Reg16::SP),
                src: (Reg16::HL),
            }],
            0xFB => vec![MicroOp::Ei],
            0xFE => {
                let addr = self.fetch8();
                vec![MicroOp::CpReg8Imm {
                    dst: (Reg8::A),
                    addr,
                }]
            }
            0xFF => vec![MicroOp::Restart { vector: (0x0038) }],
            _ => panic!("Unimplemented opcode: {:02X}", opcode),
        }
    }

    pub fn execute_microop(&mut self, op: MicroOp) {
        match op {
            MicroOp::Nop => {}

            MicroOp::Halt => {}

            MicroOp::Stop => {}

            //Load instructions
            MicroOp::LdReg8FromReg8 { dst, src } => {
                let v = self.regs.get8(src);
                self.regs.set8(dst, v);
            }

            MicroOp::LdReg8FromMem { dst, src } => {
                let addr = self.regs.get16(src);
                let value = self.inter.read_byte(addr);
                self.regs.set8(dst, value);
            }
            MicroOp::LdReg8FromImm { dst } => {
                let value = self.fetch8();
                self.regs.set8(dst, value);
            }

            MicroOp::LdMemFromReg8 { addr, src } => {
                let value = self.regs.get8(src);
                let address = self.regs.get16(addr);
                self.inter.write_byte(address, value);
                self.cycles += 1;
            }

            MicroOp::LdA8FromA { offset } => {
                let addr = 0xFF00u16 + offset as u16;
                let value = self.regs.get8(Reg8::A);
                self.inter.write_byte(addr, value);
            }

            MicroOp::LdAFromA8 { offset } => {
                let addr = 0xFF00u16 + offset as u16;
                let value = self.inter.read_byte(addr);
                self.regs.set8(Reg8::A, value);
            }

            MicroOp::LdCFromA => {
                let addr: u16 = 0xFF00u16 + self.regs.get8(Reg8::C) as u16;
                let value = self.regs.get8(Reg8::A);
                self.inter.write_byte(addr, value);
            }

            MicroOp::LdAFromC => {
                let addr: u16 = 0xFF00u16 + self.regs.get8(Reg8::C) as u16;
                let value = self.inter.read_byte(addr);
                self.regs.set8(Reg8::A, value);
            }

            MicroOp::LdMemFromA { addr } => {
                let value = self.regs.get8(Reg8::A);
                self.inter.write_byte(addr, value);
            }

            MicroOp::LdReg16FromMem { dst, src } => {
                let addr = self.regs.get16(src);
                let lo = self.inter.read_byte(addr) as u16;
                let hi = self.inter.read_byte(addr.wrapping_add(1)) as u16;
                let value = (hi << 8) | lo;
                self.regs.set16(dst, value);
            }

            MicroOp::LdReg8FromMemIncHL { dst } => {
                let hl = self.regs.get16(Reg16::HL);
                let value = self.inter.read_byte(hl);

                self.regs.set8(dst, value);

                self.regs.set16(Reg16::HL, hl.wrapping_add(1));
            }

            MicroOp::LdMemFromReg8IncHL { src } => {
                let hl = self.regs.get16(Reg16::HL);

                let value = self.regs.get8(src);

                self.inter.write_byte(hl, value);

                self.regs.set16(Reg16::HL, hl.wrapping_add(1));
            }

            MicroOp::LdMemFromReg8DecHL { src } => {
                let hl = self.regs.get16(Reg16::HL);

                let value = self.regs.get8(src);

                self.inter.write_byte(hl, value);

                self.regs.set16(Reg16::HL, hl.wrapping_sub(1));
            }

            MicroOp::LdReg8FromMemDecHL { dst } => {
                let hl = self.regs.get16(Reg16::HL);
                let value = self.inter.read_byte(hl);

                self.regs.set8(dst, value);

                self.regs.set16(Reg16::HL, hl.wrapping_sub(1));
            }

            MicroOp::LdMemImm16FromReg16 { src } => {
                let lo = self.fetch8() as u16;
                let hi = self.fetch8() as u16;
                let addr = (hi << 8) | lo;

                let value = self.regs.get16(src);

                let lo_val = (value & 0x00FF) as u8;
                let hi_val = (value >> 8) as u8;

                self.inter.write_byte(addr, lo_val);
                self.inter.write_byte(addr + 1, hi_val);
            }

            MicroOp::LdReg8FromReg16 { dst, src } => {
                let addr = self.regs.get16(src);
                let value = self.inter.read_byte(addr);
                self.regs.set8(dst, value);
            }

            MicroOp::LdMemFromImm8 { addr } => {
                let hl = self.regs.get16(addr);

                let value = self.fetch8();
                self.inter.write_byte(hl, value);
            }

            //Logical
            MicroOp::IncReg8 { reg } => {
                let value = self.regs.get8(reg);
                self.regs.set8(reg, value.wrapping_add(1));
            }

            MicroOp::DecReg8 { reg } => {
                let value = self.regs.get8(reg);
                self.regs.set8(reg, value.wrapping_sub(1));
            }

            MicroOp::IncReg16 { reg } => {
                let value = self.regs.get16(reg);
                self.regs.set16(reg, value.wrapping_add(1));
            }

            MicroOp::DecReg16 { reg } => {
                let value = self.regs.get16(reg);
                self.regs.set16(reg, value.wrapping_sub(1));
            }

            MicroOp::AddReg8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let alu_out = self.alu.add_8bit(a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::AddReg8Mem { dst, src } => {
                let a = self.regs.get8(dst);
                let addr = self.regs.get16(src);
                let value = self.inter.read_byte(addr);

                let alu_out = self.alu.add_8bit(a, value);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::AddReg8Imm { dst, addr } => {
                let a = self.regs.get8(dst);

                let alu_out = self.alu.add_8bit(a, addr);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::AddReg16 { dst, src } => {
                let a = self.regs.get16(dst);
                let b = self.regs.get16(src);

                let alu_out = self.alu.add_16bit(a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set16(dst, result);
            }

            MicroOp::AddCarry8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let cpu_flag = self.flags.get_flag('C');

                let alu_out = self.alu.adc_8bit(cpu_flag, a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::AddCarry8Mem { dst, src } => {
                let mem = self.regs.get16(src);
                let value = self.inter.read_byte(mem);
                let a = self.regs.get8(dst);
                let carry = self.flags.get_flag('c') as u8;

                let alu_out = self.alu.add_8bit(a, value + carry);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::AddCarry8Imm { dst, addr } => {
                let a = self.regs.get8(dst);
                let carry = if self.flags.get_flag('C') { 1 } else { 0 };

                let alu_out = self.alu.add_8bit(a, addr + carry);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::SubReg8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let alu_out = self.alu.sub_8bit(a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            /*  MicroOp::SubReg8Mem { dst, src } => {
                let mem = self.regs.get16(src);
                let value = self.inter.read_byte(mem);
                let a = self.regs.get8(dst);

                let alu_out = self.alu.sub_8bit(a, value);
                let result = alu_out.result;

                self.regs.set8(dst, result);
            }*/
            MicroOp::SubReg8Imm { dst, addr } => {
                let a = self.regs.get8(dst);

                let alu_out = self.alu.sub_8bit(a, addr);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::SubCarry8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);
                let cpu_flag = self.flags.get_flag('C');

                let alu_out = self.alu.sbc_8bit(cpu_flag, a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::SubCarry8Mem { dst, src } => {
                let mem = self.regs.get16(src);
                let value = self.inter.read_byte(mem);
                let a = self.regs.get8(dst);
                let carry = self.flags.get_flag('c') as u8;

                let alu_out = self.alu.sub_8bit(a, value + carry);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::SubCarry8Imm { dst, addr } => {
                let a = self.regs.get8(dst);
                let carry = if self.flags.get_flag('C') { 1 } else { 0 };

                let alu_out = self.alu.add_8bit(a, addr + carry);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::XorReg8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let alu_out = self.alu.xor_8bit(a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::XorReg8Mem { dst, src } => {
                let mem = self.regs.get16(src);
                let value = self.inter.read_byte(mem);
                let a = self.regs.get8(dst);

                let alu_out = self.alu.xor_8bit(a, value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::XorReg8Imm { dst, addr } => {
                let a = self.regs.get8(dst);

                let alu_out = self.alu.xor_8bit(a, addr);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::CpReg8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let alu_out = self.alu.cp_8bit(a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
            }

            MicroOp::CpReg8Mem { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get16(src);
                let value = self.inter.read_byte(b);

                let alu_out = self.alu.cp_8bit(a, value);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
            }

            MicroOp::CpReg8Imm { dst, addr } => {
                let a = self.regs.get8(dst);

                let alu_out = self.alu.cp_8bit(a, addr);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
            }

            MicroOp::OrReg8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let alu_out = self.alu.or_8bit(a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::OrReg8Mem { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get16(src);
                let value = self.inter.read_byte(b);

                let alu_out = self.alu.or_8bit(a, value);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::OrReg8Imm { dst, addr } => {
                let a = self.regs.get8(dst);

                let alu_out = self.alu.or_8bit(a, addr);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::AndReg8 { dst, src } => {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let alu_out = self.alu.and_8bit(a, b);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::AndReg8Mem { dst, src } => {
                let mem = self.regs.get16(src);
                let value = self.inter.read_byte(mem);

                let a = self.regs.get8(dst);
                let alu_out = self.alu.and_8bit(a, value);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::AndReg8Imm { dst, addr } => {
                let a = self.regs.get8(dst);
                let alu_out = self.alu.and_8bit(a, addr);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                let result = alu_out.result;
                self.regs.set8(dst, result);
            }

            MicroOp::PushReg16 { reg } => {
                let val = self.regs.get16(reg);
                let hi = (val >> 8) as u8;
                let lo = val as u8;

                self.regs.sp = self.regs.sp.wrapping_sub(1);
                self.inter.write_byte(self.regs.sp, hi);

                self.regs.sp = self.regs.sp.wrapping_sub(1);
                self.inter.write_byte(self.regs.sp, lo);
            }

            MicroOp::PopReg16 { reg } => {
                let lo = self.inter.read_byte(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);

                let hi = self.inter.read_byte(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);

                let val = ((hi as u16) << 8) | lo as u16;
                self.regs.set16(reg, val);
            }

            MicroOp::JumpAbsolute { addr } => {
                self.regs.pc = addr;
            }

            MicroOp::JumpAbsoluteIf {
                addr,
                flag,
                expected,
            } => {
                let value = self.flags.get_flag(flag);
                let taken = value == expected;

                if taken {
                    self.regs.set16(Reg16::PC, addr);
                }

                //taken
            }

            MicroOp::JumpRelative { offset } => {
                let pc = self.regs.pc.wrapping_add(offset as u16);
                self.regs.pc = pc;
            }

            MicroOp::JumpRelativeIf {
                offset,
                flag,
                expected,
            } => {
                if self.flags.get_flag(flag) == expected {
                    let new_pc = self.regs.pc.wrapping_add(offset as u16);
                    self.regs.pc = new_pc;
                }
            }

            MicroOp::JumpHL => {
                let hl = self.regs.get16(Reg16::HL);
                self.regs.set16(Reg16::PC, hl);
            }

            MicroOp::CallAbsolute { addr } => {
                let pc = self.regs.pc;
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                self.inter
                    .write_byte(self.regs.sp, ((pc >> 8) & 0xFF) as u8);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                self.inter.write_byte(self.regs.sp, (pc & 0xFF) as u8);

                self.regs.pc = addr;
            }

            MicroOp::CallAbsoluteIf {
                addr,
                flag,
                expected,
            } => {
                if self.flags.get_flag(flag) == expected {
                    // Push current PC onto stack
                    let pc = self.regs.pc;
                    self.regs.sp = self.regs.sp.wrapping_sub(1);
                    self.inter
                        .write_byte(self.regs.sp, ((pc >> 8) & 0xFF) as u8);
                    self.regs.sp = self.regs.sp.wrapping_sub(1);
                    self.inter.write_byte(self.regs.sp, (pc & 0xFF) as u8);

                    // Jump to target
                    self.regs.pc = addr;
                } else {
                    // No jump; nothing else to do
                }
            }

            MicroOp::Return => {
                let lo = self.inter.read_byte(self.regs.sp) as u16;
                self.regs.sp = self.regs.sp.wrapping_add(1);
                let hi = self.inter.read_byte(self.regs.sp) as u16;
                self.regs.sp = self.regs.sp.wrapping_add(1);

                self.regs.pc = (hi << 8) | lo;
            }

            MicroOp::ReturnIf { flag, expected } => {
                if self.flags.get_flag(flag) == expected {
                    // Pop 16-bit address from stack (little-endian)
                    let lo = self.inter.read_byte(self.regs.sp) as u16;
                    self.regs.sp = self.regs.sp.wrapping_add(1);
                    let hi = self.inter.read_byte(self.regs.sp) as u16;
                    self.regs.sp = self.regs.sp.wrapping_add(1);

                    // Set PC to popped address
                    self.regs.pc = (hi << 8) | lo;
                }
            }

            MicroOp::Reti => {
                let lo = self.pop();
                let hi = self.pop();
                self.regs.pc = ((hi as u16) << 8) | (lo as u16);

                self.interrupt = true;
            }

            MicroOp::Restart { vector } => {
                let pc = self.regs.get16(Reg16::PC);

                let hi = ((pc >> 8) & 0xFF) as u8;
                let lo = (pc & 0xFF) as u8;

                self.push(hi);
                self.push(lo);

                self.regs.set16(Reg16::PC, vector);
            }

            MicroOp::Rlca => {
                let a = self.regs.get8(Reg8::A);
                let old = (a >> 7) & 1;
                let result = (a << 1) | old;

                self.regs.set8(Reg8::A, result);

                self.flags.set_flag('z', false);
                self.flags.set_flag('n', false);
                self.flags.set_flag('h', false);
                self.flags.set_flag('c', old == 1);
            }

            MicroOp::Rrca => {
                let bit0 = self.regs.get8(Reg8::A) & 0x01;

                let result = (self.regs.get8(Reg8::A) >> 1) | (bit0 << 7);

                self.regs.set8(Reg8::A, result);

                self.flags.set_flag('z', false);
                self.flags.set_flag('n', false);
                self.flags.set_flag('h', false);
                self.flags.set_flag('c', bit0 != 0);
            }

            MicroOp::Rla => {
                let a = self.regs.get8(Reg8::A);
                let old = (a >> 7) & 1;

                let carry = if self.flags.get_flag('c') { 1 } else { 0 };
                let result = (a << 1) | carry;

                self.regs.set8(Reg8::A, result);

                self.flags.set_flag('z', false);
                self.flags.set_flag('n', false);
                self.flags.set_flag('h', false);
                self.flags.set_flag('c', old == 1);
            }

            MicroOp::Rra => {
                let a = self.regs.get8(Reg8::A);
                let old = a & 1;
                let result = (a >> 1) | (old << 7);

                self.regs.set8(Reg8::A, result);

                self.flags.set_flag('z', false);
                self.flags.set_flag('n', false);
                self.flags.set_flag('h', false);
                self.flags.set_flag('c', old == 1);
            }

            MicroOp::Di => {
                self.interrupt_enable_next = false;
                self.interrupt = false;
            }

            MicroOp::Ei => {
                self.interrupt_enable_next = true;
            }

            MicroOp::Cpl => {
                self.regs.a = !self.regs.a;

                self.flags.set_flag('C', true);
                self.flags.set_flag('H', true);
            }

            MicroOp::Ccf => {
                let carry = self.flags.get_flag('C');

                self.flags.set_flag('C', !carry);

                self.flags.set_flag('N', false);
                self.flags.set_flag('H', false);
            }

            MicroOp::Scf => {
                self.flags.set_flag('C', true);

                self.flags.set_flag('N', false);
                self.flags.set_flag('H', false);
            }

            MicroOp::Daa => {
                let mut a = self.regs.a;
                let mut correction: u8 = 0;
                let mut carry = self.flags.get_flag('C');

                let n = self.flags.get_flag('N');

                if !n {
                    // After ADD
                    if self.flags.get_flag('H') || (a & 0x0F) > 9 {
                        correction |= 0x06;
                    }
                    if carry || a > 0x99 {
                        correction |= 0x60;
                        carry = true;
                    }
                    a = a.wrapping_add(correction);
                } else {
                    // After SUB
                    if self.flags.get_flag('H') {
                        correction |= 0x06;
                    }
                    if carry {
                        correction |= 0x60;
                    }
                    a = a.wrapping_sub(correction);
                }

                self.regs.a = a;

                self.flags.set_flag('Z', a == 0);
                self.flags.set_flag('H', false);
                self.flags.set_flag('C', carry);
            }

            /*MicroOp::RlReg8 { dst } => {
                let value = self.regs.get8(dst);
                let c_flag = self.flags.get_flag('c');

                let alu_out = self.alu.rl_byte(value, c_flag);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }*/
            MicroOp::RlcReg8 { dst } => {
                let value = self.regs.get8(dst);

                let alu_out = self.alu.rlc_byte(value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            /*  MicroOp::RrReg8 { dst } => {
                let value = self.regs.get8(dst);
                let c_flag = self.flags.get_flag('c');

                let alu_out = self.alu.rr_byte(value, c_flag);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::RrcReg8 { dst } => {
                let value = self.regs.get8(dst);

                let alu_out = self.alu.rrc_byte(value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::SlaReg8 { dst } => {
                let value = self.regs.get8(dst);

                let alu_out = self.alu.sla_byte(value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::SraReg8 { dst } => {
                let value = self.regs.get8(dst);

                let alu_out = self.alu.sra_byte(value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::SrlReg8 { dst } => {
                let value = self.regs.get8(dst);

                let alu_out = self.alu.srl_byte(value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::SwapReg8 { dst } => {
                let value = self.regs.get8(dst);

                let alu_out = self.alu.swap_byte(value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::RlRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);
                let c_flag = self.flags.get_flag('c');

                let alu_out = self.alu.rl_byte(val, c_flag);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }
            MicroOp::RlcRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let alu_out = self.alu.rlc_byte(val);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }
            MicroOp::RrRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);
                let c_flag = self.flags.get_flag('c');

                let alu_out = self.alu.rr_byte(val, c_flag);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }
            MicroOp::RrcRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let alu_out = self.alu.rrc_byte(val);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }
            MicroOp::SlaRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let alu_out = self.alu.sla_byte(val);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }
            MicroOp::SraRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let alu_out = self.alu.sra_byte(val);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }
            MicroOp::SrlRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let alu_out = self.alu.srl_byte(val);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }
            MicroOp::SwapRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let alu_out = self.alu.swap_byte(val);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
                self.inter.write_byte(addr, result);
            }*/
            MicroOp::AddImmToSP { imm } => {
                let sp = self.regs.sp;
                let result = sp.wrapping_add(imm as i16 as u16);

                let half_carry = ((sp & 0xF) + ((imm as u16) & 0xF)) > 0xF;
                let carry = ((sp & 0xFF) + ((imm as u16) & 0xFF)) > 0xFF;

                self.regs.sp = result;

                self.flags.set_flag('z', false);
                self.flags.set_flag('n', false);
                self.flags.set_flag('h', half_carry);
                self.flags.set_flag('c', carry);
            }

            MicroOp::LdHLSPPlusR8 => {
                let sp = self.regs.sp;

                let imm = self.fetch8() as i8 as i16;
                let result = sp.wrapping_add(imm as u16);

                self.regs.set16(Reg16::HL, result);

                self.flags.set_flag('z', false);
                self.flags.set_flag('n', false);

                let sp_lo = sp as u8;
                let imm8 = imm as u8;

                let half_carry = ((sp_lo & 0x0f) + (imm8 & 0x0F)) > 0x0F;
                let carry = (sp_lo as u16 + imm8 as u16) > 0xFF;
                self.flags.set_flag('h', half_carry);
                self.flags.set_flag('c', carry);
            } //Never used might delete
              //MicroOp::Illegal { opcode } => {
              //    println!("illegal opcode: {}", opcode);
              //}
        }
    }
}

#[cfg(test)]
mod tests;
