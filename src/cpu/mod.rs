use crate::interconnect::Interconnect;

use crate::cpu::instructions::process_instruction;

mod instructions;

//mod cpu
//{
    use sdl2::{libc::regex_t, sys::valloc};

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

    F: u8,

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
//impl<'a> Cpu <'a>
{
    pub fn new(inter: Interconnect) -> Cpu
    //pub fn new<'n>(inter: Interconnect<'n>) -> Cpu<'n> 
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

   // pub fn next_byte(&mut self) -> u8
   // {

   //     let b = self.inter.read_byte(self.regs.PC);
   //     self.regs.PC = self.regs.PC.wrapping_add(1);
   //     b
   // }

    fn fetch_byte(&mut self, val: u16) -> u8
    {
        let b = self.inter.read_byte(val);

        self.delay(1);

        b
    }

    fn store_byte(&mut self, addr: u16, val:u8)
    {
        self.inter.write_byte(addr, val);

        self.delay(1);
    }

    // pop byte returns 8bit
    // the stack decrements 
    fn pop_byte(&mut self) -> u8
    {
        let sp = self.get_register_16("SP");

        let b = self.fetch_byte(sp);

        self.set_register_16(sp+1, "SP");

        b

    }

    fn push_byte(&mut self, val: u8)
    {
        let mut sp = self.get_register_16("SP");

        sp -= 1;

        self.set_register_16(sp, "SP");

        self.store_byte(sp, val);
    }

    //push two bytes onto the stack and then decrenment the stack pointer
    // it will decrement twice since its a word and is 16 bits. 
    // this is basically utilizing the push byte function twice for a 16 bit operation. 
    fn push_word(&mut self, val: u16)
    {
        self.push_byte((val >> 8) as u8);
        self.push_byte(val as u8);

    }

    // incerment the stack as is the oppposite of the push method. 
    fn pop_word(&mut self, ) -> u16
    {
        let lo = self.pop_byte() as u16;
        let hi = self.pop_byte() as u16;

        (hi << 8) | lo
    }


    /*fn interrupt(&mut self, it: Interrupt)
    {
        self.halted = false;

        self.disable_interrupts();

        let handler_addr = match it
        {
            Interrupt::VBlank => 0x40,
            Interrupt::Lcdc => 0x48,
            Interrupt::Timer => 0x50,
        };

        let pc = self.get_register_16("PC");
        //self.push_word(pc);

        //self.delay(6);

        self.set_register_16(handler_addr, "PC");

    }*/

    pub fn run_next_instruction(&mut self) -> u8
    {
        self.instruction_cycles
    }

    fn advance(&mut self, cycles: u8 )
    {
        for _ in 0..cycles
        {
     // To DO: I need to figure out how the cart works before I Can implement
     // what the interconnect and the cpu do. 
     //       self.inter.step();
        }

        self.instruction_cycles += cycles;

    }

    //there is one machine cycle for ever 4 clock cycles
    fn delay(&mut self, machine_cycles: u8)
    {
        self.advance(machine_cycles * 4);
    }

    // set 16 bit dual registers 
    fn set_8_to_16_conversion(&mut self, reg: &str, val: u16)
    {
        if reg == "AF"
        {
            self.regs.A = (val >> 8) as u8;        
            self.regs.F = (val & 0xF0) as u8;      
        }
        else if reg == "BC"
        {
            self.regs.B = (val >> 8) as u8;        
            self.regs.C = (val & 0xFF) as u8;      
        }
        else if reg == "DE"
        {
            self.regs.D = (val >> 8) as u8;       
            self.regs.E = (val & 0xFF) as u8;     
        }
        else if reg == "HL"
        {
            self.regs.H = (val >> 8) as u8;        
            self.regs.L = (val & 0xFF) as u8;    
        }
        else
        {
            println!("invalid entry");
        }
    }

    // get the 16 bit registers values
    fn get_8_to_16_conversion(&mut self, reg: &str) -> u16
    {
        if reg == "AF"
        {
            let mut v = self.regs.F as u16;        
            v |= (self.regs.A as u16) << 8;          
            return v & 0xFFF0;                        
        }
        else if reg == "BC"
        {
            let mut v = self.regs.C as u16;          
            v |= (self.regs.B as u16) << 8;          
            return v;
        }
        else if reg == "DE"
        {
            let mut v = self.regs.E as u16;          
            v |= (self.regs.D as u16) << 8;          
            return v;
        }
        else if reg == "HL"
        {
            let mut v = self.regs.L as u16;          
            v |= (self.regs.H as u16) << 8;          
            return v;
        }
        else  
        {
            println!("invalid entry");
            return 0;  
        }
    }

    // set 16 bit register
    fn set_register_16(&mut self, val: u16, reg: &str)
    {
        match reg
        {
            "PC" => self.regs.PC = val,
            "SP" => self.regs.SP = val,
            "AF" => self.set_8_to_16_conversion("AF", val),
            "BC" => self.set_8_to_16_conversion("BC", val),
            "DE" => self.set_8_to_16_conversion("DE", val),
            "HL" => self.set_8_to_16_conversion("HL", val),

            _ => println!("invalid value"),
        }
    }

    // get 16 bit register
    fn get_register_16(&mut self, reg: & str) -> u16
    {
        match reg
        {
            "PC" => self.regs.PC,
            "SP" => self.regs.SP,
            "AF" => self.get_8_to_16_conversion(reg),
            "BC" => self.get_8_to_16_conversion(reg),
            "DE" => self.get_8_to_16_conversion(reg),
            "HL" => self.get_8_to_16_conversion(reg),

            _ => 0,
        }
    }


    // set 8 bit register
    fn set_register_8(&mut self, val: u8, reg: char)
    {
        match reg
        {
            'A' => self.regs.A = val,
            'B' => self.regs.B = val,
            'C' => self.regs.C = val,
            'D' => self.regs.D = val,
            'E' => self.regs.E = val,
            'H' => self.regs.H = val,
            'L' => self.regs.L = val, 

            _ => println!("invalid value"),

        }

    }

    //get 8 bit register
    fn get_register_8(& mut self, reg: char) -> u8
    {
        match reg
        {
            'A' => self.regs.A,
            'B' => self.regs.B,
            'C' => self.regs.C,
            'D' => self.regs.D,
            'E' => self.regs.E,
            'H' => self.regs.H,
            'L' => self.regs.L,

            _ => 0,

        }
    }


    // set up F register functions  since it needs to be broken up for each flag
    fn get_F_reg(& mut self, val: u8) -> u8
    {
        let z = self.flags.Z as u8;
        let n = self.flags.N as u8;
        let h = self.flags.H as u8;
        let c =self.flags.C as u8;

        // only the top 7 - 4 bits of the 8 bit register matter.
        // z is bit 7, n is bit 6, h is bit 4, c is bit 4
        (z << 7) | (n << 6) | (h << 5) | (c << 4)
    }

    fn set_F_reg(&mut self, val: u8)
    {
        self.flags.Z = (val &(1 << 7)) != 0;
        self.flags.N = (val &(1 << 6)) != 0;
        self.flags.H = (val &(1 << 5)) != 0;
        self.flags.C = (val &(1 << 4)) != 0;
    }

    fn set_flags(&mut self, flag: char, val: bool)
    {
        match flag
        {
            'H' => self.flags.H = val,
            'N' => self.flags.N = val,
            'Z' => self.flags.Z = val,
            'C' => self.flags.C = val,

            _ => println!("invald input"),
        }

    }

    fn get_flags(&mut self, flag: char) -> bool
    {
        match flag
        {
            'H' => self.flags.H,
            'N' => self.flags.N,
            'Z' => self.flags.Z,
            'C' => self.flags.C,

            _ => false,
        }

    }

    fn disable_interrupts(&mut self)
    {
        self.interrupt = false;
        self.interrupt_enable_next = false
    }

    fn enable_interrupts(&mut self)
    {
        self.interrupt = true;
        self.interrupt_enable_next = true;
    }

    fn enable_interrupts_next(&mut self)
    {
        self.interrupt_enable_next = true;
    }
    
    fn set_halt(&mut self, val: bool)
    {
        self.halted = val;
    }

    fn stop(&mut self)
    {
        panic!("STOP")
    }


}

#[test]
fn test_register_pair_consistency() 
{
    let memory = vec![0; 0x10000];
    let inter = Interconnect::new(memory);
    let mut cpu = Cpu::new(inter);

    cpu.set_8_to_16_conversion("HL", 0x1234);
    assert_eq!(cpu.get_8_to_16_conversion("HL"), 0x1234);

    cpu.set_8_to_16_conversion("DE", 0xABCD);
    assert_eq!(cpu.get_8_to_16_conversion("DE"), 0xABCD);
}
