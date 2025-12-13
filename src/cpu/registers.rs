pub struct Registers {
    pub pc: u16,

    pub sp: u16,

    pub a: u8,

    pub f: Flags,

    pub b: u8,

    pub c: u8,

    pub d: u8,

    pub e: u8,

    pub h: u8,

    pub l: u8,
    // pub ie: u16,

    // pub ir: u16,
}

pub struct Flags {
    pub z: bool, // Zero

    pub h: bool, // Half Carry Flag

    pub n: bool, // Subtract Flag

    pub c: bool, // Carry Flag
}

impl Flags {
    pub fn new() -> Self {
        Self {
            z: false,
            n: false,
            h: false,
            c: false,
        }
    }

    pub fn set_flag(&mut self, flag: char, value: bool) {
        match flag {
            'Z' | 'z' => self.z = value,
            'N' | 'n' => self.n = value,
            'H' | 'h' => self.h = value,
            'C' | 'c' => self.c = value,
            _ => panic!("Invalid flag {}", flag),
        }
    }

    pub fn get_flag(&self, flag: char) -> bool {
        match flag {
            'Z' | 'z' => self.z,
            'N' | 'n' => self.n,
            'H' | 'h' => self.h,
            'C' | 'c' => self.c,
            _ => panic!("Invalid flag {}", flag),
        }
    }
    pub fn to_u8(&self) -> u8 {
        ((self.z as u8) << 7)
            | ((self.n as u8) << 6)
            | ((self.h as u8) << 5)
            | ((self.c as u8) << 4)
    }

    pub fn from_u8(f: u8) -> Self {
        Flags {
            z: f & 0x80 != 0,
            n: f & 0x40 != 0,
            h: f & 0x20 != 0,
            c: f & 0x10 != 0,
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
   // F,
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
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            //Reg8::F => self.f.to_u8(),
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    pub fn set8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::A => self.a = val,
            Reg8::B => self.b = val,
            Reg8::C => self.c = val,
            Reg8::D => self.d = val,
            Reg8::E => self.e = val,
            //Reg8::F => self.f = Flags::from_u8(val),
            Reg8::H => self.h = val,
            Reg8::L => self.l = val,
        }
    }

    pub fn set16(&mut self, reg: Reg16, val: u16) {
        match reg {
            Reg16::AF => {
                self.a = (val >> 8) as u8;
                self.f = Flags::from_u8((val & 0xF0) as u8);
            }

            Reg16::BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0xFF) as u8;
            }

            Reg16::DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0xFF) as u8;
            }

            Reg16::HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0xFF) as u8;
            }
            Reg16::SP => self.sp = val,
            Reg16::PC => self.pc = val,
        }
    }

    pub fn get16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => ((self.a as u16) << 8) | (self.f.to_u8() as u16),
            Reg16::BC => ((self.b as u16) << 8) | (self.c as u16),
            Reg16::DE => ((self.d as u16) << 8) | (self.e as u16),
            Reg16::HL => ((self.h as u16) << 8) | (self.l as u16),
            Reg16::SP => self.sp,
            Reg16::PC => self.pc,
        }
    }
}
