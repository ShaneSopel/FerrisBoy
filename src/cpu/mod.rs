pub mod alu;
pub mod microops;
pub mod registers;

use crate::cpu::alu::Alu;
use crate::cpu::microops::{ByteSel, MicroOp};
use crate::cpu::registers::{Flags, Reg16, Reg8, Registers};
use crate::interconnect::Interconnect;

pub struct Cpu {
    regs: Registers,

    flags: Flags,

    alu: Alu,

    interrupt: bool,

    interrupt_enable_next: bool,

    halted: bool,

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
            ie: 0,
            ir: 0,
        };

        Cpu {
            regs: regs,
            flags: Flags {
                z: false,
                h: false,
                n: false,
                c: false,
            },

            alu: Alu::new(),
            inter: inter,
            interrupt: true,
            interrupt_enable_next: true,
            halted: false,
            cycles: 0,
        }
    }

    pub fn step(&mut self) {}

    fn fetch8(&mut self) -> u8 {
        let byte = self.inter.read_byte(self.regs.get16(Reg16::PC));
        self.regs
            .set16(Reg16::PC, self.regs.get16(Reg16::PC).wrapping_add(1));
        self.cycles += 1; // 1 machine cycle for fetch
        byte
    }

    pub fn decode(opcode: u8) -> Vec<MicroOp> {
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
            //0x06 => vec![MicroOp::LdReg8}]
            //0x07 => vec![MicroOp::]
            //0x08 => vec![MicroOp::Ld]
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
            //0x0E => vec![MicroOp::L]
            //0x0F => vec![]
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
            //0x16 => vec![MicroOp::LdReg8}]
            //0x17 => vec![MicroOp::]
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
            //0x1E => vec![MicroOp::L]
            //0x1F => vec![]
            0x20 => vec![MicroOp::JumpRelativeIf {
                offset: (8),
                flag: ('z'),
                expected: (false),
            }],
            0x21 => vec![MicroOp::LdReg16FromMem {
                dst: Reg16::HL,
                src: Reg16::PC,
            }],
            //0x22
            0x23 => vec![MicroOp::IncReg16 { reg: (Reg16::HL) }],
            0x24 => vec![MicroOp::IncReg8 { reg: (Reg8::H) }],
            0x25 => vec![MicroOp::DecReg8 { reg: (Reg8::H) }],
            //0x26 => vec![MicroOp::LdReg8}]
            //0x27 => vec![MicroOp::]
            0x28 => vec![MicroOp::JumpRelativeIf {
                offset: (8),
                flag: ('z'),
                expected: (true),
            }],
            0x29 => vec![MicroOp::AddReg16 {
                dst: (Reg16::HL),
                src: (Reg16::HL),
            }],
            //0x2A => vec![]
            0x2B => vec![MicroOp::DecReg16 { reg: (Reg16::HL) }],
            0x2C => vec![MicroOp::IncReg8 { reg: (Reg8::L) }],
            0x2D => vec![MicroOp::DecReg8 { reg: (Reg8::L) }],
            //0x2E => vec![],
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
            //0x32 => vec![MicroOp::]
            0x33 => vec![MicroOp::IncReg16 { reg: (Reg16::SP) }],
            0x34 => vec![MicroOp::IncReg16 { reg: (Reg16::HL) }],
            0x35 => vec![MicroOp::DecReg16 { reg: (Reg16::HL) }],
            //0x36
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
            //0x3A
            0x3B => vec![MicroOp::DecReg16 { reg: (Reg16::SP) }],
            0x3C => vec![MicroOp::IncReg8 { reg: (Reg8::A) }],
            0x3D => vec![MicroOp::DecReg8 { reg: (Reg8::A) }],
            //0x3E
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
            0x46 => vec![MicroOp::LdReg8FromReg16 { dst: (Reg8::B), src: (Reg16::HL) }],
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
            0x4E => vec![MicroOp::LdReg8FromReg16 { dst: (Reg8::C), src: (Reg16::HL)}],
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
            0x56 => vec![MicroOp::LdReg8FromReg16 { dst: (Reg8::D), src: (Reg16::HL)}],
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
            0x5E => vec![MicroOp::LdReg8FromReg16 { dst: (Reg8::E), src: (Reg16::HL)}],
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
            0x66 => vec![MicroOp::LdReg8FromReg16 { dst: (Reg8::H), src: (Reg16::HL)}],
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
            0x6E => vec![MicroOp::LdReg8FromReg16 { dst: (Reg8::L), src: (Reg16::HL)}],
            0x6F => vec![MicroOp::LdReg8FromReg8 {
                dst: (Reg8::L),
                src: (Reg8::A),
            }],

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
                self.cycles += 1;
            }

            MicroOp::LdReg8FromMem { dst, src } => {
                let addr = self.regs.get16(src);
                let value = self.inter.read_byte(addr);
                self.regs.set8(dst, value);
                self.cycles += 1;
            }

            MicroOp::LdMemFromReg8 { addr, src } => {
                let value = self.regs.get8(src);
                let address = self.regs.get16(addr);
                self.inter.write_byte(address, value);
                self.cycles += 1;
            }

            MicroOp::LdReg16FromMem { dst, src } => {
                let addr = self.regs.get16(src);
                let lo = self.inter.read_byte(addr) as u16;
                let hi = self.inter.read_byte(addr.wrapping_add(1)) as u16;
                let value = (hi << 8) | lo;
                self.regs.set16(dst, value);
            }

            MicroOp::LdReg8FromReg16 { dst, src } => {
                let addr = self.regs.get16(src);
                let value = self.inter.read_byte(addr);
                self.regs.set8(dst, value);
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

            MicroOp::AddReg8Imm { dst, src } => {
                let a = self.regs.get8(dst);

                let alu_out = self.alu.add_8bit(a, src);

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

            MicroOp::CpReg8 { a, src } => {
                let na = self.regs.a;
                let b = self.regs.get8(src);

                let results = self.alu.cp_8bit(na, b);
            }

            MicroOp::CpReg8Mem { a, src } => {
                let a = self.regs.a;
                let b = self.regs.get16(src);
                let value = self.inter.read_byte(b);

                let results = self.alu.cp_8bit(a, value);
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

            MicroOp::OrReg8Imm { dst, src } => {
                let a = self.regs.get8(dst);

                let alu_out = self.alu.or_8bit(a, src);

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
                let address = self.regs.get16(addr);
                self.regs.pc = address;
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

            MicroOp::CallAbsolute { addr } => {}

            MicroOp::CallAbsoluteIf {
                addr,
                flag,
                expected,
            } => {}

            MicroOp::Return {} => {}

            MicroOp::ReturnIf { flag, expected } => {}

            MicroOp::Restart { vector } => {}

            MicroOp::Di => {
                self.interrupt = false;
            }

            MicroOp::Ei => {
                self.interrupt = true;
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

                // Set flags
                self.flags.set_flag('Z', a == 0);
                self.flags.set_flag('H', false);
                self.flags.set_flag('C', carry);
            }

            MicroOp::RlReg8 { dst } => {}
            MicroOp::RlcReg8 { dst } => {}

            MicroOp::RrReg8 { dst } => {}

            MicroOp::RrcReg8 { dst } => {}

            MicroOp::SlaReg8 { dst } => {}

            MicroOp::SraReg8 { dst } => {}

            MicroOp::SrlReg8 { dst } => {}

            MicroOp::SwapReg8 { dst } => {}

            MicroOp::AddImmToSP { imm } => {}

            MicroOp::LdHLSPPlusR8 { r8 } => {}

            MicroOp::Illegal { opcode } => {
                //print("illegal opcode");
            }
        }
    }
}
