use std::collections::btree_map::Values;

pub mod cpu;
pub mod alu;
pub mod registers;
pub mod microops;

use crate::interconnect::Interconnect;
use crate::cpu::registers::{Registers, Reg8, Reg16};
use crate::cpu::microops::MicroOp;
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

struct Flags
{
    Z: bool, // Zero

    H: bool, // Half Carry Flag

    N: bool, // Subtract Flag

    C: bool, // Carry Flag
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
            F: 0,
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

            alu: alu,
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
            


        }
    }



    pub fn execute_microop(&mut self, op: MicroOp)
    {
        match op
        {
           /* MicroOp::Nop =>
            {

            }

            MicroOp::LdReg8FromReg8 { dst: reg_8, src: reg_8 } =>
            {
                let v = self.regs.get8(src);
                self.regs.set8(dst, v);
                self.cycles += 1;
            }

            MicroOp::LdReg8FromMem { dst: reg_8, src: reg_16 } =>
            {
                let addr = self.regs.get16(src);
                self.regs.set8(dst, v);
                self.cycles += 1;

            }

            MicroOp::LdReg16FromMem { dst: reg_16, mem: reg_16 } =>
            {
                let addr = self.regs.get16(dst);
                let lo = self.inter.read(addr) as u16;
                let hi = self.inter.read(addr.wrapping_add(1)) as u16;
                let value = (hi << 8) | lo;
                self.regs.set16(mem, value);
            }

            MicroOp::LdReg8FromReg16  { dst: reg_8,  src: reg_16 } =>
            {

            }

            MicroOp::IncReg8 { reg: reg_8 } =>
            {
                let v = self.regs.get8(reg);
                self.regs.set8(v.wrapping_add(1));
            }

            MicroOp::DecReg8 { reg: reg_8 } =>
            {

            }

            MicroOp::IncReg16 { reg: reg_16 } =>
            {

            }

            MicroOp::DecReg16 { reg: reg_16 } =>
            {

            }

            MicroOp::AddReg8 { dst: reg_8, src: reg_8 } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.add_8bit(self, a, b);
                self.regs.set8(dst, result);
            }

            MicroOp::AddReg16 { dst: Reg16, src: Reg16 } =>
            {

            }*/

            MicroOp::AddCarry8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.adc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            /*MicroOp::AddCarry16 { dst: Reg8,  src: Reg16 } =>
            {

            }

            MicroOp::SubReg8 { dst: Reg8,  src: Reg8 } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sub_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::SubReg16 { dst: Reg8,  src: Reg16 } =>
            {

            }

            MicroOp::SubCarry8 { dst: Reg8,  src: Reg8 } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sbc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::SubCarry16 { dst: Reg8,  src: Reg16 } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let result = self.alu.sbc_8bit(self, a, b);

                self.regs.set8(dst, result);
            }

            MicroOp::PushReg16 { reg: reg_16 } =>
            {
                let a = self.regs.get16(dst);

                let result = self.alu.push_word();

                self.regs.set8(dst, result);

            }

            MicroOp::PopReg16 { reg: reg_16} =>
            {

            }*/
        }
    }
}