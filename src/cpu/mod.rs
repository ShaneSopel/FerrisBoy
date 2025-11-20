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
        let byte = self.interconnect.read8(self.regs.get16(Reg16::PC));
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
                let addr = self.regs.get16(src);
                let value = self.inter.read(addr);
                self.regs.set8(dst, value);
            }

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
                let value = self.regs.get16(reg)
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

                self.regs.set8(a, results);
            }

            MicroOp::OrReg8 { dst, src } =>
            {
                let a = self.regs.get8(dst);
                let b = self.regs.get8(src);

                let results = self.alu.or_8bit(self, a, b);

                self.regs.set8(dst, results);
            }

            MicroOp::PushReg16 { reg } =>
            {
                let val = cpu.get_register_16(src);
                let hi = (val >> 8) as u8;
                let lo = val as u8;

                self.regs.SP = self.regs.SP.wrapping_sub(1);
                interconnect.write8(self.regs.SP, hi);

                self.regs.SP = self.regs.SP.wrapping_sub(1);
                interconnect.write8(self.regs.SP, lo);
            }

            MicroOp::PopReg16 { reg } =>
            {
                let lo = interconnect.read8(cpu.SP);
                self.regs.SP = self.regs.SP.wrapping_add(1);

                let hi = interconnect.read8(cpu.SP);
                self.regs.SP = self.regs.SP.wrapping_add(1);

                let val = ((hi as u16) << 8) | lo as u16;
                self.regs.set16(dst, val);
            }

            MicroOp::JumpAbsolute { addr } =>
            {
                self.regs.PC = addr;
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

            MicroOp::Illegal { opcode } =>
            {

            }
        }
    }
}