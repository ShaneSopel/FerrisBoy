use crate::cpu::Reg16;
use crate::cpu::Reg8;

#[derive(Debug, Clone, Copy)]
pub enum ByteSel {
    High,
    Low,
}

pub enum MicroOp {
    Nop,
    Halt,
    Stop,
    LdReg8FromReg8 {
        dst: Reg8,
        src: Reg8,
    },
    LdReg8FromMem {
        dst: Reg8,
        src: Reg16,
    },
    LdMemFromReg8 {
        addr: Reg16,
        src: Reg8,
    },
    LdReg16FromMem {
        dst: Reg16,
        src: Reg16,
    },
    LdReg8FromReg16 {
        dst: Reg8,
        src: Reg16,
    },
    IncReg8 {
        reg: Reg8,
    },
    DecReg8 {
        reg: Reg8,
    },
    IncReg16 {
        reg: Reg16,
    },
    DecReg16 {
        reg: Reg16,
    },
    AddReg8 {
        dst: Reg8,
        src: Reg8,
    },
    AddReg8Mem {
        dst: Reg8,
        src: Reg16,
    },
    AddReg8Imm {
        dst: Reg8,
        src: u8,
    },
    AddReg16 {
        dst: Reg16,
        src: Reg16,
    }, //REG16::HL
    AddCarry8 {
        dst: Reg8,
        src: Reg8,
    },
    SubReg8 {
        dst: Reg8,
        src: Reg8,
    },
    SubCarry8 {
        dst: Reg8,
        src: Reg8,
    },
    XorReg8 {
        dst: Reg8,
        src: Reg8,
    },
    CpReg8 {
        a: Reg8,
        src: Reg8,
    },
    CpReg8Mem {
        a: Reg8,
        src: Reg16,
    },
    OrReg8 {
        dst: Reg8,
        src: Reg8,
    },
    OrReg8Mem {
        dst: Reg8,
        src: Reg16,
    },
    OrReg8Imm {
        dst: Reg8,
        src: u8,
    },
    AndReg8 {
        dst: Reg8,
        src: Reg8,
    },
    PushReg16 {
        reg: Reg16,
    },
    PopReg16 {
        reg: Reg16,
    },
    JumpAbsolute {
        addr: Reg16,
    },
    JumpAbsoluteIf {
        addr: u16,
        flag: char,
        expected: bool,
    },
    JumpRelative {
        offset: i8,
    },
    JumpRelativeIf {
        offset: i8,
        flag: char,
        expected: bool,
    },
    CallAbsolute {
        addr: u16,
    },
    CallAbsoluteIf {
        addr: u16,
        flag: char,
        expected: bool,
    },
    Return {},
    ReturnIf {
        flag: char,
        expected: bool,
    },
    Restart {
        vector: u16,
    },
    Di,
    Ei,
    Cpl,
    Ccf,
    Scf,
    Daa,
    RlReg8 {
        dst: Reg8,
    },
    RlcReg8 {
        dst: Reg8,
    },
    RrReg8 {
        dst: Reg8,
    },
    RrcReg8 {
        dst: Reg8,
    },
    SlaReg8 {
        dst: Reg8,
    },
    SraReg8 {
        dst: Reg8,
    },
    SrlReg8 {
        dst: Reg8,
    },
    SwapReg8 {
        dst: Reg8,
    },
    AddImmToSP {
        imm: i8,
    },
    LdHLSPPlusR8 {
        r8: i8,
    },
    Illegal {
        opcode: u8,
    },
}
