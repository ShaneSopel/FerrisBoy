pub mod alu;
pub mod registers;
pub mod microops;

use crate::interconnect::Interconnect;
use crate::cpu::registers::{Registers, Flags, Reg8, Reg16};
use crate::cpu::microops::{MicroOp, ByteSel};
use crate::cpu::alu::Alu;

pub struct Cpu
{
    regs: Registers,

    flags: Flags,

    alu: Alu,

    interrupt: bool,

    interrupt_enable_next: bool,

    halted: bool,

    inter: Interconnect,

    cycles: u8,
}

impl Cpu
{
    pub fn new(inter: Interconnect) -> Cpu
    {
        let regs = Registers
        {
            PC:0x100,
            SP: 0,
            A: 0,
            B: 0,
            F: Flags::new(),
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            IE: 0,
            IR: 0,
            
        };

        Cpu
        {
            regs: regs,
            flags: Flags 
            {
                Z: false,
                H: false,
                N: false,
                C: false,
            },

            alu: Alu::new(),
            inter: inter,
            interrupt: true,
            interrupt_enable_next:  true,
            halted: false,
            cycles: 0,
        }


    }

    pub fn step(&mut self)
    {
    }

    fn fetch8(&mut self) -> u8 
    {
        let byte = self.inter.read_byte(self.regs.get16(Reg16::PC));
        self.regs.set16(Reg16::PC, self.regs.get16(Reg16::PC).wrapping_add(1));
        self.cycles += 1; // 1 machine cycle for fetch
        byte
    }

    pub fn decode(opcode: u8) -> Vec<MicroOp>
    {
        match opcode
        {
            0x00 => vec![MicroOp::Nop],
            
            _ => panic!("Unimplemented opcode: {:02X}", opcode),
        }
    }

    pub fn execute_microop(&mut self, op: MicroOp)
    {
        match op
        {
            MicroOp::Nop =>
            {

            }

            MicroOp::Halt =>
            {

            }

            MicroOp::Stop =>
            {

            }
            
            //Load instructions
            MicroOp::LdReg8FromReg8 { dst, src } =>
            {
                let v = self.regs.get8(src);
                self.regs.set8(dst, v);
                self.cycles += 1;
            }

            MicroOp::LdReg8FromMem { dst, src } =>
            {
                  let addr = self.regs.get16(src);
                let value = self.inter.read_byte(addr);
                self.regs.set8(dst, value);
                self.cycles += 1;

            }

            MicroOp::LdMemFromReg8 { addr , src } =>
            {
                let value = self.regs.get8(src);
                self.inter.write_byte(addr, value);
                self.cycles += 1;
            }

            MicroOp::LdReg16FromMem { dst, src } =>
            {
                let addr = self.regs.get16(src);
                let lo = self.inter.read_byte(addr) as u16;
                let hi = self.inter.read_byte(addr.wrapping_add(1)) as u16;
                let value = (hi << 8) | lo;
                self.regs.set16(dst, value);
            }

            MicroOp::LdReg8FromReg16  { dst,  src, byte } =>
            {
                let addr = self.regs.get16(src);
                let value = self.inter.read_byte(addr);
                self.regs.set8(dst, value);
            }

            //Logical 
            MicroOp::IncReg8 { reg } =>
            {
                let value = self.regs.get8(reg);
                self.regs.set8(reg, value.wrapping_add(1));
            }

            MicroOp::DecReg8 { reg } =>
            {
                let value = self.regs.get8(reg);
                self.regs.set8(reg, value.wrapping_sub(1));
            }

            MicroOp::IncReg16 { reg } =>
            {
                let value = self.regs.get16(reg);
                self.regs.set16(reg, value.wrapping_add(1));
            }

            MicroOp::DecReg16 { reg } =>
            {
                let value = self.regs.get16(reg);
                self.regs.set16(reg, value.wrapping_sub(1));
            }

            MicroOp::AddReg8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.add_8bit(self, a, b);
                self.regs.set8(dst, result);
            }

            MicroOp::AddReg8Mem { dst , src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get16(src);
                let value = self.inter.read_byte(b);

                let result = self.alu.add_8bit(self, a, b);
                self.regs.set8(dst, result);
            }

            AddReg8Imm { dst , src } => 
            {
                let a = self.regs.get8(dst);

                let result = self.alu.add_8bit(self, a, src);
                self.regs.set8(dst, result);
            }

            MicroOp::AddReg16 { dst, src } =>
            {
                let a = self.regs.get16(dst);
                let a = self.regs.get16(src);

                let result = self.alu.add_16bit(self, a, b);

                self.regs.set16(dst, result);
            }

            MicroOp::AddCarry8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.adc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::SubReg8 { dst ,  src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sub_8bit(self, a, b);

                self.regs.set8(dst, result);
            }


            MicroOp::SubCarry8 { dst, src} =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sbc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::XorReg8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.xor_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::CpReg8 { a, src } =>
            {
                let a = self.regs.A;
                let b = self.regs.get8(src);

                let results = self.alu.cp_8bit(self, a, b);
            }

            MicroOp::CpReg8Mem { a, src } =>
            {
                let a = self.regs.A;
                let b = self.regs.get16(src);
                let value = self.inter.read_byte(b);

                let results = self.alu.cp_8bit(self, a, value);
            }

            MicroOp::OrReg8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let results = self.alu.or_8bit(self, a, b);

                self.regs.set8(dst, results);
            }

            OrReg8Mem  { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get16(src);
                let value = self.inter.read_byte(b);

                let results = self.alu.or_8bit(self, a, value);

                self.regs.set8(dst, results);
            }

            OrReg8Imm { dst, src } =>
            {
                let a = self.regs.get8(dst);

                let results = self.alu.or_8bit(self, a, src);
                self.regs.set8(dst, results);
            }

            AndReg8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let results = self.alu.and_8bit(self, a, b);
                self.regs.set8(dst, results);

            }

            MicroOp::PushReg16 { reg } =>
            {
                let val = self.regs.get16(reg);
                let hi = (val >> 8) as u8;
                let lo = val as u8;

                self.regs.SP = self.regs.SP.wrapping_sub(1);
                self.inter.write_byte(self.regs.SP, hi);

                self.regs.SP = self.regs.SP.wrapping_sub(1);
                self.inter.write_byte(self.regs.SP, lo);
            }

            MicroOp::PopReg16 { reg } =>
            {
                let lo = self.inter.read_byte(self.regs.SP);
                self.regs.SP = self.regs.SP.wrapping_add(1);

                let hi = self.inter.read_byte(self.regs.SP);
                self.regs.SP = self.regs.SP.wrapping_add(1);

                let val = ((hi as u16) << 8) | lo as u16;
                self.regs.set16(dst, val);
            }

            MicroOp::JumpAbsolute { addr } =>
            {
                self.regs.PC = addr;
            }

            JumpAbsoluteIf   { addr, flag, expected} =>
            {
                let value = self.flags.get_flag(flag);
                let taken = value == expected;

                if taken
                {
                    self.regs.set16(Reg16::PC, addr);
                }

                taken
            }

            MicroOp::JumpRelative { offset } =>
            {
                let pc = self.regs.PC.wrapping_add(offset as u16);
                self.regs.PC = pc;
            }

            MicroOp::JumpRelativeIf { offset, flag, expected } => 
            {
                if self.flags.get_flag(flag) == expected 
                {
                    let new_pc = self.regs.PC.wrapping_add(offset as u16);
                    self.regs.PC = new_pc;
                }
            }

            MicroOp::Return =>
            {

            }

            MicroOp::Restart { vector } =>
            {

            }

            MicroOp::Di =>
            {
                self.interrupt = false;
            }

            MicroOp::Ei =>
            {
                self.interrupt = true;
            }

            MicroOp::Cpl =>
            {
                self.regs.A = !self.regs.A;

                self.flags.set_flag(Flags::N, true);
                self.flags.set_flag(Flags::H, true);
            }

            MicroOp::Ccf =>
            {
                let carry = self.flags.get_flag(Flags::C);

                self.flags.set_flag(Flags::C, !carry);

                self.flags.set_flag(Flags::N, false);
                self.flags.set_flag(Flags::H, false);
            }

            MicroOp::Scf =>
            {
                self.flags.set_flag(Flags::C, true);
                
                self.flags.set_flag(Flags::N, false);
                self.flags.set_flag(Flags::H, false);
            }


            MicroOp::Daa => 
            {
                let mut a = self.regs.A;
                let mut correction: u8 = 0;
                let mut carry = self.flags.get_flag(Flags::C);

                let n = self.flags.get_flag(Flags::N);

                if !n 
                {
                    // After ADD
                    if self.flags.get_flag(Flags::H) || (a & 0x0F) > 9 {
                        correction |= 0x06;
                    }
                    if carry || a > 0x99 {
                        correction |= 0x60;
                        carry = true;
                    }
                    a = a.wrapping_add(correction);
                } 
                else 
                {
                    // After SUB
                    if self.flags.get_flag(Flags::H) {
                        correction |= 0x06;
                    }
                    if carry {
                        correction |= 0x60;
                    }
                    a = a.wrapping_sub(correction);
                }

                self.regs.A = a;

                // Set flags
                self.flags.set_flag(Flags::Z, a == 0);
                self.flags.set_flag(Flags::H, false);
                self.flags.set_flag(Flags::C, carry);
            }

            MicroOp::Illegal { opcode } =>
            {
                //print("illegal opcode");
            }
        }
    }
}