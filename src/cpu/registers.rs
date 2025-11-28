pub struct Registers {
    pub PC: u16,

    pub SP: u16,

    pub A: u8,

    pub F: Flags,

    pub B: u8,

    pub C: u8,

    pub D: u8,

    pub E: u8,

    pub H: u8,

    pub L: u8,

    pub IE: u16,

    pub IR: u16,
}

pub struct Flags {
    pub Z: bool, // Zero

    pub H: bool, // Half Carry Flag

    pub N: bool, // Subtract Flag

    pub C: bool, // Carry Flag
}

impl Flags {
    pub fn new() -> Self {
        Self {
            Z: false,
            N: false,
            H: false,
            C: false,
        }
    }

    pub fn set_flag(&mut self, flag: char, value: bool) {
        match flag {
            'Z' | 'z' => self.Z = value,
            'N' | 'n' => self.N = value,
            'H' | 'h' => self.H = value,
            'C' | 'c' => self.C = value,
            _ => panic!("Invalid flag {}", flag),
        }
    }

    pub fn get_flag(&self, flag: char) -> bool {
        match flag {
            'Z' | 'z' => self.Z,
            'N' | 'n' => self.N,
            'H' | 'h' => self.H,
            'C' | 'c' => self.C,
            _ => panic!("Invalid flag {}", flag),
        }
    }
    pub fn to_u8(&self) -> u8 {
        ((self.Z as u8) << 7)
            | ((self.N as u8) << 6)
            | ((self.H as u8) << 5)
            | ((self.C as u8) << 4)
    }

    pub fn from_u8(f: u8) -> Self {
        Flags {
            Z: f & 0x80 != 0,
            N: f & 0x40 != 0,
            H: f & 0x20 != 0,
            C: f & 0x10 != 0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

impl Registers {
    pub fn get8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.A,
            Reg8::B => self.B,
            Reg8::C => self.C,
            Reg8::D => self.D,
            Reg8::E => self.E,
            Reg8::F => self.F.to_u8(),
            Reg8::H => self.H,
            Reg8::L => self.L,
        }
    }

    pub fn set8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.A = val,
            Reg8::B => self.B = val,
            Reg8::C => self.C = val,
            Reg8::D => self.D = val,
            Reg8::E => self.E = val,
            Reg8::F => self.F = Flags::from_u8(val),
            Reg8::H => self.H = val,
            Reg8::L => self.L = val,
        }
    }

    pub fn set16(&mut self, reg: Reg16, val: u16) {
        match reg {
            Reg16::AF => {
                self.A = (val >> 8) as u8;
                self.F = Flags::from_u8((val & 0xF0) as u8);
            }

            Reg16::BC => {
                self.B = (val >> 8) as u8;
                self.C = (val & 0xFF) as u8;
            }

            Reg16::DE => {
                self.D = (val >> 8) as u8;
                self.E = (val & 0xFF) as u8;
            }

            Reg16::HL => {
                self.H = (val >> 8) as u8;
                self.L = (val & 0xFF) as u8;
            }
            Reg16::SP => self.SP = val,
            Reg16::PC => self.PC = val,
        }
    }

    pub fn get16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => ((self.A as u16) << 8) | (self.F.to_u8() as u16),
            Reg16::BC => ((self.B as u16) << 8) | (self.C as u16),
            Reg16::DE => ((self.D as u16) << 8) | (self.E as u16),
            Reg16::HL => ((self.H as u16) << 8) | (self.L as u16),
            Reg16::SP => self.SP,
            Reg16::PC => self.PC,
        }
    }
}
