use std::collections::btree_map::Values;

use crate::mmu::Interconnect;
use crate::registers::Registers;
mod microops;

pub struct Cpu//<'a>
{
    regs: Registers,

    flags: Flags,

    interrupt: bool,

    interrupt_enable_next: bool,

    halted: bool,

    inter: Interconnect,//<'a>,

    instruction_cycles: u8,
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

            inter: inter,
            interrupt: true,
            interrupt_enable_next:  true,
            halted: false,
            instruction_cycles: 0,

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
            MicroOp::Nop =>
            {

            }

            MicroOp::LdReg8FromReg8   { dst: reg_8, src: reg_8 } =>
            {
                let v = self.regs.get8(src);
                self.regs.set8(dst, v);
                self.cycles += 1;
            }

            MicroOp::LdReg8FromMem    { dst: reg_8, src: reg_16 } =>
            {
                let addr = self.regs.get16(src);
                self.regs.set8(dst, v);
                self.cycles += 1;

            }

            LdReg16FromMem   { reg: reg_16, mem: reg_16 } =>
            {
                let addr = self.regs.get16(reg);
                let lo = self.inter.read(addr) as u16;
                let hi = self.inter.read(addr.wrapping_add(1)) as u16;
                let value = (hi << 8) | lo;
                self.regs.set16(mem, value);
            }

            LdReg8FromReg16  { dst: reg_8,  src: reg_16 } =>
            {

            }

            IncReg8          { reg: reg_8 } =>
            {
                let v = self.regs.get8(reg);
                self.regs.set8()


            }

            DecReg8          { reg: reg_8 } =>
            {

            }

            IncReg16         { reg: reg_16 } =>
            {

            }

            DecReg16         { reg: reg_16 } =>
            {

            }

            AddReg8          { dst: reg_8, src: reg_8 } =>
            {

            }

            AddReg16         { dst: Reg16, src: Reg16 } =>
            {

            }

            AddCarry8        { dst: Reg16, src: Reg8 } =>
            {

            }

            AddCarry16       { dst: Reg8,  src: Reg16 } =>
            {

            }

            SubReg8          { dst: Reg8,  src: Reg8 } =>
            {

            }

            SubReg16         { dst: Reg8,  src: Reg16 } =>
            {

            }

            SubCarry8        { dst: Reg8,  src: Reg8 } =>
            {

            }

            SubCarry16       { dst: Reg8,  src: Reg16 } =>
            {

            }

            PushReg16        { reg: reg_16 } =>
            {

            }

            PopReg16         { reg: reg_16}
                    

        }
    }
}