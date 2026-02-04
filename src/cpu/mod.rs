pub mod alu;
pub mod microops;
pub mod registers;

use crate::cpu::alu::Alu;
use crate::cpu::microops::MicroOp;
use crate::cpu::registers::{Flags, Reg16, Reg8, Registers};
use crate::interconnect::Interconnect;
use std::collections::VecDeque;

const CPU_TRACE: bool = true;

#[derive(Debug)]
pub enum DecodeFlow {
    NoImm,
    Imm8,
    Imm16,
    CbPrefix,
}

enum CpuState {
    FetchOpcode,
    FetchImm8,
    FetchImm16Lo,
    FetchImm16Hi,
    ExecuteMicroOp,
    Decode,
    FetchCbOpcode,
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
        let mut total_cycles = 0;

        match self.state {
            CpuState::FetchOpcode => {
                self.opcode = self.inter.read_byte(self.regs.pc);

                if CPU_TRACE {
                    println!(
                        "[FETCH ] PC={:04X} OPCODE={:02X}",
                        self.regs.pc, self.opcode
                    );
                }

                assert!(
                    self.micro_ops.is_empty(),
                    "Fetched opcode while micro-ops still pending"
                );

                self.regs.pc = self.regs.pc.wrapping_add(1);
                self.state = CpuState::Decode;
            }

            CpuState::Decode => {
                let pc_before = self.regs.get16(Reg16::PC);

                let (ops, flow, cycles) = self.decode(self.opcode);

                if CPU_TRACE {
                    println!(
                        "[DECODE] PC={:04X} OPCODE={:02X} FLOW={:?} μOPS={} CYCLES={}",
                        self.regs.pc,
                        self.opcode,
                        flow,
                        ops.len(),
                        cycles
                    );
                }

                self.micro_ops = ops.into();

                self.state = match flow {
                    DecodeFlow::NoImm => CpuState::ExecuteMicroOp,
                    DecodeFlow::Imm8 => CpuState::FetchImm8,
                    DecodeFlow::Imm16 => CpuState::FetchImm16Lo,
                    DecodeFlow::CbPrefix => CpuState::FetchCbOpcode,
                };

                debug_assert!(
                    matches!(flow, DecodeFlow::NoImm) || self.regs.get16(Reg16::PC) == pc_before,
                    "BAD PC BUMP in decode: {:02X}",
                    self.opcode
                );
            }

            CpuState::FetchImm8 => {
                if let Some(op) = self.micro_ops.front() {
                    match op {
                        MicroOp::JumpRelative | MicroOp::JumpRelativeIf { .. } => {
                            self.imm8 = self.inter.read_byte(self.regs.pc);
                            if CPU_TRACE {
                                println!(
                                    "[IMM8  ] PC={:04X} VALUE={:02X}",
                                    self.regs.pc, self.imm8
                                );
                            }
                        }
                        _ => {}
                    }
                }
                self.regs.pc = self.regs.pc.wrapping_add(1);
                self.state = CpuState::ExecuteMicroOp;
            }

            CpuState::FetchImm16Lo => {
                let lo = self.inter.read_byte(self.regs.pc);

                if CPU_TRACE {
                    println!("[IMM16L] PC={:04X} LO={:02X}", self.regs.pc, lo);
                }
                self.regs.pc = self.regs.pc.wrapping_add(1);
                self.imm16 = lo as u16;
                self.state = CpuState::FetchImm16Hi;
            }

            CpuState::FetchImm16Hi => {
                let hi = self.inter.read_byte(self.regs.pc);

                if CPU_TRACE {
                    println!(
                        "[IMM16H] PC={:04X} HI={:02X} => IMM16={:04X}",
                        self.regs.pc,
                        hi,
                        (hi as u16) << 8 | self.imm16
                    );
                }
                self.regs.pc = self.regs.pc.wrapping_add(1);
                self.imm16 |= (hi as u16) << 8;
                self.state = CpuState::ExecuteMicroOp;
            }

            CpuState::FetchCbOpcode => {
                let cb_opcode = self.fetch_byte();
                if CPU_TRACE {
                    println!(
                        "[CBFETCH] PC={:04X} CB OPCODE={:02X}",
                        self.regs.pc, cb_opcode
                    );
                }
                self.regs.pc = self.regs.pc.wrapping_add(1);

                let (ops, flow, cycles) = self.cb_decode(cb_opcode);

                self.micro_ops = ops.into();
                self.cycles += cycles as u64;
                total_cycles += cycles as u64;

                self.state = CpuState::ExecuteMicroOp;
            }

            CpuState::ExecuteMicroOp => {
                if let Some(op) = self.micro_ops.pop_front() {
                    if CPU_TRACE {
                        println!(
                            "[EXEC  ] PC={:04X} OPCODE={:02X} MicroOP={:?}",
                            self.regs.pc, self.opcode, op
                        );
                    }
                    self.execute_micro_op(op);
                } else {
                    self.state = CpuState::FetchOpcode;
                }
            }
        }

        self.cycles = self.cycles.wrapping_add(total_cycles);

        total_cycles
    }

    fn fetch_byte(&mut self) -> u8 {
        let b = self.inter.read_byte(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        b
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

    fn cb_rot(&self, reg_op: fn(Reg8) -> MicroOp, hl_op: MicroOp, reg_index: u8) -> MicroOp {
        match reg_index {
            0 => reg_op(Reg8::B),
            1 => reg_op(Reg8::C),
            2 => reg_op(Reg8::D),
            3 => reg_op(Reg8::E),
            4 => reg_op(Reg8::H),
            5 => reg_op(Reg8::L),
            6 => hl_op, // HL memory
            7 => reg_op(Reg8::A),
            _ => unreachable!(),
        }
    }

    pub fn cb_decode(&self, opcode: u8) -> (Vec<MicroOp>, DecodeFlow, u8) {
        let reg_index = opcode & 0x07;
        let bit = (opcode >> 3) & 0x07;
        let group = opcode >> 6;

        let cycles = if reg_index == 6 { 4 } else { 2 };

        // Helper to map reg_index to Reg8
        let reg8 = match reg_index {
            0 => Reg8::B,
            1 => Reg8::C,
            2 => Reg8::D,
            3 => Reg8::E,
            4 => Reg8::H,
            5 => Reg8::L,
            7 => Reg8::A,
            _ => Reg8::A, // placeholder for HL case, not used
        };

        let micro_ops = match group {
            // ROTATES / SHIFTS
            0b00 => match bit {
                0 => {
                    vec![self.cb_rot(|dst| MicroOp::RlcReg8 { dst }, MicroOp::RlcRegHl, reg_index)]
                }
                1 => {
                    vec![self.cb_rot(|dst| MicroOp::RrcReg8 { dst }, MicroOp::RrcRegHl, reg_index)]
                }
                2 => vec![self.cb_rot(|dst| MicroOp::RlReg8 { dst }, MicroOp::RlRegHl, reg_index)],
                3 => vec![self.cb_rot(|dst| MicroOp::RrReg8 { dst }, MicroOp::RrRegHl, reg_index)],
                4 => {
                    vec![self.cb_rot(|dst| MicroOp::SlaReg8 { dst }, MicroOp::SlaRegHl, reg_index)]
                }
                5 => {
                    vec![self.cb_rot(|dst| MicroOp::SraReg8 { dst }, MicroOp::SraRegHl, reg_index)]
                }
                6 => vec![self.cb_rot(
                    |dst| MicroOp::SwapReg8 { dst },
                    MicroOp::SwapRegHl,
                    reg_index,
                )],
                7 => {
                    vec![self.cb_rot(|dst| MicroOp::SrlReg8 { dst }, MicroOp::SrlRegHl, reg_index)]
                }
                _ => unreachable!(),
            },
            0b01 => {
                if reg_index == 6 {
                    vec![MicroOp::BitRegHl { bit }]
                } else {
                    vec![MicroOp::BitReg8 { bit, reg: reg8 }]
                }
            }
            0b10 => {
                if reg_index == 6 {
                    vec![MicroOp::ResRegHl { bit }]
                } else {
                    vec![MicroOp::ResReg8 { bit, reg: reg8 }]
                }
            }
            0b11 => {
                if reg_index == 6 {
                    vec![MicroOp::SetRegHl { bit }]
                } else {
                    vec![MicroOp::SetReg8 { bit, reg: reg8 }]
                }
            }
            _ => unreachable!(),
        };

        (micro_ops, DecodeFlow::NoImm, cycles)
    }

    pub fn decode(&mut self, opcode: u8) -> (Vec<MicroOp>, DecodeFlow, u8) {
        println!("DECODING OPCODE {:02X}", opcode);

        if opcode == 0xCB {
            return (vec![], DecodeFlow::CbPrefix, 4);
        }

        match opcode {
            // =========================
            // 0x00–0x0F
            // =========================
            0x00 => (vec![MicroOp::Nop], DecodeFlow::NoImm, 4),

            0x01 => (
                vec![MicroOp::LdReg16FromImm { dst: Reg16::BC }],
                DecodeFlow::Imm16,
                12,
            ),

            0x02 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::BC,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x03 => (
                vec![MicroOp::IncReg16 { reg: Reg16::BC }],
                DecodeFlow::NoImm,
                8,
            ),

            0x04 => (
                vec![MicroOp::IncReg8 { reg: Reg8::B }],
                DecodeFlow::NoImm,
                4,
            ),

            0x05 => (
                vec![MicroOp::DecReg8 { reg: Reg8::B }],
                DecodeFlow::NoImm,
                4,
            ),

            0x06 => (
                vec![MicroOp::LdReg8FromImm { dst: Reg8::B }],
                DecodeFlow::Imm8,
                8,
            ),

            0x07 => (vec![MicroOp::Rlca], DecodeFlow::NoImm, 4),

            0x08 => (
                vec![MicroOp::LdMemImm16FromReg16 { src: Reg16::SP }],
                DecodeFlow::Imm16,
                20,
            ),

            0x09 => (
                vec![MicroOp::AddReg16 {
                    dst: Reg16::HL,
                    src: Reg16::BC,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x0A => (
                vec![MicroOp::LdAFromMemReg16 { reg: Reg16::BC }],
                DecodeFlow::NoImm,
                8,
            ),

            0x0B => (
                vec![MicroOp::DecReg16 { reg: Reg16::BC }],
                DecodeFlow::NoImm,
                8,
            ),

            0x0C => (
                vec![MicroOp::IncReg8 { reg: Reg8::C }],
                DecodeFlow::NoImm,
                4,
            ),

            0x0D => (
                vec![MicroOp::DecReg8 { reg: Reg8::C }],
                DecodeFlow::NoImm,
                4,
            ),

            0x0E => (
                vec![MicroOp::LdReg8FromImm { dst: Reg8::C }],
                DecodeFlow::Imm8,
                8,
            ),

            0x0F => (vec![MicroOp::Rrca], DecodeFlow::NoImm, 4),

            // =========================
            // 0x10–0x1F
            // =========================
            0x10 => (vec![MicroOp::Stop], DecodeFlow::NoImm, 4),
            0x11 => (
                vec![MicroOp::LdReg16FromImm { dst: Reg16::DE }],
                DecodeFlow::Imm16,
                12,
            ),

            0x12 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::DE,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x13 => (
                vec![MicroOp::IncReg16 { reg: Reg16::DE }],
                DecodeFlow::NoImm,
                8,
            ),

            0x14 => (
                vec![MicroOp::IncReg8 { reg: Reg8::D }],
                DecodeFlow::NoImm,
                4,
            ),

            0x15 => (
                vec![MicroOp::DecReg8 { reg: Reg8::D }],
                DecodeFlow::NoImm,
                4,
            ),

            0x16 => (
                vec![MicroOp::LdReg8FromImm { dst: Reg8::D }],
                DecodeFlow::Imm8,
                8,
            ),

            0x17 => (vec![MicroOp::Rla], DecodeFlow::NoImm, 4),

            0x18 => (vec![MicroOp::JumpRelative], DecodeFlow::Imm8, 0),

            0x19 => (
                vec![MicroOp::AddReg16 {
                    dst: Reg16::HL,
                    src: Reg16::DE,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x1A => (
                vec![MicroOp::LdAFromMemReg16 { reg: Reg16::DE }],
                DecodeFlow::NoImm,
                8,
            ),

            0x1B => (
                vec![MicroOp::DecReg16 { reg: Reg16::DE }],
                DecodeFlow::NoImm,
                8,
            ),

            0x1C => (
                vec![MicroOp::IncReg8 { reg: Reg8::E }],
                DecodeFlow::NoImm,
                4,
            ),

            0x1D => (
                vec![MicroOp::DecReg8 { reg: Reg8::E }],
                DecodeFlow::NoImm,
                4,
            ),

            0x1E => (
                vec![MicroOp::LdReg8FromImm { dst: Reg8::E }],
                DecodeFlow::Imm8,
                8,
            ),

            0x1F => (vec![MicroOp::Rra], DecodeFlow::NoImm, 4),

            // =========================
            // 0x20–0x2F
            // =========================
            0x20 => (
                vec![MicroOp::JumpRelativeIf {
                    flag: 'z',
                    expected: false,
                }],
                DecodeFlow::Imm8,
                0,
            ),

            0x21 => (
                vec![MicroOp::LdReg16FromImm { dst: Reg16::HL }],
                DecodeFlow::Imm16,
                12,
            ),

            0x22 => (
                vec![MicroOp::LdMemFromA, MicroOp::IncReg16 { reg: Reg16::HL }],
                DecodeFlow::NoImm,
                8,
            ),

            0x23 => (
                vec![MicroOp::IncReg16 { reg: Reg16::HL }],
                DecodeFlow::NoImm,
                8,
            ),

            0x24 => (
                vec![MicroOp::IncReg8 { reg: Reg8::H }],
                DecodeFlow::NoImm,
                4,
            ),

            0x25 => (
                vec![MicroOp::DecReg8 { reg: Reg8::H }],
                DecodeFlow::NoImm,
                4,
            ),

            0x26 => (
                vec![MicroOp::LdReg8FromImm { dst: Reg8::H }],
                DecodeFlow::Imm8,
                8,
            ),

            0x27 => (vec![MicroOp::Daa], DecodeFlow::NoImm, 4),

            0x28 => (
                vec![MicroOp::JumpRelativeIf {
                    flag: 'z',
                    expected: true,
                }],
                DecodeFlow::Imm8,
                0,
            ),

            0x29 => (
                vec![MicroOp::AddReg16 {
                    dst: Reg16::HL,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x2A => (
                vec![MicroOp::LdReg8FromMemIncHL { dst: Reg8::A }],
                DecodeFlow::NoImm,
                8,
            ),

            0x2B => (
                vec![MicroOp::DecReg16 { reg: Reg16::DE }],
                DecodeFlow::NoImm,
                8,
            ),
            0x2C => (
                vec![MicroOp::IncReg8 { reg: Reg8::E }],
                DecodeFlow::NoImm,
                4,
            ),
            0x2D => (
                vec![MicroOp::DecReg8 { reg: Reg8::E }],
                DecodeFlow::NoImm,
                4,
            ),

            0x2E => (
                vec![MicroOp::LdReg8FromImm { dst: Reg8::L }],
                DecodeFlow::Imm8,
                8,
            ),

            0x2F => (vec![MicroOp::Cpl], DecodeFlow::NoImm, 4),

            // =========================
            // 0x30–0x3F
            // =========================
            0x30 => (
                vec![MicroOp::JumpRelativeIf {
                    flag: 'c',
                    expected: false,
                }],
                DecodeFlow::Imm8,
                12,
            ),

            0x31 => (
                vec![MicroOp::LdReg16FromImm { dst: Reg16::SP }],
                DecodeFlow::Imm16,
                12,
            ),

            0x32 => (
                vec![MicroOp::LdMemFromReg8DecHL { src: Reg8::A }],
                DecodeFlow::NoImm,
                8,
            ),

            0x33 => (
                vec![MicroOp::IncReg16 { reg: Reg16::SP }],
                DecodeFlow::NoImm,
                8,
            ),

            0x34 => (vec![MicroOp::IncRegHl], DecodeFlow::NoImm, 12),

            0x35 => (vec![], DecodeFlow::NoImm, 12),

            0x36 => (
                vec![MicroOp::LdMemFromImm8 { addr: Reg16::HL }],
                DecodeFlow::Imm8,
                12,
            ),

            0x37 => (vec![MicroOp::Scf], DecodeFlow::NoImm, 4),

            0x38 => (
                vec![MicroOp::JumpRelativeIf {
                    flag: 'c',
                    expected: true,
                }],
                DecodeFlow::Imm8,
                12,
            ),

            0x39 => (
                vec![MicroOp::AddReg16 {
                    dst: Reg16::HL,
                    src: Reg16::SP,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x3A => (
                vec![MicroOp::LdReg8FromMemDecHL { dst: Reg8::A }],
                DecodeFlow::NoImm,
                8,
            ),

            0x3B => (
                vec![MicroOp::DecReg16 { reg: Reg16::SP }],
                DecodeFlow::NoImm,
                8,
            ),

            0x3C => (
                vec![MicroOp::IncReg8 { reg: Reg8::A }],
                DecodeFlow::NoImm,
                4,
            ),

            0x3D => (
                vec![MicroOp::DecReg8 { reg: Reg8::A }],
                DecodeFlow::NoImm,
                4,
            ),

            0x3E => (
                vec![MicroOp::LdReg8FromImm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0x3F => (vec![MicroOp::Ccf], DecodeFlow::NoImm, 4),

            // =========================
            // 0x40–0x4F
            // =========================
            0x40 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::B,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x41 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::B,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x42 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::B,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x43 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::B,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x44 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::B,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x45 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::B,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x46 => (
                vec![MicroOp::LdReg8FromMem {
                    dst: Reg8::B,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x47 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::B,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x48 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::C,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x49 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::C,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x4A => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::C,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x4B => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::C,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x4C => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::C,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x4D => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::C,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x4E => (
                vec![MicroOp::LdReg8FromMem {
                    dst: Reg8::C,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x4F => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::C,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0x50–0x5F
            // =========================
            0x50 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::D,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x51 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::D,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x52 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::D,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x53 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::D,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x54 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::D,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x55 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::D,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x56 => (
                vec![MicroOp::LdReg8FromMem {
                    dst: Reg8::D,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x57 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::D,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x58 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::E,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x59 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::E,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x5A => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::E,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x5B => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::E,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x5C => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::E,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x5D => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::E,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x5E => (
                vec![MicroOp::LdReg8FromMem {
                    dst: Reg8::E,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x5F => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::E,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0x60-0x6F
            // =========================
            0x60 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::H,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x61 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::H,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x62 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::H,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x63 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::H,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x64 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::H,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x65 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::H,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x66 => (
                vec![MicroOp::LdReg8FromMem {
                    dst: Reg8::H,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x67 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::H,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x68 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::L,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x69 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::L,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x6A => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::L,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x6B => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::L,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x6C => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::L,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x6D => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::L,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x6E => (
                vec![MicroOp::LdReg8FromMem {
                    dst: Reg8::L,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x6F => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::L,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0x70-0x7F
            // =========================
            0x70 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::HL,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x71 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::HL,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x72 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::HL,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x73 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::HL,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x74 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::HL,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x75 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::HL,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x76 => (vec![MicroOp::Halt], DecodeFlow::NoImm, 4), // HALT

            0x77 => (
                vec![MicroOp::LdMemFromReg8 {
                    addr: Reg16::HL,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x78 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x79 => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x7A => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x7B => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x7C => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x7D => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x7E => (
                vec![MicroOp::LdReg8FromMem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x7F => (
                vec![MicroOp::LdReg8FromReg8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0x80–0x8F
            // =========================
            0x80 => (
                vec![MicroOp::AddReg8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x81 => (
                vec![MicroOp::AddReg8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x82 => (
                vec![MicroOp::AddReg8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x83 => (
                vec![MicroOp::AddReg8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x84 => (
                vec![MicroOp::AddReg8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x85 => (
                vec![MicroOp::AddReg8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x86 => (
                vec![MicroOp::AddReg8Mem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x87 => (
                vec![MicroOp::AddReg8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x88 => (
                vec![MicroOp::AddCarry8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x89 => (
                vec![MicroOp::AddCarry8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x8A => (
                vec![MicroOp::AddCarry8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x8B => (
                vec![MicroOp::AddCarry8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x8C => (
                vec![MicroOp::AddCarry8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x8D => (
                vec![MicroOp::AddCarry8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x8E => (
                vec![MicroOp::AddCarry8Mem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x8F => (
                vec![MicroOp::AddCarry8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0x90–0x9F
            // =========================
            0x90 => (
                vec![MicroOp::SubReg8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x91 => (
                vec![MicroOp::SubReg8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x92 => (
                vec![MicroOp::SubReg8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x93 => (
                vec![MicroOp::SubReg8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x94 => (
                vec![MicroOp::SubReg8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x95 => (
                vec![MicroOp::SubReg8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x96 => (vec![MicroOp::SubReg8FromMemHl], DecodeFlow::NoImm, 8),

            0x97 => (
                vec![MicroOp::SubReg8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x98 => (
                vec![MicroOp::SubCarry8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x99 => (
                vec![MicroOp::SubCarry8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x9A => (
                vec![MicroOp::SubCarry8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x9B => (
                vec![MicroOp::SubCarry8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x9C => (
                vec![MicroOp::SubCarry8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x9D => (
                vec![MicroOp::SubCarry8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0x9E => (
                vec![MicroOp::SubCarry8Mem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0x9F => (
                vec![MicroOp::SubCarry8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0xA0–0xAF
            // =========================
            0xA0 => (
                vec![MicroOp::AndReg8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA1 => (
                vec![MicroOp::AndReg8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA2 => (
                vec![MicroOp::AndReg8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA3 => (
                vec![MicroOp::AndReg8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA4 => (
                vec![MicroOp::AndReg8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA5 => (
                vec![MicroOp::AndReg8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA6 => (
                vec![MicroOp::AndReg8Mem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0xA7 => (
                vec![MicroOp::AndReg8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA8 => (
                vec![MicroOp::XorReg8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xA9 => (
                vec![MicroOp::XorReg8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xAA => (
                vec![MicroOp::XorReg8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xAB => (
                vec![MicroOp::XorReg8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xAC => (
                vec![MicroOp::XorReg8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xAD => (
                vec![MicroOp::XorReg8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xAE => (
                vec![MicroOp::XorReg8Mem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0xAF => (
                vec![MicroOp::XorReg8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0xB0–0xBF
            // =========================
            0xB0 => (
                vec![MicroOp::OrReg8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB1 => (
                vec![MicroOp::OrReg8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB2 => (
                vec![MicroOp::OrReg8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB3 => (
                vec![MicroOp::OrReg8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB4 => (
                vec![MicroOp::OrReg8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB5 => (
                vec![MicroOp::OrReg8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB6 => (
                vec![MicroOp::OrReg8Mem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0xB7 => (
                vec![MicroOp::OrReg8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB8 => (
                vec![MicroOp::CpReg8 {
                    dst: Reg8::A,
                    src: Reg8::B,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xB9 => (
                vec![MicroOp::CpReg8 {
                    dst: Reg8::A,
                    src: Reg8::C,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xBA => (
                vec![MicroOp::CpReg8 {
                    dst: Reg8::A,
                    src: Reg8::D,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xBB => (
                vec![MicroOp::CpReg8 {
                    dst: Reg8::A,
                    src: Reg8::E,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xBC => (
                vec![MicroOp::CpReg8 {
                    dst: Reg8::A,
                    src: Reg8::H,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xBD => (
                vec![MicroOp::CpReg8 {
                    dst: Reg8::A,
                    src: Reg8::L,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            0xBE => (
                vec![MicroOp::CpReg8Mem {
                    dst: Reg8::A,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),

            0xBF => (
                vec![MicroOp::CpReg8 {
                    dst: Reg8::A,
                    src: Reg8::A,
                }],
                DecodeFlow::NoImm,
                4,
            ),

            // =========================
            // 0xC0–0xCF
            // =========================
            0xC0 => (
                vec![MicroOp::ReturnIf {
                    flag: ('z'),
                    expected: (false),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xC1 => (
                vec![MicroOp::PopReg16 { reg: Reg16::BC }],
                DecodeFlow::NoImm,
                12,
            ),

            0xC2 => (
                vec![MicroOp::JumpAbsoluteIf {
                    flag: 'z',
                    expected: false,
                }],
                DecodeFlow::Imm16,
                0,
            ),

            0xC3 => (vec![MicroOp::JumpAbsolute], DecodeFlow::Imm16, 16),

            0xC4 => (
                vec![MicroOp::CallAbsoluteIf {
                    flag: ('z'),
                    expected: (false),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xC5 => (
                vec![MicroOp::PushReg16 { reg: Reg16::BC }],
                DecodeFlow::NoImm,
                16,
            ),

            0xC6 => (
                vec![MicroOp::AddReg8Imm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0xC7 => (
                vec![MicroOp::Restart { vector: 0x00 }],
                DecodeFlow::NoImm,
                0,
            ),

            0xC8 => (
                vec![MicroOp::ReturnIf {
                    flag: ('z'),
                    expected: (true),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xC9 => (vec![MicroOp::Return], DecodeFlow::NoImm, 0),

            0xCA => (
                vec![MicroOp::JumpAbsoluteIf {
                    flag: 'z',
                    expected: true,
                }],
                DecodeFlow::Imm16,
                0,
            ),

            0xCC => (
                vec![MicroOp::CallAbsoluteIf {
                    flag: ('z'),
                    expected: (true),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xCD => (vec![MicroOp::CallAbsolute], DecodeFlow::Imm16, 0),

            0xCE => (
                vec![MicroOp::AddCarry8Imm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0xCF => (
                vec![MicroOp::Restart { vector: 0x08 }],
                DecodeFlow::NoImm,
                0,
            ),

            // =========================
            // 0xD0–0xDF
            // =========================
            0xD0 => (
                vec![MicroOp::ReturnIf {
                    flag: ('c'),
                    expected: (false),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xD1 => (
                vec![MicroOp::PopReg16 { reg: Reg16::DE }],
                DecodeFlow::NoImm,
                12,
            ),

            0xD2 => (
                vec![MicroOp::JumpAbsoluteIf {
                    flag: 'c',
                    expected: false,
                }],
                DecodeFlow::Imm16,
                0,
            ),

            0xD4 => (
                vec![MicroOp::CallAbsoluteIf {
                    flag: ('c'),
                    expected: (false),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xD5 => (
                vec![MicroOp::PushReg16 { reg: Reg16::DE }],
                DecodeFlow::NoImm,
                16,
            ),

            0xD6 => (
                vec![MicroOp::SubReg8Imm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0xD7 => (
                vec![MicroOp::Restart { vector: 0x10 }],
                DecodeFlow::NoImm,
                0,
            ),

            0xD8 => (
                vec![MicroOp::ReturnIf {
                    flag: ('c'),
                    expected: (true),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xD9 => (vec![MicroOp::Reti], DecodeFlow::NoImm, 0),

            0xDA => (
                vec![MicroOp::JumpAbsoluteIf {
                    flag: 'c',
                    expected: true,
                }],
                DecodeFlow::Imm16,
                0,
            ),

            0xDC => (
                vec![MicroOp::CallAbsoluteIf {
                    flag: ('c'),
                    expected: (true),
                }],
                DecodeFlow::NoImm,
                0,
            ),

            0xDE => (
                vec![MicroOp::SubCarry8Imm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0xDF => (
                vec![MicroOp::Restart { vector: 0x18 }],
                DecodeFlow::NoImm,
                0,
            ),

            // =========================
            // 0xE0–0xEF
            // =========================
            0xE0 => (vec![MicroOp::LdA8FromA], DecodeFlow::Imm8, 12),

            0xE1 => (
                vec![MicroOp::PopReg16 { reg: Reg16::HL }],
                DecodeFlow::NoImm,
                12,
            ),

            0xE2 => (vec![MicroOp::LdCFromA], DecodeFlow::NoImm, 8),

            0xE5 => (
                vec![MicroOp::PushReg16 { reg: Reg16::HL }],
                DecodeFlow::NoImm,
                16,
            ),

            0xE6 => (
                vec![MicroOp::AndReg8Imm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0xE7 => (
                vec![MicroOp::Restart { vector: 0x20 }],
                DecodeFlow::NoImm,
                0,
            ),

            0xE8 => (vec![MicroOp::AddImmToSP], DecodeFlow::Imm8, 8),

            0xE9 => (vec![MicroOp::JumpHL], DecodeFlow::Imm8, 0),

            0xEA => (vec![MicroOp::LdMemAbsFromA], DecodeFlow::Imm16, 16),
            0xEE => (
                vec![MicroOp::XorReg8Imm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0xEF => (
                vec![MicroOp::Restart { vector: 0x28 }],
                DecodeFlow::NoImm,
                0,
            ),

            // =========================
            // 0xF0–0xFF
            // =========================
            0xF0 => (vec![MicroOp::LdAFromA8], DecodeFlow::Imm8, 12),

            0xF1 => (
                vec![MicroOp::PopReg16 { reg: Reg16::AF }],
                DecodeFlow::NoImm,
                12,
            ),

            0xF2 => (vec![MicroOp::LdAFromC], DecodeFlow::NoImm, 8),

            0xF3 => (vec![MicroOp::Di], DecodeFlow::NoImm, 4),

            0xF5 => (
                vec![MicroOp::PushReg16 { reg: Reg16::AF }],
                DecodeFlow::NoImm,
                16,
            ),

            0xF6 => (
                vec![MicroOp::OrReg8Imm { dst: Reg8::A }],
                DecodeFlow::Imm8,
                8,
            ),

            0xF7 => (
                vec![MicroOp::Restart { vector: 0x30 }],
                DecodeFlow::NoImm,
                0,
            ),

            0xF8 => (vec![MicroOp::LdHlFromSpPlusImm8], DecodeFlow::Imm8, 12),

            0xF9 => (
                vec![MicroOp::LdReg16FromMem {
                    dst: Reg16::SP,
                    src: Reg16::HL,
                }],
                DecodeFlow::NoImm,
                8,
            ),
            0xFA => (vec![MicroOp::LdAFromMemAbs], DecodeFlow::Imm16, 16),
            0xFB => (vec![MicroOp::Ei], DecodeFlow::NoImm, 4),

            0xFE => (
                vec![MicroOp::CpReg8Imm { dst: Reg8::A }],
                DecodeFlow::NoImm,
                8,
            ),

            0xFF => (
                vec![MicroOp::Restart { vector: 0x38 }],
                DecodeFlow::NoImm,
                0,
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

            MicroOp::LdReg16FromMem { dst, src } => {
                let addr = self.regs.get16(src);
                let lo = self.inter.read_byte(addr) as u16;
                let hi = self.inter.read_byte(addr.wrapping_add(1)) as u16;
                self.regs.set16(dst, (hi << 8) | lo);
            }

            MicroOp::LdMemFromReg8 { addr, src } => {
                let address = self.regs.get16(addr);
                let value = self.regs.get8(src);
                self.inter.write_byte(address, value);
            }

            MicroOp::LdMemFromImm8 { addr } => {
                let address = self.regs.get16(addr);
                self.inter.write_byte(address, self.imm8);
            }

            MicroOp::LdAFromMemReg16 { reg } => {
                let addr = self.regs.get16(reg);
                let val = self.inter.read_byte(addr);
                self.regs.set8(Reg8::A, val);
            }

            MicroOp::LdMemAbsFromA => {
                let addr = self.imm16;
                let a = self.regs.get8(Reg8::A);
                self.inter.write_byte(addr, a);
            }

            MicroOp::LdAFromMemAbs => {
                let addr = self.imm16;
                let val = self.inter.read_byte(addr);
                self.regs.set8(Reg8::A, val);
            }

            MicroOp::LdHlFromSpPlusImm8 => {
                let imm = self.imm8 as i8 as i16;
                let sp = self.regs.sp;
                let result = sp.wrapping_add(imm as u16);

                let half_carry = ((sp & 0xF) + ((imm as u16) & 0xF)) > 0xF;
                let carry = ((sp & 0xFF) + ((imm as u16) & 0xFF)) > 0xFF;

                self.regs.set16(Reg16::HL, result);

                self.flags.set_flag('z', false);
                self.flags.set_flag('n', false);
                self.flags.set_flag('h', half_carry);
                self.flags.set_flag('c', carry);
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
                let result = value.wrapping_sub(1);
                self.regs.set16(reg, result);

                println!("DEC BC: {:04X} → {:04X}", value, result);
            }

            MicroOp::IncRegHl => {
                let addr = self.regs.get16(Reg16::HL);
                let old = self.inter.read_byte(addr);
                let result = old.wrapping_add(1);

                self.flags.set_flag('z', result == 0);
                self.flags.set_flag('n', false);
                self.flags.set_flag('h', (old & 0x0F) == 0x0F);

                self.inter.write_byte(addr, result);
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

            MicroOp::AddReg8Imm { dst } => {
                let a = self.regs.get8(dst);
                let addr = self.imm8;

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

            MicroOp::AddCarry8Imm { dst } => {
                let a = self.regs.get8(dst);
                let carry = if self.flags.get_flag('C') { 1 } else { 0 };
                let addr = self.imm8;

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

            MicroOp::SubReg8FromMemHl => {
                let addr = self.regs.get16(Reg16::HL);
                let val = self.inter.read_byte(addr);

                let a = self.regs.get8(Reg8::A);
                let alu_out = self.alu.sub_8bit(a, val);

                self.regs.set8(Reg8::A, alu_out.result);

                self.flags.set_flag('z', alu_out.z);
                self.flags.set_flag('n', alu_out.n);
                self.flags.set_flag('h', alu_out.h);
                self.flags.set_flag('c', alu_out.c);
            }

            MicroOp::SubReg8Imm { dst } => {
                let a = self.regs.get8(dst);
                let addr = self.imm8;

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

            MicroOp::SubCarry8Imm { dst } => {
                let a = self.regs.get8(dst);
                let carry = self.flags.get_flag('c');
                let addr = self.imm8;

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

            MicroOp::XorReg8Imm { dst } => {
                let a = self.regs.get8(dst);
                let addr = self.imm8;

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

            MicroOp::CpReg8Imm { dst } => {
                let a = self.regs.get8(dst);
                let addr = self.imm8;

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

            MicroOp::OrReg8Imm { dst } => {
                let a = self.regs.get8(dst);
                let addr = self.imm8;

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

            MicroOp::AndReg8Imm { dst } => {
                let a = self.regs.get8(dst);
                let addr = self.imm8;
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
                self.inter
                    .write_byte(sp.wrapping_add(1), (value >> 8) as u8);
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
                self.cycles += 16;
            }

            MicroOp::JumpAbsoluteIf { flag, expected } => {
                if self.flags.get_flag(flag) == expected {
                    self.regs.set16(Reg16::PC, self.imm16);
                    self.cycles += 16;
                } else {
                    self.cycles += 12;
                }
            }

            MicroOp::JumpRelative => {
                let offset = self.imm8 as i8 as i16;
                let pc = self.regs.get16(Reg16::PC);
                self.regs.set16(Reg16::PC, pc.wrapping_add_signed(offset));
                self.cycles += 12;
            }

            MicroOp::JumpRelativeIf { flag, expected } => {
                if self.flags.get_flag(flag) == expected {
                    let pc = self.regs.get16(Reg16::PC);
                    let offset = self.imm8 as i8 as i16;
                    let new_pc = pc.wrapping_add(2).wrapping_add(offset as u16); // PC after JR + signed offset
                    self.regs.set16(Reg16::PC, new_pc);
                    self.cycles += 12;
                } else {
                    self.regs.set16(Reg16::PC, self.regs.get16(Reg16::PC) + 2); // skip opcode + offset
                    self.cycles += 8;
                }
            }

            MicroOp::JumpHL => {
                let hl = self.regs.get16(Reg16::HL);
                self.regs.set16(Reg16::PC, hl);
                self.cycles += 4;
            }

            MicroOp::CallAbsolute => {
                let pc = self.regs.get16(Reg16::PC);
                self.push_16bit(pc);
                self.regs.set16(Reg16::PC, self.imm16);
                self.cycles += 24;
            }

            MicroOp::CallAbsoluteIf { flag, expected } => {
                if self.flags.get_flag(flag) == expected {
                    let pc = self.regs.get16(Reg16::PC);
                    self.push_16bit(pc);
                    self.regs.set16(Reg16::PC, self.imm16);
                    self.cycles += 24;
                } else {
                    self.cycles += 12;
                }
            }

            MicroOp::Return => {
                let addr = self.pop_16bit();
                self.regs.set16(Reg16::PC, addr);
                self.cycles += 16;
            }

            MicroOp::ReturnIf { flag, expected } => {
                if self.flags.get_flag(flag) == expected {
                    let addr = self.pop_16bit();
                    self.regs.set16(Reg16::PC, addr);
                    self.cycles += 20;
                } else {
                    self.cycles += 8;
                }
            }

            MicroOp::Reti => {
                let addr = self.pop_16bit();
                self.regs.set16(Reg16::PC, addr);
                self.interrupt = true;
                self.cycles += 16;
            }

            MicroOp::Restart { vector } => {
                let pc = self.regs.get16(Reg16::PC);
                self.push_16bit(pc);
                self.regs.set16(Reg16::PC, vector);
                self.cycles += 16;
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
            MicroOp::AddImmToSP => {
                let sp = self.regs.sp;
                let imm = self.imm8;
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
        }
    }
}

#[cfg(test)]
mod tests;
