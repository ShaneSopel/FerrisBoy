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

            MicroOp::LdReg8FromReg8 { dst, src } =>
            {
                let v = self.regs.get8(src);
                self.regs.set8(dst, v);
                self.cycles += 1;
            }

            MicroOp::LdReg8FromMem { dst, src } =>
            {
                let addr = self.regs.get16(src);
                self.regs.set8(dst, v);
                self.cycles += 1;

            }

            MicroOp::LdReg16FromMem { dst, src } =>
            {
                let addr = self.regs.get16(src);
                let lo = self.inter.read(addr) as u16;
                let hi = self.inter.read(addr.wrapping_add(1)) as u16;
                let value = (hi << 8) | lo;
                self.regs.set16(dst, value);
            }

            MicroOp::LdReg8FromReg16  { dst,  src, byte } =>
            {

            }

            MicroOp::IncReg8 { reg } =>
            {
                let v = self.regs.get8(reg);
                self.regs.set8(reg, v.wrapping_add(1));
            }

            MicroOp::DecReg8 { reg } =>
            {
                let v = self.regs.get8(reg);
                self.regs.set8(reg, v.wrapping_sub(1));
            }

            MicroOp::IncReg16 { reg } =>
            {

            }

            MicroOp::DecReg16 { reg } =>
            {

            }

            MicroOp::AddReg8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.add_8bit(self, a, b);
                self.regs.set8(dst, result);
            }

            MicroOp::AddReg16 { dst, src } =>
            {


            }

            MicroOp::AddCarry8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.adc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            /*MicroOp::AddCarry16 { dst, src } =>
            {

            }

            MicroOp::SubReg8 { dst ,  src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sub_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::SubReg16 { dst, src } =>
            {

            }

            MicroOp::SubCarry8 { dst, src} =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sbc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::SubCarry16 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sbc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::PushReg16 { reg } =>
            {
                let a = self.regs.get16(dst);

                let result = self.alu.push_word();

                self.regs.set8(dst, result);

            }

            MicroOp::PopReg16 { reg } =>
            {

            }*/
        }
    }
}