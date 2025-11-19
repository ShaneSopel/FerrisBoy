use crate::cpu::Reg8;
use crate::cpu::Reg16;

#[derive(Debug, Clone, Copy)]
pub enum ByteSel 
{
    High,
    Low,
}

pub enum MicroOp
{
    Nop, 
    LdReg8FromReg8   { dst: Reg8, src: Reg8 },
    LdReg8FromMem    { dst: Reg8, src: Reg16 }, 
    LdReg16FromMem   { dst: Reg16, src: Reg16 },
    LdReg16FromReg8  { dst: Reg16, src_hi: Reg8, src_lo: Reg8 },
    LdReg8FromReg16  { dst: Reg8,  src: Reg16, byte: ByteSel },
    IncReg8          { reg: Reg8 },
    DecReg8          { reg: Reg8 },
    IncReg16         { reg: Reg16 }, 
    DecReg16         { reg: Reg16 },
    AddReg8          { dst: Reg8, src: Reg8 },
    AddReg16         { dst: Reg16, src: Reg16 },
    AddCarry8        { dst: Reg8, src: Reg8 },
    AddCarry16       { dst: Reg8,  src: Reg16 },
    SubReg8          { dst: Reg8,  src: Reg8 },
    SubReg16         { dst: Reg8,  src: Reg16 },
    SubCarry8        { dst: Reg8,  src: Reg8 },
    SubCarry16       { dst: Reg8,  src: Reg16 },  
    PushReg16        { reg: Reg16 },
    PopReg16         { reg: Reg16 },
    Fetch8,
    Fetch16,
    JumpAbsolute    { addr: Reg16 },
    JumpRelative    { offset: i8 },
    CallAbsolute    { addr: Reg16 },
    Return,
    Restart         { vector: u16 },
    Illegal         { opcode: u8 },
}