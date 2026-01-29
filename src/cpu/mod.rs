pub mod alu;
pub mod logging;
pub mod microops;
pub mod registers;

use crate::cpu::alu::Alu;
use crate::cpu::logging::opcode_info;
use crate::cpu::microops::MicroOp;
use crate::cpu::registers::{Flags, Reg16, Reg8, Registers};
use crate::interconnect::Interconnect;
use std::collections::VecDeque;

enum DecodeFlow {
    NoImm,
    Imm8,
    Imm16,
}

enum CpuState {
    FetchOpcode,
    FetchImm8,
    FetchImm16Lo,
    FetchImm16Hi,
    ExecuteMicroOp,
    Decode,
}

pub struct Cpu {
    pub regs: Registers,

    state: CpuState,

    flags: Flags,

    alu: Alu,

    interrupt: bool,

    imm8: u8,

    imm16: u16,
    
    micro_ops: VecDeque<MicroOp>,

    opcode: u8,

    interrupt_enable_next: bool,

    //halted: bool,
    pub inter: Interconnect,

    pub cycles: u64,
}

impl Cpu {
    pub fn new(inter: Interconnect) -> Cpu {
        let regs = Registers {
            pc: 0x100,
            sp: 0xFFFE,
            a: 0x01,
            b: 0x00,
            f: Flags::new(),
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
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

            state: CpuState::FetchOpcode,

            imm8: 0,
            imm16: 0,

            micro_ops: VecDeque::new(),
            opcode: 0,


            interrupt: true,
            interrupt_enable_next: true,
            //halted: false,
            cycles: 0,
        }
    }
pub fn step(&mut self) -> u64 {
    self.cycles += 1;

    match self.state {
        CpuState::FetchOpcode => {
            self.opcode = self.inter.read_byte(self.regs.pc);
            self.regs.pc = self.regs.pc.wrapping_add(1);
            self.state = CpuState::Decode;
        }

        CpuState::Decode => {
          
          let (ops, flow, cycles) = self.decode(self.opcode);
self.cycles += cycles as u64;
self.micro_ops = ops.into();


            self.state = match flow {
                DecodeFlow::NoImm => CpuState::ExecuteMicroOp,
                DecodeFlow::Imm8  => CpuState::FetchImm8,
                DecodeFlow::Imm16 => CpuState::FetchImm16Lo,
            };
        }

        CpuState::FetchImm8 => {
            self.imm8 = self.inter.read_byte(self.regs.pc);
            self.regs.pc = self.regs.pc.wrapping_add(1);
            self.state = CpuState::ExecuteMicroOp;
        }

        CpuState::FetchImm16Lo => {
            let lo = self.inter.read_byte(self.regs.pc);
            self.regs.pc = self.regs.pc.wrapping_add(1);
            self.imm16 = lo as u16;
            self.state = CpuState::FetchImm16Hi;
        }

        CpuState::FetchImm16Hi => {
            let hi = self.inter.read_byte(self.regs.pc);
            self.regs.pc = self.regs.pc.wrapping_add(1);
            self.imm16 |= (hi as u16) << 8;
            self.state = CpuState::ExecuteMicroOp;
        }

        CpuState::ExecuteMicroOp => {
            if let Some(op) = self.micro_ops.pop_front() {
                self.execute_micro_op(op);
            } else {
                self.state = CpuState::FetchOpcode;
            }
        }
    }

    1
}

    fn fetch_imm8(&mut self) {
    self.imm8 = self.inter.read_byte(self.regs.pc);
    self.regs.pc = self.regs.pc.wrapping_add(1);

    self.state = CpuState::ExecuteMicroOp;
}

fn fetch_imm16_low(&mut self) {
    let lo = self.inter.read_byte(self.regs.pc);
    self.regs.pc += 1;
    self.imm16 = lo as u16;
    self.state = CpuState::FetchImm16Hi;
}

fn fetch_imm16_high(&mut self) {
    let hi = self.inter.read_byte(self.regs.pc);
    self.regs.pc += 1;
    self.imm16 |= (hi as u16) << 8;
    self.state = CpuState::ExecuteMicroOp;
}

fn pop_8bit(&mut self) -> u8 {
    let sp = self.regs.get16(Reg16::SP);

    let value = self.inter.read_byte(sp);

    self.regs.set16(Reg16::SP, sp.wrapping_add(1));

    value
}

fn push_8bit(&mut self, value: u8) {
    let sp = self.regs.get16(Reg16::SP).wrapping_sub(1);
    self.regs.set16(Reg16::SP, sp);

    self.inter.write_byte(sp, value);
}

fn pop_16bit(&mut self) -> u16 {
    let sp = self.regs.get16(Reg16::SP);

    let lo = self.inter.read_byte(sp) as u16;
    let hi = self.inter.read_byte(sp.wrapping_add(1)) as u16;

    self.regs.set16(Reg16::SP, sp.wrapping_add(2));

    (hi << 8) | lo
}

fn push_16bit(&mut self, value: u16) {
    let sp = self.regs.get16(Reg16::SP).wrapping_sub(2);

    self.regs.set16(Reg16::SP, sp);

    let lo = (value & 0x00FF) as u8;
    let hi = (value >> 8) as u8;

    self.inter.write_byte(sp, lo);
    self.inter.write_byte(sp.wrapping_add(1), hi);
}


    pub fn cb_decode(&mut self, opcode: u8) -> (Vec<MicroOp>, u8) {
        match opcode {
            0x00 => (vec![MicroOp::RlcReg8 { dst: (Reg8::B) }], 2),
            0x01 => (vec![MicroOp::RlcReg8 { dst: (Reg8::C) }], 2),
            0x02 => (vec![MicroOp::RlcReg8 { dst: (Reg8::D) }], 2),
            0x03 => (vec![MicroOp::RlcReg8 { dst: (Reg8::E) }], 2),
            0x04 => (vec![MicroOp::RlcReg8 { dst: (Reg8::H) }], 2),
            0x05 => (vec![MicroOp::RlcReg8 { dst: (Reg8::L) }], 2),
            0x06 => (vec![MicroOp::RlcRegHl], 4),
            0x07 => (vec![MicroOp::RlcReg8 { dst: (Reg8::A) }], 2),
            0x08 => (vec![MicroOp::RrcReg8 { dst: (Reg8::B) }], 2),
            0x09 => (vec![MicroOp::RrcReg8 { dst: (Reg8::C) }], 2),
            0x0A => (vec![MicroOp::RrcReg8 { dst: (Reg8::D) }], 2),
            0x0B => (vec![MicroOp::RrcReg8 { dst: (Reg8::E) }], 2),
            0x0C => (vec![MicroOp::RrcReg8 { dst: (Reg8::H) }], 2),
            0x0D => (vec![MicroOp::RrcReg8 { dst: (Reg8::L) }], 2),
            0x0E => (vec![MicroOp::RrcRegHl], 4),
            0x0F => (vec![MicroOp::RrcReg8 { dst: (Reg8::A) }], 2),
            0x10 => (vec![MicroOp::RlReg8 { dst: (Reg8::B) }], 2),
            0x11 => (vec![MicroOp::RlReg8 { dst: (Reg8::C) }], 2),
            0x12 => (vec![MicroOp::RlReg8 { dst: (Reg8::D) }], 2),
            0x13 => (vec![MicroOp::RlReg8 { dst: (Reg8::E) }], 2),
            0x14 => (vec![MicroOp::RlReg8 { dst: (Reg8::H) }], 2),
            0x15 => (vec![MicroOp::RlReg8 { dst: (Reg8::L) }], 2),
            0x16 => (vec![MicroOp::RlRegHl], 4),
            0x17 => (vec![MicroOp::RlReg8 { dst: (Reg8::A) }], 2),
            0x18 => (vec![MicroOp::RrReg8 { dst: (Reg8::B) }], 2),
            0x19 => (vec![MicroOp::RrReg8 { dst: (Reg8::C) }], 2),
            0x1A => (vec![MicroOp::RrReg8 { dst: (Reg8::D) }], 2),
            0x1B => (vec![MicroOp::RrReg8 { dst: (Reg8::E) }], 2),
            0x1C => (vec![MicroOp::RrReg8 { dst: (Reg8::H) }], 2),
            0x1D => (vec![MicroOp::RrReg8 { dst: (Reg8::L) }], 2),
            0x1E => (vec![MicroOp::RrRegHl], 4),
            0x1F => (vec![MicroOp::SlaReg8 { dst: (Reg8::A) }], 2),
            0x20 => (vec![MicroOp::SlaReg8 { dst: (Reg8::B) }], 2),
            0x21 => (vec![MicroOp::SlaReg8 { dst: (Reg8::C) }], 2),
            0x22 => (vec![MicroOp::SlaReg8 { dst: (Reg8::D) }], 2),
            0x23 => (vec![MicroOp::SlaReg8 { dst: (Reg8::E) }], 2),
            0x24 => (vec![MicroOp::SlaReg8 { dst: (Reg8::H) }], 2),
            0x25 => (vec![MicroOp::SlaReg8 { dst: (Reg8::L) }], 2),
            0x26 => (vec![MicroOp::SlaRegHl], 4),
            0x27 => (vec![MicroOp::SlaReg8 { dst: (Reg8::A) }], 2),
            0x28 => (vec![MicroOp::SraReg8 { dst: (Reg8::B) }], 2),
            0x29 => (vec![MicroOp::SraReg8 { dst: (Reg8::C) }], 2),
            0x2A => (vec![MicroOp::SraReg8 { dst: (Reg8::D) }], 2),
            0x2B => (vec![MicroOp::SraReg8 { dst: (Reg8::E) }], 2),
            0x2C => (vec![MicroOp::SraReg8 { dst: (Reg8::H) }], 2),
            0x2D => (vec![MicroOp::SraReg8 { dst: (Reg8::L) }], 2),
            0x2E => (vec![MicroOp::SraRegHl], 4),
            0x2F => (vec![MicroOp::SraReg8 { dst: (Reg8::A) }], 2),
            0x30 => (vec![MicroOp::SwapReg8 { dst: (Reg8::B) }], 2),
            0x31 => (vec![MicroOp::SwapReg8 { dst: (Reg8::C) }], 2),
            0x32 => (vec![MicroOp::SwapReg8 { dst: (Reg8::D) }], 2),
            0x33 => (vec![MicroOp::SwapReg8 { dst: (Reg8::E) }], 2),
            0x34 => (vec![MicroOp::SwapReg8 { dst: (Reg8::H) }], 2),
            0x35 => (vec![MicroOp::SwapReg8 { dst: (Reg8::L) }], 2),
            0x36 => (vec![MicroOp::SwapRegHl], 4),
            0x37 => (vec![MicroOp::SwapReg8 { dst: (Reg8::A) }], 2),
            0x38 => (vec![MicroOp::SrlReg8 { dst: (Reg8::B) }], 2),
            0x39 => (vec![MicroOp::SrlReg8 { dst: (Reg8::C) }], 2),
            0x3A => (vec![MicroOp::SrlReg8 { dst: (Reg8::D) }], 2),
            0x3B => (vec![MicroOp::SrlReg8 { dst: (Reg8::E) }], 2),
            0x3C => (vec![MicroOp::SrlReg8 { dst: (Reg8::H) }], 2),
            0x3D => (vec![MicroOp::SrlReg8 { dst: (Reg8::L) }], 2),
            0x3E => (vec![MicroOp::SrlRegHl], 4),
            0x3F => (vec![MicroOp::SrlReg8 { dst: (Reg8::A) }], 2),
            0x40 => (
                vec![MicroOp::BitReg8 {
                    bit: (0),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x41 => (
                vec![MicroOp::BitReg8 {
                    bit: (0),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x42 => (
                vec![MicroOp::BitReg8 {
                    bit: (0),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x43 => (
                vec![MicroOp::BitReg8 {
                    bit: (0),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x44 => (
                vec![MicroOp::BitReg8 {
                    bit: (0),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x45 => (
                vec![MicroOp::BitReg8 {
                    bit: (0),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x46 => (vec![MicroOp::BitRegHl { bit: (0) }], 4),
            0x47 => (
                vec![MicroOp::BitReg8 {
                    bit: (0),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x48 => (
                vec![MicroOp::BitReg8 {
                    bit: (1),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x49 => (
                vec![MicroOp::BitReg8 {
                    bit: (1),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x4A => (
                vec![MicroOp::BitReg8 {
                    bit: (1),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x4B => (
                vec![MicroOp::BitReg8 {
                    bit: (1),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x4C => (
                vec![MicroOp::BitReg8 {
                    bit: (1),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x4D => (
                vec![MicroOp::BitReg8 {
                    bit: (1),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x4E => (vec![MicroOp::BitRegHl { bit: (1) }], 4),
            0x4F => (
                vec![MicroOp::BitReg8 {
                    bit: (1),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x50 => (
                vec![MicroOp::BitReg8 {
                    bit: (2),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x51 => (
                vec![MicroOp::BitReg8 {
                    bit: (2),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x52 => (
                vec![MicroOp::BitReg8 {
                    bit: (2),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x53 => (
                vec![MicroOp::BitReg8 {
                    bit: (2),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x54 => (
                vec![MicroOp::BitReg8 {
                    bit: (2),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x55 => (
                vec![MicroOp::BitReg8 {
                    bit: (2),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x56 => (vec![MicroOp::BitRegHl { bit: (2) }], 4),
            0x57 => (
                vec![MicroOp::BitReg8 {
                    bit: (2),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x58 => (
                vec![MicroOp::BitReg8 {
                    bit: (3),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x59 => (
                vec![MicroOp::BitReg8 {
                    bit: (3),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x5A => (
                vec![MicroOp::BitReg8 {
                    bit: (3),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x5B => (
                vec![MicroOp::BitReg8 {
                    bit: (3),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x5C => (
                vec![MicroOp::BitReg8 {
                    bit: (3),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x5D => (
                vec![MicroOp::BitReg8 {
                    bit: (3),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x5E => (vec![MicroOp::BitRegHl { bit: (3) }], 4),
            0x5F => (
                vec![MicroOp::BitReg8 {
                    bit: (3),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x60 => (
                vec![MicroOp::BitReg8 {
                    bit: (4),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x61 => (
                vec![MicroOp::BitReg8 {
                    bit: (4),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x62 => (
                vec![MicroOp::BitReg8 {
                    bit: (4),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x63 => (
                vec![MicroOp::BitReg8 {
                    bit: (4),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x64 => (
                vec![MicroOp::BitReg8 {
                    bit: (4),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x65 => (
                vec![MicroOp::BitReg8 {
                    bit: (4),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x66 => (vec![MicroOp::BitRegHl { bit: (4) }], 4),
            0x67 => (
                vec![MicroOp::BitReg8 {
                    bit: (4),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x68 => (
                vec![MicroOp::BitReg8 {
                    bit: (5),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x69 => (
                vec![MicroOp::BitReg8 {
                    bit: (5),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x6A => (
                vec![MicroOp::BitReg8 {
                    bit: (5),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x6B => (
                vec![MicroOp::BitReg8 {
                    bit: (5),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x6C => (
                vec![MicroOp::BitReg8 {
                    bit: (5),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x6D => (
                vec![MicroOp::BitReg8 {
                    bit: (5),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x6E => (vec![MicroOp::BitRegHl { bit: (5) }], 4),
            0x6F => (
                vec![MicroOp::BitReg8 {
                    bit: (5),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x70 => (
                vec![MicroOp::BitReg8 {
                    bit: (6),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x71 => (
                vec![MicroOp::BitReg8 {
                    bit: (6),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x72 => (
                vec![MicroOp::BitReg8 {
                    bit: (6),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x73 => (
                vec![MicroOp::BitReg8 {
                    bit: (6),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x74 => (
                vec![MicroOp::BitReg8 {
                    bit: (6),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x75 => (
                vec![MicroOp::BitReg8 {
                    bit: (6),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x76 => (vec![MicroOp::BitRegHl { bit: (6) }], 4),
            0x77 => (
                vec![MicroOp::BitReg8 {
                    bit: (6),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x78 => (
                vec![MicroOp::BitReg8 {
                    bit: (7),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x79 => (
                vec![MicroOp::BitReg8 {
                    bit: (7),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x7A => (
                vec![MicroOp::BitReg8 {
                    bit: (7),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x7B => (
                vec![MicroOp::BitReg8 {
                    bit: (7),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x7C => (
                vec![MicroOp::BitReg8 {
                    bit: (7),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x7D => (
                vec![MicroOp::BitReg8 {
                    bit: (7),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x7E => (vec![MicroOp::BitRegHl { bit: (7) }], 4),
            0x7F => (
                vec![MicroOp::BitReg8 {
                    bit: (7),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x80 => (
                vec![MicroOp::ResReg8 {
                    bit: (0),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x81 => (
                vec![MicroOp::ResReg8 {
                    bit: (0),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x82 => (
                vec![MicroOp::ResReg8 {
                    bit: (0),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x83 => (
                vec![MicroOp::ResReg8 {
                    bit: (0),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x84 => (
                vec![MicroOp::ResReg8 {
                    bit: (0),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x85 => (
                vec![MicroOp::ResReg8 {
                    bit: (0),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x86 => (vec![MicroOp::ResRegHl { bit: (0) }], 4),
            0x87 => (
                vec![MicroOp::ResReg8 {
                    bit: (0),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x88 => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x89 => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x8A => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x8B => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x8C => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x8D => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x8E => (vec![MicroOp::ResRegHl { bit: (1) }], 4),
            0x8F => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x90 => (
                vec![MicroOp::ResReg8 {
                    bit: (2),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x91 => (
                vec![MicroOp::ResReg8 {
                    bit: (2),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x92 => (
                vec![MicroOp::ResReg8 {
                    bit: (2),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x93 => (
                vec![MicroOp::ResReg8 {
                    bit: (2),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x94 => (
                vec![MicroOp::ResReg8 {
                    bit: (2),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x95 => (
                vec![MicroOp::ResReg8 {
                    bit: (2),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x96 => (vec![MicroOp::ResRegHl { bit: (2) }], 4),
            0x97 => (
                vec![MicroOp::ResReg8 {
                    bit: (2),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0x98 => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0x99 => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0x9A => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0x9B => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0x9C => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0x9D => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0x9E => (vec![MicroOp::ResRegHl { bit: (3) }], 4),
            0x9F => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xA0 => (
                vec![MicroOp::ResReg8 {
                    bit: (4),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xA1 => (
                vec![MicroOp::ResReg8 {
                    bit: (4),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xA2 => (
                vec![MicroOp::ResReg8 {
                    bit: (4),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xA3 => (
                vec![MicroOp::ResReg8 {
                    bit: (4),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xA4 => (
                vec![MicroOp::ResReg8 {
                    bit: (4),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xA5 => (
                vec![MicroOp::ResReg8 {
                    bit: (4),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xA6 => (vec![MicroOp::ResRegHl { bit: (4) }], 4),
            0xA7 => (
                vec![MicroOp::ResReg8 {
                    bit: (4),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xA8 => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xA9 => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xAA => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xAB => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xAC => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xAD => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xAE => (vec![MicroOp::ResRegHl { bit: (5) }], 4),
            0xAF => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xB0 => (
                vec![MicroOp::ResReg8 {
                    bit: (6),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xB1 => (
                vec![MicroOp::ResReg8 {
                    bit: (6),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xB2 => (
                vec![MicroOp::ResReg8 {
                    bit: (6),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xB3 => (
                vec![MicroOp::ResReg8 {
                    bit: (6),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xB4 => (
                vec![MicroOp::ResReg8 {
                    bit: (6),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xB5 => (
                vec![MicroOp::ResReg8 {
                    bit: (6),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xB6 => (vec![MicroOp::ResRegHl { bit: (6) }], 4),
            0xB7 => (
                vec![MicroOp::ResReg8 {
                    bit: (6),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xB8 => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xB9 => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xBA => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xBB => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xBC => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xBD => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xBE => (vec![MicroOp::ResRegHl { bit: (7) }], 4),
            0xBF => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xC0 => (
                vec![MicroOp::SetReg8 {
                    bit: (0),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xC1 => (
                vec![MicroOp::SetReg8 {
                    bit: (0),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xC2 => (
                vec![MicroOp::SetReg8 {
                    bit: (0),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xC3 => (
                vec![MicroOp::SetReg8 {
                    bit: (0),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xC4 => (
                vec![MicroOp::SetReg8 {
                    bit: (0),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xC5 => (
                vec![MicroOp::SetReg8 {
                    bit: (0),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xC6 => (vec![MicroOp::SetRegHl { bit: (0) }], 4),
            0xC7 => (
                vec![MicroOp::SetReg8 {
                    bit: (0),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xC8 => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xC9 => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xCA => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xCB => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xCC => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xCD => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xCE => (vec![MicroOp::ResRegHl { bit: (1) }], 4),
            0xCF => (
                vec![MicroOp::ResReg8 {
                    bit: (1),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xD0 => (
                vec![MicroOp::SetReg8 {
                    bit: (2),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xD1 => (
                vec![MicroOp::SetReg8 {
                    bit: (2),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xD2 => (
                vec![MicroOp::SetReg8 {
                    bit: (2),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xD3 => (
                vec![MicroOp::SetReg8 {
                    bit: (2),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xD4 => (
                vec![MicroOp::SetReg8 {
                    bit: (2),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xD5 => (
                vec![MicroOp::SetReg8 {
                    bit: (2),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xD6 => (vec![MicroOp::SetRegHl { bit: (2) }], 4),
            0xD7 => (
                vec![MicroOp::SetReg8 {
                    bit: (2),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xD8 => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xD9 => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xDA => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xDB => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xDC => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xDD => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xDE => (vec![MicroOp::ResRegHl { bit: (3) }], 4),
            0xDF => (
                vec![MicroOp::ResReg8 {
                    bit: (3),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xE0 => (
                vec![MicroOp::SetReg8 {
                    bit: (4),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xE1 => (
                vec![MicroOp::SetReg8 {
                    bit: (4),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xE2 => (
                vec![MicroOp::SetReg8 {
                    bit: (4),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xE3 => (
                vec![MicroOp::SetReg8 {
                    bit: (4),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xE4 => (
                vec![MicroOp::SetReg8 {
                    bit: (4),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xE5 => (
                vec![MicroOp::SetReg8 {
                    bit: (4),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xE6 => (vec![MicroOp::SetRegHl { bit: (4) }], 4),
            0xE7 => (
                vec![MicroOp::SetReg8 {
                    bit: (4),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xE8 => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xE9 => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xEA => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xEB => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xEC => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xED => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xEE => (vec![MicroOp::ResRegHl { bit: (5) }], 2),
            0xEF => (
                vec![MicroOp::ResReg8 {
                    bit: (5),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xF0 => (
                vec![MicroOp::SetReg8 {
                    bit: (6),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xF1 => (
                vec![MicroOp::SetReg8 {
                    bit: (6),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xF2 => (
                vec![MicroOp::SetReg8 {
                    bit: (6),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xF3 => (
                vec![MicroOp::SetReg8 {
                    bit: (6),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xF4 => (
                vec![MicroOp::SetReg8 {
                    bit: (6),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xF5 => (
                vec![MicroOp::SetReg8 {
                    bit: (6),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xF6 => (vec![MicroOp::SetRegHl { bit: (6) }], 4),
            0xF7 => (
                vec![MicroOp::SetReg8 {
                    bit: (6),
                    reg: (Reg8::A),
                }],
                2,
            ),
            0xF8 => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::B),
                }],
                2,
            ),
            0xF9 => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::C),
                }],
                2,
            ),
            0xFA => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::D),
                }],
                2,
            ),
            0xFB => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::E),
                }],
                2,
            ),
            0xFC => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::H),
                }],
                2,
            ),
            0xFD => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::L),
                }],
                2,
            ),
            0xFE => (vec![MicroOp::ResRegHl { bit: (7) }], 4),
            0xFF => (
                vec![MicroOp::ResReg8 {
                    bit: (7),
                    reg: (Reg8::A),
                }],
                2,
            ),
            // _ => panic!("Unimplemented opcode: {:02X}", opcode),
        }
    }

pub fn decode(&mut self, opcode: u8) -> (Vec<MicroOp>, DecodeFlow, u8) {
    match opcode {
        // =========================
        // 0x00–0x0F
        // =========================
        0x00 => (vec![MicroOp::Nop], DecodeFlow::NoImm, 4), // NOP: 4 cycles

        0x01 => (
            vec![MicroOp::LdReg16FromImm { dst: Reg16::BC }],
            DecodeFlow::Imm16,
            12, // LD BC,nn: 12 cycles
        ),

        0x06 => (
            vec![MicroOp::LdReg8FromImm { dst: Reg8::B }],
            DecodeFlow::Imm8,
            8, // LD B,n: 8 cycles
        ),

        0x0E => (
            vec![MicroOp::LdReg8FromImm { dst: Reg8::C }],
            DecodeFlow::Imm8,
            8, // LD C,n: 8 cycles
        ),

        // =========================
        // 0x10–0x1F
        // =========================
        0x11 => (
            vec![MicroOp::LdReg16FromImm { dst: Reg16::DE }],
            DecodeFlow::Imm16,
            12, // LD DE,nn
        ),

        0x16 => (
            vec![MicroOp::LdReg8FromImm { dst: Reg8::D }],
            DecodeFlow::Imm8,
            8, // LD D,n
        ),

        0x18 => (
            vec![MicroOp::JumpRelative],
            DecodeFlow::Imm8,
            12, // JR n
        ),

        0x1E => (
            vec![MicroOp::LdReg8FromImm { dst: Reg8::E }],
            DecodeFlow::Imm8,
            8, // LD E,n
        ),

        // =========================
        // 0x20–0x2F
        // =========================
        0x20 => (
            vec![MicroOp::JumpRelativeIf { flag: 'z', expected: false }],
            DecodeFlow::Imm8,
            12, // JR NZ,n (12 cycles if not taken, +4 if taken)
        ),

        0x21 => (
            vec![MicroOp::LdReg16FromImm { dst: Reg16::HL }],
            DecodeFlow::Imm16,
            12, // LD HL,nn
        ),

        0x26 => (
            vec![MicroOp::LdReg8FromImm { dst: Reg8::H }],
            DecodeFlow::Imm8,
            8, // LD H,n
        ),

        0x28 => (
            vec![MicroOp::JumpRelativeIf { flag: 'z', expected: true }],
            DecodeFlow::Imm8,
            12, // JR Z,n
        ),

        0x2A => (
            vec![MicroOp::LdReg8FromMemIncHL { dst: Reg8::A }],
            DecodeFlow::NoImm,
            8, // LD A,(HL+) 
        ),

        0x2E => (
            vec![MicroOp::LdReg8FromImm { dst: Reg8::L }],
            DecodeFlow::Imm8,
            8, // LD L,n
        ),

        // =========================
        // 0x30–0x3F
        // =========================
        0x30 => (
            vec![MicroOp::JumpRelativeIf { flag: 'c', expected: false }],
            DecodeFlow::Imm8,
            12, // JR NC,n
        ),

        0x31 => (
            vec![MicroOp::LdReg16FromImm { dst: Reg16::SP }],
            DecodeFlow::Imm16,
            12, // LD SP,nn
        ),

        0x32 => (
            vec![MicroOp::LdMemFromReg8DecHL { src: Reg8::A }],
            DecodeFlow::NoImm,
            8, // LD (HL-),A
        ),

        0x36 => (
            vec![MicroOp::LdMemFromImm8 { addr: Reg16::HL }],
            DecodeFlow::Imm8,
            12, // LD (HL),n
        ),

        0x38 => (
            vec![MicroOp::JumpRelativeIf { flag: 'c', expected: true }],
            DecodeFlow::Imm8,
            12, // JR C,n
        ),

        0x3A => (
            vec![MicroOp::LdReg8FromMemDecHL { dst: Reg8::A }],
            DecodeFlow::NoImm,
            8, // LD A,(HL-)
        ),

        0x3E => (
            vec![MicroOp::LdReg8FromImm { dst: Reg8::A }],
            DecodeFlow::Imm8,
            8, // LD A,n
        ),

        // =========================
        // 0x40–0x7F (LD r,r′ and r,(HL))
        // =========================
        0x46 => (
            vec![MicroOp::LdReg8FromMem { dst: Reg8::B, src: Reg16::HL }],
            DecodeFlow::NoImm,
            8, // LD B,(HL)
        ),

        0x4E => (
            vec![MicroOp::LdReg8FromMem { dst: Reg8::C, src: Reg16::HL }],
            DecodeFlow::NoImm,
            8, // LD C,(HL)
        ),

        0x56 => (
            vec![MicroOp::LdReg8FromMem { dst: Reg8::D, src: Reg16::HL }],
            DecodeFlow::NoImm,
            8, // LD D,(HL)
        ),

        0x5E => (
            vec![MicroOp::LdReg8FromMem { dst: Reg8::E, src: Reg16::HL }],
            DecodeFlow::NoImm,
            8, // LD E,(HL)
        ),

        0x7E => (
            vec![MicroOp::LdReg8FromMem { dst: Reg8::A, src: Reg16::HL }],
            DecodeFlow::NoImm,
            8, // LD A,(HL)
        ),

        // =========================
        // 0x76 HALT
        // =========================
        0x76 => (vec![MicroOp::Halt], DecodeFlow::NoImm, 4), // HALT

        // =========================
        // 0xC0–0xCF
        // =========================
        0xC3 => (
            vec![MicroOp::JumpAbsolute],
            DecodeFlow::Imm16,
            16, // JP nn
        ),

        0xC9 => (vec![MicroOp::Return], DecodeFlow::NoImm, 16), // RET

        0xCD => (
            vec![MicroOp::CallAbsolute],
            DecodeFlow::Imm16,
            24, // CALL nn
        ),

        _ => panic!(
            "Invalid opcode {:02X} at PC {:04X}",
            opcode,
            self.regs.pc.wrapping_sub(1)
        ),
    }
}

    pub fn execute_micro_op(&mut self, op: MicroOp) {
        match op {
            MicroOp::Nop => {}

            MicroOp::Halt => {}

            MicroOp::Stop => {}

            //Load Instructions
            MicroOp::LdReg8FromImm { dst } => {
    self.regs.set8(dst, self.imm8);

}

MicroOp::LdReg8FromReg8 { dst, src } => {
    let v = self.regs.get8(src);
    self.regs.set8(dst, v);
}

MicroOp::LdReg8FromMem { dst, src } => {
    let addr = self.regs.get16(src);
    let v = self.inter.read_byte(addr);
    self.regs.set8(dst, v);
}


MicroOp::LdReg8FromImm { dst } => {
    self.regs.set8(dst, self.imm8);
}


MicroOp::LdReg8FromMemImm16 { dst } => {
    let value = self.inter.read_byte(self.imm16);
    self.regs.set8(dst, value);
}


MicroOp::LdReg16FromImm { dst } => {
    self.regs.set16(dst, self.imm16);
}

MicroOp::LdMemImm16FromReg16 { src } => {
    let value = self.regs.get16(src);

    let lo = (value & 0x00FF) as u8;
    let hi = (value >> 8) as u8;

    self.inter.write_byte(self.imm16, lo);
    self.inter.write_byte(self.imm16.wrapping_add(1), hi);
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

MicroOp::LdA8FromA => {
    let addr = 0xFF00u16 + self.imm8 as u16;
    let value = self.regs.get8(Reg8::A);
    self.inter.write_byte(addr, value);
}

MicroOp::LdAFromA8 => {
    let addr = 0xFF00u16 + self.imm8 as u16;
    let value = self.inter.read_byte(addr);
    self.regs.set8(Reg8::A, value);
}

MicroOp::LdCFromA => {
    let addr = 0xFF00u16 + self.regs.get8(Reg8::C) as u16;
    let value = self.regs.get8(Reg8::A);
    self.inter.write_byte(addr, value);
}

MicroOp::LdAFromC => {
    let addr = 0xFF00u16 + self.regs.get8(Reg8::C) as u16;
    let value = self.inter.read_byte(addr);
    self.regs.set8(Reg8::A, value);
}

MicroOp::LdMemFromA => {
    let value = self.regs.get8(Reg8::A);
    self.inter.write_byte(self.imm16, value);
}

MicroOp::LdReg8FromMemImm16 { dst } => {
    let value = self.inter.read_byte(self.imm16);
    self.regs.set8(dst, value);
}

MicroOp::LdReg16FromMem { dst, src } => {
    let addr = self.regs.get16(src);
    let lo = self.inter.read_byte(addr) as u16;
    let hi = self.inter.read_byte(addr.wrapping_add(1)) as u16;
    self.regs.set16(dst, (hi << 8) | lo);
}

MicroOp::LdReg16FromImm { dst } => {
    self.regs.set16(dst, self.imm16);
}

MicroOp::LdMemImm16FromReg16 { src } => {
    let value = self.regs.get16(src);
    let lo = value as u8;
    let hi = (value >> 8) as u8;

    self.inter.write_byte(self.imm16, lo);
    self.inter.write_byte(self.imm16.wrapping_add(1), hi);
}

MicroOp::LdMemFromReg8 { addr, src } => {
    let address = self.regs.get16(addr);
    let value = self.regs.get8(src);
    self.inter.write_byte(address, value);
}

MicroOp::LdReg8FromReg16 { dst, src } => {
    let addr = self.regs.get16(src);
    let value = self.inter.read_byte(addr);
    self.regs.set8(dst, value);
}

//HL, n
MicroOp::LdMemFromImm8 { addr } => {
    let address = self.regs.get16(addr);
    self.inter.write_byte(address, self.imm8);
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
                let carry = self.flags.get_flag('c');

                let alu_out = self.alu.sbc_8bit(carry, a, value);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }

            MicroOp::SubCarry8Imm { dst, addr } => {
                let a = self.regs.get8(dst);
                let carry = self.flags.get_flag('c');

                let alu_out = self.alu.sbc_8bit(carry, a, addr);
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
    let value = self.regs.get16(reg);
    let sp = self.regs.get16(Reg16::SP).wrapping_sub(2);

    self.regs.set16(Reg16::SP, sp);
    self.inter.write_byte(sp, (value & 0xFF) as u8);
    self.inter.write_byte(sp.wrapping_add(1), (value >> 8) as u8);
}

MicroOp::PopReg16 { reg } => {
    let sp = self.regs.get16(Reg16::SP);

    let lo = self.inter.read_byte(sp) as u16;
    let hi = self.inter.read_byte(sp.wrapping_add(1)) as u16;

    self.regs.set16(Reg16::SP, sp.wrapping_add(2));
    self.regs.set16(reg, (hi << 8) | lo);
}

MicroOp::JumpAbsolute => {
    self.regs.set16(Reg16::PC, self.imm16);
}

MicroOp::JumpAbsoluteIf { flag, expected } => {
    if self.flags.get_flag(flag) == expected {
        self.regs.set16(Reg16::PC, self.imm16);
    }
}

MicroOp::JumpRelative => {
    let offset = self.imm8 as i8 as i16;
    let pc = self.regs.get16(Reg16::PC);
    self.regs.set16(Reg16::PC, pc.wrapping_add(offset as u16));
}

MicroOp::JumpRelativeIf { flag, expected } => {
    if self.flags.get_flag(flag) == expected {
        let offset = self.imm8 as i8 as i16;
        let pc = self.regs.get16(Reg16::PC);
        self.regs.set16(Reg16::PC, pc.wrapping_add(offset as u16));
    }
}

         MicroOp::JumpHL => {
    let hl = self.regs.get16(Reg16::HL);
    self.regs.set16(Reg16::PC, hl);
}

MicroOp::CallAbsolute => {
    let pc = self.regs.get16(Reg16::PC);
    self.push_16bit(pc);
    self.regs.set16(Reg16::PC, self.imm16);
}

MicroOp::CallAbsoluteIf { flag, expected } => {
    if self.flags.get_flag(flag) == expected {
        let pc = self.regs.get16(Reg16::PC);
        self.push_16bit(pc);
        self.regs.set16(Reg16::PC, self.imm16);
    }
}

MicroOp::Return => {
    let addr = self.pop_16bit();
    self.regs.set16(Reg16::PC, addr);
}

MicroOp::ReturnIf { flag, expected } => {
    if self.flags.get_flag(flag) == expected {
        let addr = self.pop_16bit();
        self.regs.set16(Reg16::PC, addr);
    }
}

MicroOp::Reti => {
    let addr = self.pop_16bit();
    self.regs.set16(Reg16::PC, addr);
    self.interrupt = true;
}

MicroOp::Restart { vector } => {
    let pc = self.regs.get16(Reg16::PC);
    self.push_16bit(pc);
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
                let carry_in = if self.flags.get_flag('C') { 1 } else { 0 };
                let carry_out = a & 1;

                let result = (a >> 1) | (carry_in << 7);

                self.regs.set8(Reg8::A, result);

                self.flags.set_flag('Z', false);
                self.flags.set_flag('N', false);
                self.flags.set_flag('H', false);
                self.flags.set_flag('C', carry_out != 0);
            }

            MicroOp::Di => {
                self.interrupt_enable_next = false;
                self.interrupt = false;
            }

            MicroOp::Ei => {
                self.interrupt_enable_next = true;
                self.interrupt = true;
            }

            MicroOp::Cpl => {
                self.regs.a = !self.regs.a;

                self.flags.set_flag('N', true);
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

            MicroOp::RlReg8 { dst } => {
                let value = self.regs.get8(dst);
                let c_flag = self.flags.get_flag('c');

                let alu_out = self.alu.rl_byte(value, c_flag);
                let result = alu_out.result;

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);

                self.regs.set8(dst, result);
            }
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

            MicroOp::RrReg8 { dst } => {
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
            }
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

            MicroOp::BitRegHl { bit } => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let alu_out = self.alu.bit_byte(val, bit);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
            }

            MicroOp::BitReg8 { bit, reg } => {
                let val = self.regs.get8(reg);

                let alu_out = self.alu.bit_byte(val, bit);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
            }

            MicroOp::ResRegHl { bit } => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let result = self.alu.res_byte(val, bit);

                self.inter.write_byte(addr, result);
            }

            MicroOp::ResReg8 { bit, reg } => {
                let val = self.regs.get8(reg);
                let result = self.alu.res_byte(val, bit);

                self.regs.set8(reg, result);
            }

            MicroOp::SetRegHl { bit } => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let result = self.alu.set_byte(val, bit);

                self.inter.write_byte(addr, result);
            }

            MicroOp::SetReg8 { bit, reg } => {
                let val = self.regs.get8(reg);
                let result = self.alu.set_byte(val, bit);

                self.regs.set8(reg, result);
            }

       MicroOp::LdHLSPPlusR8 { offset } => {
    // offset is already an i8 from fetched immediate
    let sp = self.regs.get16(Reg16::SP);
    let result = sp.wrapping_add(offset as i16 as u16);
    self.regs.set16(Reg16::HL, result);

    // Flags
    self.flags.set_flag('z', false);
    self.flags.set_flag('n', false);

    let sp_lo = sp as u8;
    let offset_u8 = offset as u8;

    // Half-carry: did the lower nibble overflow?
    let half_carry = ((sp_lo & 0x0F).wrapping_add(offset_u8 & 0x0F)) > 0x0F;
    // Carry: did the full byte overflow?
    let carry = sp_lo.wrapping_add(offset_u8) < sp_lo;

    self.flags.set_flag('h', half_carry);
    self.flags.set_flag('c', carry);
}

            MicroOp::Unimplemented => {} //Never used might delete
                                         //MicroOp::Illegal { opcode } => {
                                         //    println!("illegal opcode: {}", opcode);
                                         //}
        }
    }
}

#[cfg(test)]
mod tests;
