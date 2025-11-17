use crate::mmu::Interconnect;

//use crate::cpu::instructions::process_instruction;

//mod instructions;

//mod cpu
//{
//    use sdl2::{libc::regex_t, sys::valloc};

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

struct Registers
{
    PC: u16,

    SP: u16,

    A: u8,

    F: Flags,

    B: u8,
    
    C: u8,

    D: u8,
    
    E: u8,

    H: u8,
    
    L: u8,

    IE: u16,

    IR: u16

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
        process_instruction(self);
    }


    pub fn execute_microop(&mut self, op: MicroOp)
    {
        match op
        {

        }
    }
}