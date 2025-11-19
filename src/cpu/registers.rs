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

pub enum Reg8
{
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Reg16
{
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

impl Registers
{
    pub fn get8(&self, reg: Reg8)
    {
        match reg
        {
            Reg8::A => self.A,
            Reg8::B => self.B,
            Reg8::C => self.C,
            Reg8::D => self.D,
            Reg8::E => self.E,
            Reg8::F => self.F,
            Reg8::H => self.H,
            Reg8::L => self.L,
        }
    }

    pub fn set8(&self, reg: Reg8, val: u8)
    {
        match reg
        {
            Reg8::A => self.A = val,
            Reg8::B => self.B = val,
            Reg8::C => self.C = val,
            Reg8::D => self.D = val,
            Reg8::E => self.E = val,
            Reg8::F => self.F = val & 0xF0,
            Reg8::H => self.H = val,
            Reg8::L => self.L = val,
        }

    }

    pub fn set16(&self, reg: Reg16, val: u16)
    {
            match reg 
        {
            Reg16::AF => 
            {
                self.A = (val >> 8) as u8;
                self.F = (val & 0xF0) as u8;
            }

            Reg16::BC=> 
            {
                self.B = (val >> 8) as u8;
                self.C = (val & 0xFF) as u8;
            }

            Reg16::DE => 
            {
                self.D = (val >> 8) as u8;
                self.E = (val & 0xFF) as u8;
            }

            Reg16::HL => 
            {
                self.H = (val >> 8) as u8;
                self.L = (val & 0xFF) as u8;
            }
            Reg16::SP => self.SP = val,
            Reg16::PC => self.PC = val,
            _ => println!("invalid entry"),
        }
    }


    fn get16(&self, reg: Reg16, val: u16)
    {
        match reg 
        {

            Reg16::AF => ((self.A as u16) << 8) | (self.F as u16 & 0xF0),
            Reg16::BC => ((self.B as u16) << 8) | (self.C as u16),
            Reg16::DE => ((self.D as u16) << 8) | (self.E as u16),
            Reg16::HL => ((self.H as u16) << 8) | (self.L as u16),
            "SP" => self.SP,
            "PC" => self.PC,
            _ => { println!("invalid entry"); 0 }
        }   

    }

}