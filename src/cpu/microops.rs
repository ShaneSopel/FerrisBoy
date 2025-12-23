use crate::cpu::Reg16;
use crate::cpu::Reg8;

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
    LdReg8FromImm {
        dst: Reg8,
    },
    LdReg8FromMemIncHL {
        dst: Reg8,
    },

    LdMemFromReg8IncHL {
        src: Reg8,
    },
    LdMemFromReg8DecHL {
        src: Reg8,
    },
    LdReg8FromMemDecHL {
        dst: Reg8,
    },
    LdMemFromReg8 {
        addr: Reg16,
        src: Reg8,
    },
    LdA8FromA {
        offset: u8,
    },
    LdAFromA8 {
        offset: u8,
    },
    LdCFromA,
    LdAFromC,
    LdMemFromA {
        addr: u16,
    },
    LdReg16FromMem {
        dst: Reg16,
        src: Reg16,
    },
    LdMemImm16FromReg16 {
        src: Reg16,
    },
    LdReg8FromReg16 {
        dst: Reg8,
        src: Reg16,
    },
    LdMemFromImm8 {
        addr: Reg16,
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
        addr: u8,
    },
    AddReg16 {
        dst: Reg16,
        src: Reg16,
    }, //REG16::HL
    AddCarry8 {
        dst: Reg8,
        src: Reg8,
    },
    AddCarry8Mem {
        dst: Reg8,
        src: Reg16,
    },
    AddCarry8Imm {
        dst: Reg8,
        addr: u8,
    },
    SubReg8 {
        dst: Reg8,
        src: Reg8,
    },
    // SubReg8Mem {
    //     dst: Reg8,
    //     src: Reg16,
    // },
    SubReg8Imm {
        dst: Reg8,
        addr: u8,
    },

    SubCarry8 {
        dst: Reg8,
        src: Reg8,
    },

    SubCarry8Mem {
        dst: Reg8,
        src: Reg16,
    },
    SubCarry8Imm {
        dst: Reg8,
        addr: u8,
    },

    XorReg8 {
        dst: Reg8,
        src: Reg8,
    },
    XorReg8Mem {
        dst: Reg8,
        src: Reg16,
    },
    XorReg8Imm {
        dst: Reg8,
        addr: u8,
    },
    CpReg8 {
        dst: Reg8,
        src: Reg8,
    },
    CpReg8Mem {
        dst: Reg8,
        src: Reg16,
    },
    CpReg8Imm {
        dst: Reg8,
        addr: u8,
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
        addr: u8,
    },
    AndReg8 {
        dst: Reg8,
        src: Reg8,
    },
    AndReg8Mem {
        dst: Reg8,
        src: Reg16,
    },
    AndReg8Imm {
        dst: Reg8,
        addr: u8,
    },
    PushReg16 {
        reg: Reg16,
    },
    PopReg16 {
        reg: Reg16,
    },
    JumpAbsolute {
        addr: u16,
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
    JumpHL,
    CallAbsolute {
        addr: u16,
    },
    CallAbsoluteIf {
        addr: u16,
        flag: char,
        expected: bool,
    },
    Return,
    ReturnIf {
        flag: char,
        expected: bool,
    },
    Reti,
    Restart {
        vector: u16,
    },
    Rlca,
    Rrca,
    Rla,
    Rra,
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
    RlRegHl,
    RlcRegHl,
    RrRegHl,
    RrcRegHl,
    SlaRegHl,
    SraRegHl,
    SrlRegHl,
    SwapRegHl,
    AddImmToSP {
        imm: i8,
    },
    BitReg8 {
        bit: u8,
        reg: Reg8,
    },
    BitRegHl {
        bit: u8,
    },

    ResReg8 {
        bit: u8,
        reg: Reg8,
    },
    ResRegHl {
        bit: u8,
    },

    SetReg8 {
        bit: u8,
        reg: Reg8,
    },
    SetRegHl {
        bit: u8,
    },
    LdHLSPPlusR8,
    // Illegal {
    //     opcode: u8,
    // },
}
