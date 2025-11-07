use crate::cpu::{self, Cpu};

use std::{f32::consts::E, fmt::Debug, rc};

#[derive(Debug)]
pub struct Opcode 
{
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub immediate: bool,
    pub execute: fn(&mut Cpu) -> u8,

}

//256
pub const OPCODES: [Opcode; 256] = 
[
    Opcode { mnemonic: "NOP", bytes: 1,    immediate: true, execute: nop}, // 0x00
    Opcode { mnemonic: "LD", bytes: 3,     immediate: true, execute: ld_bc_d16}, // 0x01
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_bc_a}, // 0x02
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_bc }, // 0x03
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_b }, // 0x04
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_b}, // 0x05
    Opcode { mnemonic: "LD", bytes: 2,     immediate: true, execute: ld_b_d8 }, // 0x06
    Opcode { mnemonic: "RLCA", bytes: 1,   immediate: true, execute: rcla}, // 0x07
    Opcode { mnemonic: "LD", bytes: 3,     immediate: false, execute: ld_a16_sp }, // 0x08
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_hl_bc }, // 0x09
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute:  ld_a_bc }, // 0x0A
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_bc }, // 0x0B
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_c }, // 0x0C
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_c }, // 0x0D
    Opcode {mnemonic: "LD", bytes: 2,      immediate: true, execute: ld_c_d8}, // 0x0E
    Opcode { mnemonic: "RRCA", bytes: 1,   immediate: true, execute: rrca }, // 0x0F
    Opcode { mnemonic: "STOP", bytes: 2,   immediate: true, execute: stop }, // 0x10
    Opcode { mnemonic: "LD", bytes: 3,     immediate: true, execute: ld_de_d16 }, // 0x11
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_de_a }, // 0x12
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_de }, // 0x13
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_d}, // 0x14
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_d }, // 0x15
    Opcode { mnemonic: "LD", bytes: 2,     immediate: true, execute: ld_d_d8 }, // 0x16
    Opcode { mnemonic: "RLA", bytes: 1,    immediate: true, execute: rla }, // 0x17
    Opcode { mnemonic: "JR", bytes: 2,     immediate: true, execute: jr_r8 }, // 0x18
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_hl_de }, // 0x19
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_a_de }, // 0x1A
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_de }, // 0x1B
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_e }, // 0x1C
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_e }, // 0x1D
    Opcode { mnemonic: "LD", bytes: 2,     immediate: true, execute: ld_e_d8 }, // 0x1E
    Opcode { mnemonic: "RRA", bytes: 1,    immediate: true, execute: rra }, // 0x1F
    Opcode { mnemonic: "JR", bytes: 2,     immediate: true, execute: jr_nz_r8 }, // 0x20
    Opcode { mnemonic: "LD", bytes: 3,     immediate: true, execute: ld_hl_d16 }, // 0x21
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_inc_a }, // 0x22
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_hl }, // 0x23
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_h }, // 0x24
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_h }, // 0x25
    Opcode { mnemonic: "LD", bytes: 2,     immediate: true, execute: ld_h_d8 }, // 0x26
    Opcode { mnemonic: "DAA", bytes: 1,    immediate: true, execute: daa }, // 0x27
    Opcode { mnemonic: "JR", bytes: 2,     immediate: true, execute: jr_z_r8 }, // 0x28
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_hl_hl }, // 0x29
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_a_hl_inc}, // 0x2A
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_hl }, // 0x2B
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_l }, // 0x2C
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_l }, // 0x2D
    Opcode { mnemonic: "LD", bytes: 2,     immediate: true, execute: ld_l_d8 }, // 0x2E
    Opcode { mnemonic: "CPL", bytes: 1,    immediate: true, execute: cpl }, // 0x2F
    Opcode { mnemonic: "JR", bytes: 2,     immediate: true, execute: jr_nc_r8 }, // 0x30
    Opcode { mnemonic: "LD", bytes: 3,     immediate: true, execute: ld_sp_d16 }, // 0x31
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_dec_a }, // 0x32
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_sp }, // 0x33
    Opcode { mnemonic: "INC", bytes: 1,    immediate: false, execute: inc_hl_mem }, // 0x34
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: false, execute: dec_hl_mem }, // 0x35
    Opcode { mnemonic: "LD", bytes: 2,     immediate: false, execute: undefined }, // 0x36
    Opcode { mnemonic: "SCF", bytes: 1,    immediate: true, execute: undefined }, // 0x37
    Opcode { mnemonic: "JR", bytes: 2,     immediate: true, execute: undefined }, // 0x38
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x39
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x3A
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: undefined }, // 0x3B
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: undefined }, // 0x3C
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: undefined }, // 0x3D
    Opcode { mnemonic: "LD", bytes: 2,     immediate: true, execute: undefined }, // 0x3E
    Opcode { mnemonic: "CCF", bytes: 1,    immediate: true, execute: undefined }, // 0x3F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x40
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x41
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x42
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x43
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x44
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x45
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x46
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x47
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x48
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x49
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x4A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x4B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x4C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x4D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x4E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined },  // 0x4F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined },// 0x50
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x51
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x52
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x53
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x54
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x55
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x56
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x57
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x58
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x59
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x5A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x5B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x5C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x5D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x5E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x5F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x60
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x61
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x62
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x63
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x64
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x65
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x66
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x67
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x68
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x69
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x6A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x6B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x6C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x6D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x6E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x6F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x70
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x71
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x72
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x73
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x74
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x75
    Opcode { mnemonic: "HALT", bytes: 1,   immediate: true, execute: undefined }, // 0x76
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x77
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x78
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x79
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x7A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x7B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x7C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x7D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: undefined }, // 0x7E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: undefined }, // 0x7F
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x80
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x81
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x82
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x83
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x84
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x85
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: false, execute: undefined }, // 0x86
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: undefined }, // 0x87
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: undefined }, // 0x88
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: undefined }, // 0x89
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: undefined }, // 0x8A
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: undefined }, // 0x8B
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: undefined }, // 0x8C
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: undefined }, // 0x8D
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: false, execute: undefined }, // 0x8E
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: undefined }, // 0x8F
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: undefined }, // 0x90
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: undefined }, // 0x91
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: undefined }, // 0x92
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: undefined }, // 0x93
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: undefined }, // 0x94
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: undefined }, // 0x95
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: false, execute: undefined }, // 0x96
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: undefined }, // 0x97
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: undefined }, // 0x98
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: undefined }, // 0x99
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: undefined }, // 0x9A
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: undefined }, // 0x9B
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: undefined }, // 0x9C
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: undefined }, // 0x9D
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: false, execute: undefined }, // 0x9E
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: undefined }, // 0x9F
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: undefined }, // 0xA0
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: undefined }, // 0xA1
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: undefined }, // 0xA2
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: undefined }, // 0xA3
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: undefined }, // 0xA4
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: undefined }, // 0xA5
    Opcode { mnemonic: "AND", bytes: 1,    immediate: false, execute: undefined}, // 0xA6
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: undefined }, // 0xA7
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: undefined }, // 0xA8
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: undefined }, // 0xA9
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: undefined }, // 0xAA
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: undefined }, // 0xAB
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: undefined }, // 0xAC
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: undefined }, // 0xAD
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: false, execute: undefined }, // 0xAE
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: in_xor_a}, // 0xAF
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: undefined }, // 0xB0
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: undefined }, // 0xB1
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: undefined }, // 0xB2
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: undefined }, // 0xB3
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: undefined }, // 0xB4
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: undefined }, // 0xB5
    Opcode { mnemonic: "OR", bytes: 1,     immediate: false, execute: undefined }, // 0xB6
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: undefined }, // 0xB7
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: undefined }, // 0xB8
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: undefined }, // 0xB9
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: undefined }, // 0xBA
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: undefined }, // 0xBB
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: undefined }, // 0xBC
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: undefined }, // 0xBD
    Opcode { mnemonic: "CP", bytes: 1,     immediate: false, execute: undefined }, // 0xBE
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: undefined }, // 0xBF
    Opcode { mnemonic: "RET", bytes: 1,    immediate: true, execute: undefined }, // 0xC0
    Opcode { mnemonic: "POP", bytes: 1,    immediate: true, execute: undefined }, // 0xC1
    Opcode { mnemonic: "JP", bytes: 3,     immediate: true, execute: undefined }, // 0xC2
    Opcode { mnemonic: "JP", bytes: 3,     immediate: true, execute: jp_a16}, // 0xC3
    Opcode { mnemonic: "CALL", bytes: 3,   immediate: true, execute: undefined }, // 0xC4
    Opcode { mnemonic: "PUSH", bytes: 1,   immediate: true, execute: undefined }, // 0xC5
    Opcode { mnemonic: "ADD", bytes: 2,    immediate: true, execute: undefined }, // 0xC6
    Opcode { mnemonic: "RST", bytes: 1,    immediate: true, execute: undefined }, // 0xC7
    Opcode { mnemonic: "RET", bytes: 1,    immediate: true, execute: undefined  }, // 0xC8
    Opcode { mnemonic: "RET", bytes: 1,    immediate: true, execute: undefined  }, // 0xC9
    Opcode { mnemonic: "JP", bytes: 3,     immediate: true, execute: undefined  }, // 0xCA
    Opcode { mnemonic: "PREFIX", bytes: 1, immediate: true, execute: undefined  }, // 0xCB
    Opcode { mnemonic: "CALL", bytes: 3,   immediate: true, execute: undefined  }, // 0xCC
    Opcode { mnemonic: "CALL", bytes: 3,   immediate: true, execute: undefined  }, // 0xCD
    Opcode { mnemonic: "ADC", bytes: 2,    immediate: true, execute: undefined  }, // 0xCE
    Opcode { mnemonic: "RST", bytes: 1,    immediate: true, execute: undefined  }, // 0xCF
    Opcode { mnemonic: "RET", bytes: 1,    immediate: true, execute: undefined  }, // 0xD0
    Opcode { mnemonic: "POP", bytes: 1,    immediate: true , execute: undefined }, // 0xD1
    Opcode { mnemonic: "JP", bytes: 3,     immediate: true, execute: undefined  }, // 0xD2
    Opcode { mnemonic: "ILLEGAL_D3", bytes: 1,  immediate: true, execute: undefined  }, // 0xD3
    Opcode { mnemonic: "CALL", bytes: 3,  immediate: true , execute: undefined }, // 0xD4
    Opcode { mnemonic: "PUSH", bytes: 1, immediate: true, execute: undefined  }, // 0xD5
    Opcode { mnemonic: "SUB", bytes: 2, immediate: true, execute: undefined  }, // 0xD6
    Opcode { mnemonic: "RST", bytes: 1,  immediate: true , execute: undefined }, // 0xD7
    Opcode { mnemonic: "RET", bytes: 1,  immediate: true, execute: undefined  }, // 0xD8
    Opcode { mnemonic: "RETI", bytes: 1, immediate: true, execute: undefined  }, // 0xD9
    Opcode { mnemonic: "JP", bytes: 3, immediate: true , execute: undefined }, // 0xDA
    Opcode { mnemonic: "ILLEGAL_DB", bytes: 1, immediate: true, execute: undefined  }, // 0xDB
    Opcode { mnemonic: "CALL", bytes: 3, immediate: true, execute: undefined }, // 0xDC
    Opcode { mnemonic: "ILLEGAL_DD", bytes: 1,  immediate: true, execute: undefined }, // 0xDD
    Opcode { mnemonic: "SBC", bytes: 2, immediate: true, execute: undefined }, // 0xDE
    Opcode { mnemonic: "RST", bytes: 1, immediate: true, execute: undefined }, // 0xDF
    Opcode { mnemonic: "LDH", bytes: 2, immediate: false, execute: undefined }, // 0xE0
    Opcode { mnemonic: "POP", bytes: 1, immediate: true, execute: undefined }, // 0xE1
    Opcode { mnemonic: "LDH", bytes: 1, immediate: false, execute: undefined }, // 0xE2
    Opcode { mnemonic: "ILLEGAL_E3", bytes: 1, immediate: true, execute: undefined }, // 0xE3
    Opcode { mnemonic: "ILLEGAL_E4", bytes: 1,  immediate: true, execute: undefined }, // 0xE4
    Opcode { mnemonic: "PUSH", bytes: 1, immediate: true, execute: undefined  }, // 0xE5
    Opcode { mnemonic: "AND", bytes: 2,  immediate: true, execute: undefined }, // 0xE6
    Opcode { mnemonic: "RST", bytes: 1,  immediate: true, execute: undefined }, // 0xE7
    Opcode { mnemonic: "ADD", bytes: 2,  immediate: true, execute: undefined }, // 0xE8
    Opcode { mnemonic: "JP", bytes: 1,   immediate: true, execute: undefined }, // 0xE9
    Opcode { mnemonic: "LD", bytes: 3,   immediate: false, execute: undefined }, // 0xEA
    Opcode { mnemonic: "ILLEGAL_EB", bytes: 1, immediate: true, execute: undefined }, // 0xEB
    Opcode { mnemonic: "ILLEGAL_EC", bytes: 1, immediate: true, execute: undefined }, // 0xEC
    Opcode { mnemonic: "ILLEGAL_ED", bytes: 1, immediate: true, execute: undefined }, // 0xED
    Opcode { mnemonic: "XOR", bytes: 2, immediate: true, execute: undefined}, // 0xEE
    Opcode { mnemonic: "RST", bytes: 1, immediate: true, execute: undefined }, // 0xEF
    Opcode { mnemonic: "LDH", bytes: 2, immediate: false, execute: undefined }, // 0xF0
    Opcode { mnemonic: "POP", bytes: 1, immediate: true, execute: undefined }, // 0xF1
    Opcode { mnemonic: "LDH", bytes: 1,immediate: false, execute: undefined }, // 0xF2
    Opcode { mnemonic: "DI", bytes: 1,  immediate: true, execute: in_di}, // 0xF3
    Opcode { mnemonic: "ILLEGAL_F4", bytes: 1,  immediate: true, execute:undefined}, // 0xF4
    Opcode { mnemonic: "PUSH", bytes: 1, immediate: true, execute: undefined }, // 0xF5
    Opcode { mnemonic: "OR", bytes: 2,immediate: true, execute: undefined }, // 0xF6
    Opcode { mnemonic: "RST", bytes: 1, immediate: true, execute: undefined }, // 0xF7
    Opcode { mnemonic: "LD", bytes: 2,  immediate: true, execute: undefined }, // 0xF8
    Opcode { mnemonic: "LD", bytes: 1, immediate: true, execute: undefined }, // 0xF9
    Opcode { mnemonic: "LD", bytes: 3,  immediate: false, execute: undefined }, // 0xFA
    Opcode { mnemonic: "EI", bytes: 1,  immediate: true, execute: undefined }, // 0xFB
    Opcode { mnemonic: "ILLEGAL_FC", bytes: 1, immediate: true, execute: undefined }, // 0xFC
    Opcode { mnemonic: "ILLEGAL_FD", bytes: 1, immediate: true, execute: undefined }, // 0xFD
    Opcode { mnemonic: "CP", bytes: 2, immediate: true, execute: undefined }, // 0xFE
    Opcode { mnemonic: "RST", bytes: 1, immediate: true, execute: undefined }, // 0xFF
];

//256
pub const CB_OPCODES: [Opcode; 1] = 
[
    Opcode { mnemonic: "RLC", bytes: 2, immediate: true, execute: undefined }, // 0x00
    /*Opcode { mnemonic: "RLC", bytes: 2, cycles: &[8], immediate: true }, // 0x01
    Opcode { mnemonic: "RLC", bytes: 2, cycles: &[8], immediate: true }, // 0x02
    Opcode { mnemonic: "RLC", bytes: 2, cycles: &[8], immediate: true }, // 0x03
    Opcode { mnemonic: "RLC", bytes: 2, cycles: &[8], immediate: true }, // 0x04
    Opcode { mnemonic: "RLC", bytes: 2, cycles: &[8], immediate: true }, // 0x05
    Opcode { mnemonic: "RLC", bytes: 2, cycles: &[16], immediate: false }, // 0x06
    Opcode { mnemonic: "RLC", bytes: 2, cycles: &[8], immediate: true }, // 0x07
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[8], immediate: true }, // 0x08
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[8], immediate: true }, // 0x09
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[8], immediate: true }, // 0x0A
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[8], immediate: true }, // 0x0B
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[8], immediate: true }, // 0x0C
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[8], immediate: true }, // 0x0D
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[16], immediate: false }, // 0x0E
    Opcode { mnemonic: "RRC", bytes: 2, cycles: &[8], immediate: true }, // 0x0F
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[8], immediate: true }, // 0x10
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[8], immediate: true }, // 0x11
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[8], immediate: true }, // 0x12
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[8], immediate: true }, // 0x13
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[8], immediate: true }, // 0x14
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[8], immediate: true }, // 0x15
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[16], immediate: false }, // 0x16
    Opcode { mnemonic: "RL", bytes: 2, cycles: &[8], immediate: true }, // 0x17
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[8], immediate: true }, // 0x18
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[8], immediate: true }, // 0x19
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[8], immediate: true }, // 0x1A
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[8], immediate: true }, // 0x1B
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[8], immediate: true }, // 0x1C
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[8], immediate: true }, // 0x1D
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[16], immediate: false }, // 0x1E
    Opcode { mnemonic: "RR", bytes: 2, cycles: &[8], immediate: true }, // 0x1F
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[8], immediate: true }, // 0x20
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[8], immediate: true }, // 0x21
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[8], immediate: true }, // 0x22
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[8], immediate: true }, // 0x23
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[8], immediate: true }, // 0x24
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[8], immediate: true }, // 0x25
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[16], immediate: false }, // 0x26
    Opcode { mnemonic: "SLA", bytes: 2, cycles: &[8], immediate: true }, // 0x27
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[8], immediate: true }, // 0x28
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[8], immediate: true }, // 0x29
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[8], immediate: true }, // 0x2A
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[8], immediate: true }, // 0x2B
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[8], immediate: true }, // 0x2C
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[8], immediate: true }, // 0x2D
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[16], immediate: false }, // 0x2E
    Opcode { mnemonic: "SRA", bytes: 2, cycles: &[8], immediate: true }, // 0x2F
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[8], immediate: true }, // 0x30
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[8], immediate: true }, // 0x31
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[8], immediate: true }, // 0x32
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[8], immediate: true }, // 0x33
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[8], immediate: true }, // 0x34
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[8], immediate: true }, // 0x35
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[16], immediate: false }, // 0x36
    Opcode { mnemonic: "SWAP", bytes: 2, cycles: &[8], immediate: true }, // 0x37
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[8], immediate: true }, // 0x38
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[8], immediate: true }, // 0x39
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[8], immediate: true }, // 0x3A
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[8], immediate: true }, // 0x3B
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[8], immediate: true }, // 0x3C
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[8], immediate: true }, // 0x3D
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[16], immediate: false }, // 0x3E
    Opcode { mnemonic: "SRL", bytes: 2, cycles: &[8], immediate: true }, // 0x3F
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x40
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x41
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x42
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x43
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x44
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x45
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x46
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x47
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x48
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x49
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x4A
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x4B
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x4C
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x4D
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x4E
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x4F
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x50
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x51
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x52
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x53
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x54
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x55
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x56
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x57
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x58
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x59
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x5A
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x5B
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x5C
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x5D
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x5E
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x5F
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x60
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x61
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x62
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x63
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x64
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x65
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x66
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x67
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x68
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x69
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x6A
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x6B
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x6C
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x6D
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x6E
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x6F
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x70
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x71
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x72
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x73
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x74
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x75
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x76
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x77
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x78
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x79
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x7A
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x7B
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x7C
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x7D
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[12], immediate: false }, // 0x7E
    Opcode { mnemonic: "BIT", bytes: 2, cycles: &[8], immediate: true }, // 0x7F
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x80
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x81
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x82
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x83
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x84
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x85
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0x86
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x87
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x88
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x89
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x8A
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x8B
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x8C
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x8D
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0x8E
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x8F
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x90
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x91
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x92
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x93
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x94
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x95
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0x96
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x97
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x98
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x99
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x9A
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x9B
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x9C
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x9D
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0x9E
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0x9F
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA0
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA1
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA2
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA3
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA4
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA5
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0xA6
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA7
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA8
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xA9
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xAA
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xAB
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xAC
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xAD
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0xAE
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xAF
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB0
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB1
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB2
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB3
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB4
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB5
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0xB6
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB7
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB8
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xB9
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xBA
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xBB
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xBC
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xBD
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[16], immediate: false }, // 0xBE
    Opcode { mnemonic: "RES", bytes: 2, cycles: &[8], immediate: true }, // 0xBF
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC0
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC1
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC2
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC3
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC4
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC5
    /*Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xC6
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC7
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC8
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xC9
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xCA
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xCB
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xCC
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xCD
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xCE
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xCF
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD0
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD1
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD2
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD3
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD4
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD5
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xD6
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD7
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD8
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xD9
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xDA
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xDB
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xDC
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xDD
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xDE
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xDF
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE0
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE1
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE2
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE3
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE4
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE5
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xE6
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE7
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE8
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xE9
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xEA
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xEB
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xEC
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xED
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xEE
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xEF
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF0
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF1
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF2*/
    Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF3
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF4
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF5
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xF6
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF7
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF8
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xF9
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xFA
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xFB
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xFC
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xFD
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[16], immediate: false }, // 0xFE
    //Opcode { mnemonic: "SET", bytes: 2, cycles: &[8], immediate: true }, // 0xFF*/
];

pub fn process_instruction(cpu: &mut Cpu) 
{
    println!("PC before fetch: {:04X}", cpu.regs.PC);

    let opcode = fetch_opcode(cpu);

    println!("PC after fetch: {:04X}", cpu.regs.PC);

    let cycles = (opcode.execute)(cpu);

    cpu.instruction_cycles +=  cycles as u8;

    println!(
        "PC: {:#06X} | Opcode: {:#04X} | Mnemonic: {:<10} | Bytes: {} | Cycles: {}",
        cpu.regs.PC.wrapping_sub(opcode.bytes as u16), // PC points to next instruction after fetch
        cpu.fetch_byte(cpu.regs.PC.wrapping_sub(opcode.bytes as u16)), // the opcode byte itself
        opcode.mnemonic,
        opcode.bytes,
        cycles
    );


}

pub fn fetch_opcode(cpu: &mut Cpu) -> &'static Opcode
{
    let opcode_byte = cpu.fetch_byte(cpu.regs.PC);
    cpu.set_register_16( cpu.regs.PC.wrapping_add(1), "PC");

    if opcode_byte == 0xCB
    {
        let cb_byte = cpu.fetch_byte(cpu.regs.PC);
        cpu.set_register_16( cpu.regs.PC.wrapping_add(1), "PC");

        &CB_OPCODES[cb_byte as usize]
    }
    else  
    {
        &OPCODES[opcode_byte as usize]  
    }
}

// return the byte at pc and increment pc 
pub fn next_byte(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");

    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let b = cpu.fetch_byte(pc);

    b
}

// next word function
//I might not use this I might just use read_u16_from pc
// it makes more sense to me logically speaking.
pub fn next_word(cpu: &mut Cpu) -> u16
{
    let pc = cpu.get_register_16("PC");

    cpu.set_register_16(pc.wrapping_add(1), "PC");


    let b1 = next_byte(cpu) as u16;
    let b2 = next_byte(cpu) as u16;

    (b1 << 8) | b2 
}

pub fn read_u16_from_pc(cpu: &mut Cpu) -> u16
{
    let pc = cpu.get_register_16("PC");
    let mut low = cpu.inter.read_byte(pc);
    let mut high = cpu.inter.read_byte(pc.wrapping_add(1));
    cpu.set_register_16(pc.wrapping_add(2), "PC");

    ((high as u16) << 8) | (low as u16)
}

pub fn read_u8_from_pc(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");

    let val = cpu.inter.read_byte(pc);

    cpu.set_register_16(pc.wrapping_add(1), "PC");

    val
}


// 0x00
pub fn nop(cpu: &mut Cpu) -> u8
{
    //nothing happens
    4
}

//0x01
pub fn ld_bc_d16(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let bc = read_u16_from_pc(cpu);
    cpu.set_register_16(bc, "BC");

    12
}

//0x02
pub fn ld_bc_a(cpu: &mut Cpu) -> u8
{

    let a = cpu.get_register_8('A');
    let bc = cpu.get_register_16("BC");

    cpu.store_byte(bc, a);

    8
}

//0x03
pub fn inc_bc(cpu: &mut Cpu) -> u8
{
    let bc = cpu.get_register_16("BC");
    cpu.set_register_16(bc.wrapping_add(1), "BC");

    8
}

//0x04
pub fn inc_b(cpu: &mut Cpu) -> u8
{
    let b = cpu.get_register_8('B');

    cpu.set_flags('H', (b & 0x0f) + 1 > 0x0f);
    cpu.set_register_8(b.wrapping_add(1), 'B');

    cpu.set_flags('Z', b == 0);
    cpu.set_flags('N', false);

    4
}

//0x05
pub fn dec_b(cpu: &mut Cpu) -> u8
{
    let mut b = cpu.regs.B;

    // if H the nibble is zero set flag true
    cpu.set_flags('H', b & 0xf == 0);

    b = b.wrapping_sub(1);
    
    cpu.set_register_8(b, 'B');

    // if b is equal to zero set flag true.
    cpu.set_flags('Z', b == 0);
    // if there is a subtract operation, true.
    cpu.set_flags('N', true);

    4
}

//0x06
pub fn ld_b_d8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let val = read_u8_from_pc(cpu);
    cpu.set_register_8(val, 'B');

    8
}

//0x07
pub fn rcla(cpu: &mut Cpu) -> u8
{
    let msb = cpu.get_register_8('A') >> 7 & 0x01;
    let a = cpu.get_register_8('A');

    //I need to check this flag, I am not sure if I set it right.
    cpu.set_flags('C', msb == 1);

    cpu.set_flags('H', false);

    cpu.set_flags('Z', false);

    cpu.set_flags('N', false);

    cpu.set_register_8( a << 1 | msb, 'A');

    4
}

//0x08
pub fn ld_a16_sp(cpu: &mut Cpu) -> u8
{
    let low = read_u8_from_pc(cpu);
    let high = read_u8_from_pc(cpu);
    let addr = ((high as u16) << 8) | (low as u16);

    let sp = cpu.get_register_16("SP");

    cpu.inter.write_byte(addr, (sp & 0xFF) as u8);
    cpu.inter.write_byte(addr.wrapping_add(1), (sp >> 8) as u8);

    20
}

//0x09
pub fn add_hl_bc(cpu: &mut Cpu) -> u8
{
    let bc: u16 = cpu.get_register_16("BC");
    let hl: u16 = cpu.get_register_16("HL");

    let hl_add = hl.wrapping_add(bc);

    cpu.set_flags('H', (hl & 0x0fff) + (bc & 0xfff) > 0x0fff);
    cpu.set_flags('C', (hl as u32 + bc as u32) > 0xffff);

    cpu.set_register_16(hl_add , "HL");

    cpu.set_flags('N', false);

    8
}

//0x0A
pub fn ld_a_bc(cpu: &mut Cpu) -> u8
{
    let bc = cpu.get_register_16("BC");
    let val = cpu.inter.read_byte(bc);

    cpu.set_register_8(val, 'A');

    8
}

//0x0B
pub fn dec_bc(cpu: &mut Cpu) -> u8
{
    let mut bc = cpu.get_register_16("BC");

    bc = bc.wrapping_sub(1);
    cpu.set_register_16(bc, "BC");

    8
}

//0x0C
pub fn inc_c(cpu: &mut Cpu) -> u8
{
    let c = cpu.get_register_8('C');

    cpu.set_flags('H', (c & 0x0f) + 1 > 0x0f);

    cpu.set_register_8(c.wrapping_add(1), 'C');

    cpu.set_flags('Z', c == 0);
    cpu.set_flags('N', false);

    4
}

//0x0D
pub fn dec_c(cpu: &mut Cpu) -> u8
{
    let c = cpu.get_register_8('C');
    cpu.set_flags('H', (c & 0x0f) + 1 > 0x0f); 

    cpu.set_register_8(c.wrapping_sub(1), 'C');

    cpu.set_flags('Z', c == 0);
    cpu.set_flags('N', true);

    4
}

//0x0E
pub fn ld_c_d8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC").wrapping_add(1);
    cpu.set_register_16(pc, "PC");

    let v = read_u8_from_pc(cpu);
    cpu.set_register_8(v, 'C');

    8
}

//0x0F
pub fn rrca(cpu: &mut Cpu) -> u8
{
    let lsb = cpu.get_register_8('A') & 0x01;
    let a = cpu.get_register_8('A');

    //I need to check this flag, I am not sure if I set it right.
    cpu.set_flags('C', lsb == 1);
    cpu.set_flags('Z', false);
    cpu.set_flags('N', false);
    cpu.set_flags('H', false);

    cpu.set_register_8( (a >> 1) | (lsb << 7), 'A');

    4
}

//0x10
pub fn stop(cpu: &mut Cpu) -> u8
{
    //stop
    4
}

//0x11
pub fn ld_de_d16(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let de = read_u16_from_pc(cpu);
    cpu.set_register_16(de, "DE");

    12
}

//0x12
pub fn ld_de_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let de = cpu.get_register_16("DE");

    cpu.store_byte(de, a);

    8
}

//0x13
pub fn inc_de(cpu: &mut Cpu) -> u8
{
    let de = cpu.get_register_16("DE");
    cpu.set_register_16(de.wrapping_add(1), "DE");

    8
}

//0x14
pub fn inc_d(cpu: &mut Cpu) -> u8
{
    let d = cpu.get_register_8('D');

    cpu.set_flags('H', (d & 0x0f) + 1 > 0x0f);
    cpu.set_register_8(d.wrapping_add(1), 'D');

    cpu.set_flags('Z', d == 0);
    cpu.set_flags('N', false);

    4
}

//0x15
pub fn dec_d(cpu: &mut Cpu) -> u8
{
    let d = cpu.get_register_8('D');
    cpu.set_flags('H', (d & 0x0f) + 1 > 0x0f); 

    cpu.set_register_8(d.wrapping_sub(1), 'D');

    cpu.set_flags('Z', d == 0);
    cpu.set_flags('N', true);

    4
} 

//0x16
pub fn ld_d_d8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let val = read_u8_from_pc(cpu);
    cpu.set_register_8(val, 'D');

    8
}

//0x17
pub fn rla(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');

    let old_carry = if cpu.get_flags('C') {1} else {0};
    let new_carry = cpu.get_register_8('A') >> 7 & 0x01;

    cpu.set_flags('C', new_carry == 1);

    cpu.set_flags('H', false);

    cpu.set_flags('Z', false);

    cpu.set_flags('N', false);

    cpu.set_register_8( a << 1 | old_carry, 'A');

    4
}

//0x18
pub fn jr_r8(cpu: &mut Cpu) -> u8
{
    let offset = read_u8_from_pc(cpu) as i8;

    let pc = cpu.get_register_16("PC");
    let new = pc.wrapping_add(offset as i16 as u16);
    
    cpu.set_register_16(new, "PC");

    12
}

//0x19
pub fn add_hl_de(cpu: &mut Cpu) -> u8
{
    let de: u16 = cpu.get_register_16("DE");
    let hl: u16 = cpu.get_register_16("HL");

    let hl_add = hl.wrapping_add(de);

    cpu.set_flags('H', (hl & 0x0fff) + (de & 0xfff) > 0x0fff);
    cpu.set_flags('C', (hl as u32 + de as u32) > 0xffff);

    cpu.set_register_16(hl_add , "HL");

    cpu.set_flags('N', false);

    8
    
}

//0x1A
pub fn ld_a_de(cpu: &mut Cpu) -> u8
{
    let de = cpu.get_register_16("DE");
    let val = cpu.inter.read_byte(de);

    cpu.set_register_8(val, 'A');

    8

}

//0x1B
pub fn dec_de(cpu: &mut Cpu) -> u8
{
    let mut de = cpu.get_register_16("DE");

    de = de.wrapping_sub(1);
    cpu.set_register_16(de, "DE");

    8
}

//0x1C
pub fn inc_e(cpu: &mut Cpu) -> u8
{
    let e = cpu.get_register_8('E');

    cpu.set_flags('H', (e & 0x0f) + 1 > 0x0f);

    cpu.set_register_8(e.wrapping_add(1), 'E');

    cpu.set_flags('Z', e == 0);
    cpu.set_flags('N', false);

    4
}

//0x1D
pub fn dec_e(cpu: &mut Cpu) -> u8
{
    let e = cpu.get_register_8('E');
    cpu.set_flags('H', (e & 0x0f) + 1 > 0x0f); 

    cpu.set_register_8(e.wrapping_sub(1), 'E');

    cpu.set_flags('Z', e == 0);
    cpu.set_flags('N', true);

    4
    
}

//0x1E
pub fn ld_e_d8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC").wrapping_add(1);
    cpu.set_register_16(pc, "PC");

    let v = read_u8_from_pc(cpu);
    cpu.set_register_8(v, 'E');

    8
}

//0x1F
pub fn rra(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let carry_in = if cpu.get_flags('C') { 1 } else { 0 };
    let carry_out = a & 0x01;

    let rotated = (a >> 1) | (carry_in << 7);
    cpu.set_register_8(rotated, 'A');

    cpu.set_flags('Z', false);
    cpu.set_flags('N', false);
    cpu.set_flags('H', false);
    cpu.set_flags('C', carry_out != 0);

    4

}

//0x20
pub fn jr_nz_r8(cpu: &mut Cpu) -> u8 
{

    let raw = read_u8_from_pc(cpu); // returns u8 and increments PC
    let offset = raw as i8;
    let pc_after_offset = cpu.get_register_16("PC");

    if !cpu.get_flags('Z') 
    {
        // jump taken: add signed offset to the address *after* operand
        let new_pc = pc_after_offset.wrapping_add(offset as i16 as u16);
        cpu.set_register_16(new_pc, "PC");
        12
    } 
    else 
    {
       8
    }

}

//0x21
pub fn ld_hl_d16(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let hl = read_u16_from_pc(cpu);
    cpu.set_register_16(hl, "HL");
    12
}

//0x22
pub fn ld_hl_inc_a(cpu: &mut Cpu) -> u8 
{
    let hl = cpu.get_register_16("HL");
    let a = cpu.get_register_8('A');

    cpu.inter.write_byte(hl, a);
    cpu.set_register_16(hl.wrapping_add(1), "HL");

    8 
}

//0x23
pub fn inc_hl(cpu: &mut Cpu) -> u8 
{
    let hl = cpu.get_register_16("HL");
    cpu.set_register_16(hl.wrapping_add(1), "HL");

    8
}

//0x24
pub fn inc_h(cpu: &mut Cpu) -> u8
{
    let h = cpu.get_register_8('H');

    cpu.set_flags('H', (h & 0x0f) + 1 > 0x0f);
    cpu.set_register_8(h.wrapping_add(1), 'H');

    cpu.set_flags('Z', h == 0);
    cpu.set_flags('N', false);

    4
}

//0x25
pub fn dec_h(cpu: &mut Cpu) -> u8
{
    let h = cpu.get_register_8('H');
    cpu.set_flags('H', (h & 0x0f) + 1 > 0x0f); 

    cpu.set_register_8(h.wrapping_sub(1), 'H');

    cpu.set_flags('Z', h == 0);
    cpu.set_flags('N', true);

    4
}

//0x26
pub fn ld_h_d8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let val = read_u8_from_pc(cpu);
    cpu.set_register_8(val, 'H');

    8
}

//0x27
pub fn daa(cpu: &mut Cpu) -> u8
{
    let mut a = cpu.get_register_8('A');
    let mut adjust = 0u8;
    let mut carry = false;

    let n = cpu.get_flags('N');
    let h = cpu.get_flags('H');
    let c = cpu.get_flags('C');

    if !n 
    {
        if c || a > 0x99 
        {
            adjust |= 0x60;
            carry = true;
        }
        if h || (a & 0x0F) > 0x09 
        {
            adjust |= 0x06;
        }
        a = a.wrapping_add(adjust);
    } 
    else 
    {
        if c 
        {
            adjust |= 0x60;
            carry = true;
        }
        if h 
        {
            adjust |= 0x06;
        }
        a = a.wrapping_sub(adjust);
    }

    cpu.set_register_8(a, 'A');
    cpu.set_flags('Z', a == 0);
    cpu.set_flags('H', false);
    cpu.set_flags('C', carry);

    4 

}

//0x28
pub fn jr_z_r8(cpu: &mut Cpu) -> u8
{
    let raw = read_u8_from_pc(cpu); // returns u8 and increments PC
    let offset = raw as i8;

    let pc_after_offset = cpu.get_register_16("PC");

    if cpu.get_flags('Z') 
    {
        // jump taken: add signedw4re4re3r                                                                                                                                                                                                                                                                                                                                       offset to the address *after* operand

        let new_pc = pc_after_offset.wrapping_add(offset as i16 as u16);
        cpu.set_register_16(new_pc, "PC");
        12
    } 
    else 
    {
       8
    }
}

//0x29
pub fn add_hl_hl(cpu: &mut Cpu) -> u8
{
    let hl: u16 = cpu.get_register_16("HL");

    let hl_add = hl.wrapping_add(hl);

    cpu.set_flags('H', (hl & 0x0fff) + (hl & 0xfff) > 0x0fff);
    cpu.set_flags('C', (hl as u32 + hl as u32) > 0xffff);

    cpu.set_register_16(hl_add , "HL");

    cpu.set_flags('N', false);

    8
}

//0x2A
pub fn ld_a_hl_inc(cpu: &mut Cpu) -> u8
{
    
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'A');
    cpu.set_register_16(hl.wrapping_add(1), "HL");

    8    
}

//0x2B
pub fn dec_hl(cpu: &mut Cpu) -> u8
{
    let mut hl = cpu.get_register_16("HL");

    hl = hl.wrapping_sub(1);
    cpu.set_register_16(hl, "HL");

    8
}

//0x2C
pub fn inc_l(cpu: &mut Cpu) -> u8
{
    let l = cpu.get_register_8('L');

    cpu.set_flags('H', (l & 0x0f) + 1 > 0x0f);

    cpu.set_register_8(l.wrapping_add(1), 'L');

    cpu.set_flags('Z', l == 0);
    cpu.set_flags('N', false);

    4
}

//0x2D
pub fn dec_l(cpu: &mut Cpu) -> u8
{
    let l = cpu.get_register_8('L');
    cpu.set_flags('H', (l & 0x0f) + 1 > 0x0f); 

    cpu.set_register_8(l.wrapping_sub(1), 'L');

    cpu.set_flags('Z', l == 0);
    cpu.set_flags('N', true);

    4
}

//0x2E
pub fn ld_l_d8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC").wrapping_add(1);
    cpu.set_register_16(pc, "PC");

    let v = read_u8_from_pc(cpu);
    cpu.set_register_8(v, 'L');

    8
}

//0x2F
pub fn cpl(cpu: &mut Cpu) -> u8
{
        // Complement the A register
        let a = cpu.get_register_8('A');
        let result = !a;
        cpu.set_register_8(result, 'A');
    
        // Set flags: N and H = 1, Z and C unaffected
        cpu.set_flags('N', true);
        cpu.set_flags('H', true);
    
        4
}

//0x30
pub fn jr_nc_r8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    if cpu.inter.read_byte(pc) == 0x30
    {
        cpu.set_register_16(pc.wrapping_add(1), "PC");
    }

    let pc = cpu.get_register_16("PC");
    let offset = cpu.inter.read_byte(pc) as i8;
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let pc_after = cpu.get_register_16("PC");
    if !cpu.get_flags('C') 
    {
        let new_pc = pc_after.wrapping_add(offset as i16 as u16);
        cpu.set_register_16(new_pc, "PC");
        12
    } 
    else 
    {
        8
    }
}

//0x31
pub fn ld_sp_d16(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let sp = read_u16_from_pc(cpu);
    cpu.set_register_16(sp, "SP");
    12
}

//0x32
pub fn ld_hl_dec_a(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let a = cpu.get_register_8('A');

    cpu.inter.write_byte(hl, a);
    cpu.set_register_16(hl.wrapping_sub(1), "HL");

    8
}

//0x33
pub fn inc_sp(cpu: &mut Cpu) -> u8
{
    let sp = cpu.get_register_16("SP");
    cpu.set_register_16(sp.wrapping_add(1), "SP");

    8
}

//0x34
pub fn inc_hl_mem(cpu: &mut Cpu) -> u8
{
    let addr = cpu.get_register_16("HL");
    let value = cpu.inter.read_byte(addr);
    let result = value.wrapping_add(1);

    cpu.inter.write_byte(addr, result);

    // Update flags
    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', false);
    cpu.set_flags('H', (value & 0x0F) + 1 > 0x0F);

    12
}

//0x35
pub fn dec_hl_mem(cpu: &mut Cpu) -> u8
{
    let addr = cpu.get_register_16("HL");
    let value = cpu.inter.read_byte(addr);
    let result = value.wrapping_sub(1);

    cpu.inter.write_byte(addr, result);

    // Update flags
    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', true);
    cpu.set_flags('H', (value & 0x0F) == 0);

    12
}


//0xAF
pub fn in_xor_a(cpu: &mut Cpu) -> u8
{
    cpu.regs.A ^= cpu.regs.A;
    cpu.set_flags('Z', true);
    cpu.set_flags('N', false);
    cpu.set_flags('H', false);
    cpu.set_flags('C', false);

    4
}

// 0xC3
pub fn jp_a16(cpu: &mut Cpu) -> u8
{
    let val = next_word(cpu);

    cpu.set_register_16(val, "pc");

    cpu.delay(1);

    16

}

//0xF3
pub fn in_di(cpu: &mut Cpu) -> u8
{
    cpu.disable_interrupts();

    4
}

//undefined
pub fn undefined(cpu: &mut Cpu) -> u8
{
    "function not defined";
    0
}


// Basic set of tests for each opcode.
// Could optimize tests for each flag when looking into making more efficient.
#[cfg(test)]
mod tests
{
    use crate::interconnect::{self, Interconnect};

    use super::*;

    //0x01
    #[test]
    fn test_ld_bc_d16()
    {
        let mut memory = vec![0; 0x10000];

        memory[0x0000] = 0x01;
        memory[0x0001] = 0x34;
        memory[0x0002] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_bc_d16(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0003);
        println!("{}", cpu.get_register_16("BC"));
        assert_eq!(cpu.get_register_16("BC"), 0x1234);
    }

    //0x02
    #[test]
    fn test_ld_bc_a()
    {
        let interconnect = Interconnect::new(vec![0; 0x10000]);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x42, 'A');
        cpu.set_register_8(0xC0, 'B');
        cpu.set_register_8(0x00, 'C');

        let cycles = ld_bc_a(&mut cpu);
        assert_eq!(cycles, 8);
        println!("first test passed");

        assert_eq!(cpu.inter.read_byte(0xC000), 0x42);
    }

    //0x03
    #[test]
    fn test_inc_bc()
    {
        let interconnect = Interconnect::new(vec![0; 0x10000]);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x21, "BC");

        let cycles = inc_bc(&mut cpu);

        assert_eq!(cycles, 8);

        assert_eq!(cpu.get_register_16("BC"), 0x22);

    }

    //0x04
    #[test]
    fn test_inc_b()
    {
        let interconnect = Interconnect::new(vec![0; 0x10000]);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x08, 'B');

        let cycles = inc_b(&mut cpu);
        assert_eq!(cycles, 4);

        assert_eq!(cpu.get_register_8('B'), 0x09);

    }

    //0x05
    #[test]
    fn test_dec_b()
    {
        let interconnect = Interconnect::new(vec![0; 0x10000]);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x08, 'B');


        let cycles = dec_b(&mut cpu);
        assert_eq!(cycles, 4);

        assert_eq!(cpu.get_register_8('B'), 0x07);
    }

    //0x06
    #[test]
    fn test_ld_b_n8()
    {
        let mut memory = vec![0; 0x10000];

        memory[0x0000] = 0x06;
        memory[0x0001] = 0x42;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_b_d8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('B'), 0x42);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
    }

    //0x07
    #[test]
    fn test_rcla()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);


        cpu.set_register_8(0x96, 'A');

        let cycles = rcla(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x2D);

    }

    //0x08
    #[test]
    fn test_ld_a16_sp()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x08;
        memory[0x0001] = 0x00;
        memory[0x0002] = 0xC0;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x1234, "SP");
        cpu.set_register_16(0x0000, "PC");

        let opcode = fetch_opcode(&mut cpu);

        let cycles = ld_a16_sp(&mut cpu);

        assert_eq!(cycles, 20);
        assert_eq!(cpu.get_register_16("PC"), 0x0003);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x34);
        assert_eq!(cpu.inter.read_byte(0xC001), 0x12);
    }

    //0x09
    #[test]
    fn test_add_hl_bc()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x1234, "BC");
        cpu.set_register_16(0x0F4E, "HL");

        let cycles = add_hl_bc(&mut cpu);

        assert_eq!(cycles, 0x08);
        assert_eq!(cpu.get_register_16("HL"), 0x2182);
    }

    //0x0A
    #[test]
    fn test_ld_a_bc()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "BC");
        cpu.set_register_8(0x00, 'A');
    
        let cycles = ld_a_bc(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('A'), 0x7F);

    }

    //0x0B
    #[test]
    fn test_dec_bc()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x22, "BC");

        let cycles = dec_bc(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("BC"), 0x21);
    }

    //0x0C
    #[test]
    fn test_inc_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x34, 'C');

        let cycles = inc_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('C'), 0x35);

    }

    //0x0D
    #[test]
    fn test_dec_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'C');

        let cycles = dec_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('C'), 0x24);

    }

    //0x0E
    #[test]
    fn test_ld_c_d8()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x0E;
        memory[0x0001] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_c_d8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
        assert_eq!(cpu.get_register_8('C'), 0x12);

    }

    //0x0F
    #[test]
    fn test_rrca()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x12, 'A');

        let cycles = rrca(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x09);
    }

    // 0x11
    #[test]
    fn test_ld_de_d16()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x11;
        memory[0x0001] = 0x23;
        memory[0x0002] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_de_d16(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0003);
        assert_eq!(cpu.get_register_16("DE"), 0x1223);
    }

    //0x12
    // I need to write this outloud so I think better.
    // you have a reg A and you have the 16byte Reg DE.
    // when you store the reg A in Reg DE what is actaully is doing is storing
    // Reg A into a memory Address (the one reg DE is set too) and when you look at that memory address,
    // it will have the value of reg A.
    #[test]
    fn test_ld_de_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x42, 'A');
        cpu.set_register_8(0xC0, 'D');
        cpu.set_register_8(0x00, 'E');

        let cycles = ld_de_a(&mut cpu);

        assert_eq!(cycles, 8);
        println!("first test passed");

        assert_eq!(cpu.inter.read_byte(0xC000), 0x42);
    }

    //0x13
    #[test]
    fn test_inc_de()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x25, "DE");

        let cycles = inc_de(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("DE"), 0x26);

    }

    //0x14
    #[test]
    fn test_inc_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'D');

        let cycles = inc_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('D'), 0x26);

    }

    //0x15
    #[test]
    fn test_dec_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'D');

        let cycles = dec_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('D'), 0x24);

    }

    //0x16
    #[test]
    fn test_ld_d_d8()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x16;
        memory[0x0001] = 0x23;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_d_d8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
        assert_eq!(cpu.get_register_8('D'), 0x23);
    }

    //0x17
    #[test]
    fn test_rla()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x12, 'A');

        let cycles = rla(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x24);
    }

    //0x18
    #[test]
    fn test_jr_r8()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x18;
        memory[0x0001] = 0x23;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0001, "PC");

        let cycles = jr_r8(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0025);

    }

    //0x19
    #[test]
    fn test_add_hl_de()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x1234, "DE");
        cpu.set_register_16(0x0F4E, "HL");

        let cycles = add_hl_de(&mut cpu);

        assert_eq!(cycles, 0x08);
        assert_eq!(cpu.get_register_16("HL"), 0x2182);
    }

    //0x1A
    #[test]
    fn test_ld_a_de()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "DE");
        cpu.set_register_8(0x00, 'A');
    
        let cycles = ld_a_de(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('A'), 0x7F);

    }

    //0x1B
    #[test]
    fn test_dec_de()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x22, "DE");

        let cycles = dec_de(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("DE"), 0x21);
    }

    //0x1C
    #[test]
    fn test_inc_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x34, 'E');

        let cycles = inc_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('E'), 0x35);
    
    }

    //0x1D
    #[test]
    fn test_dec_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'E');

        let cycles = dec_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('E'), 0x24);

    }

    //0x1E
    #[test]
    fn test_ld_e_d8()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x1E;
        memory[0x0001] = 0x23;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_e_d8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
        assert_eq!(cpu.get_register_8('E'), 0x23);

    }

    //0x1F
    #[test]
    fn test_rra() {
        let mut memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_8(0b1001_0011, 'A'); // 0x93
        cpu.set_flags('C', true);
    
        let cycles = rra(&mut cpu);
    
        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0b1100_1001); // 0xC9
        assert!(!cpu.get_flags('Z'));
        assert!(!cpu.get_flags('N'));
        assert!(!cpu.get_flags('H'));
        assert!(cpu.get_flags('C')); // bit0 was 1
    }

    //0x20
    #[test]
    fn test_jr_nz_r8_taken() 
    {
        let mut memory = vec![0; 0x10000];
        memory[0x0000] = 0x20;
        memory[0x0001] = 0x05;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        // Simulate dispatcher already consumed the opcode:
        cpu.set_register_16(0x0001, "PC");

        // Also place your operand (the offset) at memory[0x0001]
        cpu.inter.write_byte(0x0001, 0x20); // e.g. +32 offset

        cpu.set_flags('Z', false);

        let cycles = jr_nz_r8(&mut cpu);
        assert_eq!(cycles, 12);
    }

    //0x20
    #[test]
    fn test_jr_nz_r8_not_taken() 
    {
        let mut memory = vec![0; 0x10000];
        memory[0x0001] = 0x20; // +32 offset byte (arbitrary)
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        // Simulate PC after opcode fetch (0x0001)
        cpu.set_register_16(0x0001, "PC");
        cpu.set_flags('Z', true); // Zero flag = 1 → NOT taken
    
        let cycles = jr_nz_r8(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
    }

    //0x21
    #[test]
    fn test_ld_hl_d16()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x21;
        memory[0x0001] = 0x23;
        memory[0x0002] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_hl_d16(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0003);
        assert_eq!(cpu.get_register_16("HL"), 0x1223);
    }

    //0x22
    #[test]
    fn test_ld_hl_inc_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x42, 'A');
        cpu.set_register_8(0xC0, 'H');
        cpu.set_register_8(0x00, 'L');

      
        let cycles = ld_hl_inc_a(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x42);
        assert_eq!(cpu.get_register_8('A'), 0x42);
        assert_eq!(cpu.get_register_16("HL"), 0xC001);
    }

    //0x23
    #[test]
    fn test_inc_hl()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x25, "HL");

        let cycles = inc_hl(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("HL"), 0x26);
    }

    //0x24
    #[test]
    fn test_inc_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'H');

        let cycles = inc_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('H'), 0x26);
    }

    //0x25
    #[test]
    fn test_dec_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'H');

        let cycles = dec_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('H'), 0x24);
    }

    //0x26
    #[test]
    fn test_ld_h_d8()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x16;
        memory[0x0001] = 0x23;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_h_d8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
        assert_eq!(cpu.get_register_8('H'), 0x23);
    }
    
    //0x27
    #[test]
    fn test_daa()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_8(0x9A, 'A');
        cpu.set_flags('N', false);
        cpu.set_flags('H', false);
        cpu.set_flags('C', false);
    
        let cycles = daa(&mut cpu);
    
        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x00);
        assert!(cpu.get_flags('Z'));
        assert!(cpu.get_flags('C'));
    }

    //0x28
    #[test]
    fn test_jr_z_r8_taken() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0001, "PC");
        cpu.set_flags('Z', true);

        // Put offset 0x05 at PC+1
        cpu.inter.write_byte(0x0001, 0x05);

        let cycles = jr_z_r8(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0002 + 0x05);
    }

    //0x28
    #[test]
    fn test_jr_z_r8_not_taken() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0001, "PC");
        cpu.set_flags('Z', false);

        // Put offset 0x05 at PC+1
        cpu.inter.write_byte(0x0001, 0x05);

        let cycles = jr_z_r8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002); // skipped
    }

    //0x29
    #[test]
    fn test_add_hl_hl()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x1234, "HL");

        let cycles = add_hl_hl(&mut cpu);

        assert_eq!(cycles, 0x08);
        assert_eq!(cpu.get_register_16("HL"), 0x2468);
    }

    //0x2A
    #[test]
    fn test_ld_a_hl_inc()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0xC000] = 0x7F;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC000, "HL");
        cpu.set_register_8(0x00, 'A');

        let cycles = ld_a_hl_inc(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('A'), 0x7F);
        assert_eq!(cpu.get_register_16("HL"), 0xC001);
    }

    //0x2B
    #[test]
    fn test_dec_hl()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x22, "HL");

        let cycles = dec_hl(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("HL"), 0x21);
    }

    //0x2C
    #[test]
    fn test_inc_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x34, 'L');

        let cycles = inc_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('L'), 0x35);
    }

    //0x2D
    #[test]
    fn test_dec_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'L');

        let cycles = dec_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('L'), 0x24);
    }

    //0x2E
    #[test]
    fn test_ld_l_d8()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x2E;
        memory[0x0001] = 0x23;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_l_d8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
        assert_eq!(cpu.get_register_8('L'), 0x23);
    }

    //0x2F
    #[test]
    fn test_cpl() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0b1010_1010, 'A');
        cpu.set_flags('Z', false);
        cpu.set_flags('C', true); 

        let cycles = cpl(&mut cpu);

        assert_eq!(cycles, 4);

        assert_eq!(cpu.get_register_8('A'), 0b0101_0101);

        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('Z'), false); // unchanged
        assert_eq!(cpu.get_flags('C'), true);  // unchanged
    }

    //0x30
    #[test]
    fn test_jr_nc_r8_taken() 
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0x0000] = 0x30; // opcode JR NC,r8
        memory[0x0001] = 0x06; // offset +6

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC"); // PC points at offset byte
        cpu.set_flags('C', false); // Carry = 0 → jump should be taken

        let cycles = jr_nc_r8(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0008); // 1 + 6 = 7
    }

    //0x30
    #[test]
    fn test_jr_nc_r8_not_taken() 
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0x0000] = 0x30;
        memory[0x0001] = 0x06;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0001, "PC");
        cpu.set_flags('C', true); // Carry = 1 → no jump

        let cycles = jr_nc_r8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002); // only offset byte consumed
    }

    //0x31
    #[test]
    fn test_ld_sp_d16()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x21;
        memory[0x0001] = 0x23;
        memory[0x0002] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_sp_d16(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0003);
        assert_eq!(cpu.get_register_16("SP"), 0x1223);
    }

    //0x32
    #[test]
    fn test_ld_hl_dec_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x42, 'A');
        cpu.set_register_8(0xC0, 'H');
        cpu.set_register_8(0x00, 'L');

      
        let cycles = ld_hl_dec_a(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x42);
        assert_eq!(cpu.get_register_8('A'), 0x42);
        assert_eq!(cpu.get_register_16("HL"), 0xBFFF)
    }

    //0x33
    #[test]
    fn test_inc_sp()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x25, "SP");

        let cycles = inc_sp(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("SP"), 0x26);
    }

    // 0x34
    #[test]
    fn test_inc_hl_reg() 
    {
        // Create memory and CPU
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC000] = 0x0F; // initial value at (HL)

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        // Set HL to point to the memory location
        cpu.set_register_16(0xC000, "HL");

        cpu.set_flags('Z', true);
        cpu.set_flags('N', true);
        cpu.set_flags('H', false);
        cpu.set_flags('C', true); // Carry should remain unchanged

        let cycles = inc_hl_mem(&mut cpu);

        // Verify timing
        assert_eq!(cycles, 12);

        assert_eq!(cpu.inter.read_byte(0xC000), 0x10);

        // Verify flags
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), true);
    }

    #[test]
    fn test_inc_hl_zero_result() {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC000] = 0xFF;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC000, "HL");

        let cycles = inc_hl_mem(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x00);
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('N'), false);
    }

    #[test]
    fn test_inc_hl_normal_no_halfcarry() {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC000] = 0x3A;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC000, "HL");

        let cycles = inc_hl_mem(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x3B);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('N'), false);
    }

    // 0x35
    #[test]
    fn test_dec_hl_reg() 
    {
        // Create memory and CPU
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC000] = 0x0F; // initial value at (HL)

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        // Set HL to point to the memory location
        cpu.set_register_16(0xC000, "HL");

        cpu.set_flags('Z', true);
        cpu.set_flags('N', true);
        cpu.set_flags('H', true);
        cpu.set_flags('C', true); // Carry should remain unchanged

        let cycles = dec_hl_mem(&mut cpu);

        // Verify timing
        assert_eq!(cycles, 12);

        assert_eq!(cpu.inter.read_byte(0xC000), 0x0E);

        // Verify flags
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), true);
    }

    #[test]
    fn test_dec_hl_zero_result() {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC000] = 0x01;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC000, "HL");

        let cycles = dec_hl_mem(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x00);
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('N'), true);
    }

    #[test]
    fn test_dec_hl_normal_no_halfcarry() {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC000] = 0x3A;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC000, "HL");

        let cycles = dec_hl_mem(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x39);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('N'), true);
    }


}