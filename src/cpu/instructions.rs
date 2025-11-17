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
    Opcode { mnemonic: "LD", bytes: 2,     immediate: false, execute: ld_hl_d8 }, // 0x36
    Opcode { mnemonic: "SCF", bytes: 1,    immediate: true, execute: scf }, // 0x37
    Opcode { mnemonic: "JR", bytes: 2,     immediate: true, execute: jr_c_r8 }, // 0x38
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_hl_sp }, // 0x39
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_a_hl_dec }, // 0x3A
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_sp }, // 0x3B
    Opcode { mnemonic: "INC", bytes: 1,    immediate: true, execute: inc_a }, // 0x3C
    Opcode { mnemonic: "DEC", bytes: 1,    immediate: true, execute: dec_a }, // 0x3D
    Opcode { mnemonic: "LD", bytes: 2,     immediate: true, execute: ld_a_d8 }, // 0x3E
    Opcode { mnemonic: "CCF", bytes: 1,    immediate: true, execute: ccf }, // 0x3F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_b_b }, // 0x40
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_b_c }, // 0x41
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_b_d }, // 0x42
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_b_e }, // 0x43
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_b_h }, // 0x44
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_b_l }, // 0x45
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_b_hl }, // 0x46
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_b_a }, // 0x47
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_c_b }, // 0x48
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_c_c }, // 0x49
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_c_d }, // 0x4A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_c_e }, // 0x4B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_c_h }, // 0x4C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_c_l }, // 0x4D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_c_hl }, // 0x4E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_c_a },  // 0x4F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_d_b },// 0x50
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_d_c }, // 0x51
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_d_d }, // 0x52
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_d_e }, // 0x53
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_d_h }, // 0x54
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_d_l }, // 0x55
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_d_hl }, // 0x56
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_d_a }, // 0x57
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_e_b }, // 0x58
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_e_c }, // 0x59
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_e_d }, // 0x5A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_e_e }, // 0x5B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_e_h }, // 0x5C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_e_l }, // 0x5D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_e_hl }, // 0x5E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_e_a }, // 0x5F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_h_b }, // 0x60
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_h_c }, // 0x61
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_h_d }, // 0x62
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_h_e }, // 0x63
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_h_h }, // 0x64
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_h_l }, // 0x65
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_h_hl }, // 0x66
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_h_a }, // 0x67
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_l_b }, // 0x68
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_l_c }, // 0x69
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_l_d }, // 0x6A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_l_e }, // 0x6B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_l_h }, // 0x6C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_l_l }, // 0x6D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_l_hl }, // 0x6E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_l_a }, // 0x6F
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_b }, // 0x70
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_c }, // 0x71
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_d }, // 0x72
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_e }, // 0x73
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_h }, // 0x74
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_l }, // 0x75
    Opcode { mnemonic: "HALT", bytes: 1,   immediate: true, execute: halt }, // 0x76
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_hl_a }, // 0x77
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_a_b }, // 0x78
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_a_c }, // 0x79
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_a_d }, // 0x7A
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_a_e }, // 0x7B
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_a_h }, // 0x7C
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_a_l }, // 0x7D
    Opcode { mnemonic: "LD", bytes: 1,     immediate: false, execute: ld_a_hl }, // 0x7E
    Opcode { mnemonic: "LD", bytes: 1,     immediate: true, execute: ld_a_a }, // 0x7F
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_a_b }, // 0x80
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_a_c }, // 0x81
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_a_d }, // 0x82
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_a_e }, // 0x83
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_a_h }, // 0x84
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_a_l }, // 0x85
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: false, execute:add_a_hl}, // 0x86
    Opcode { mnemonic: "ADD", bytes: 1,    immediate: true, execute: add_a_a}, // 0x87
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: adc_a_b }, // 0x88
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: adc_a_c }, // 0x89
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: adc_a_d }, // 0x8A
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: adc_a_e }, // 0x8B
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: adc_a_h }, // 0x8C
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: adc_a_l }, // 0x8D
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: false, execute: adc_a_hl }, // 0x8E
    Opcode { mnemonic: "ADC", bytes: 1,    immediate: true, execute: adc_a_a}, // 0x8F
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: sub_b }, // 0x90
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: sub_c }, // 0x91
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: sub_d }, // 0x92
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: sub_e }, // 0x93
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: sub_h }, // 0x94
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: sub_l }, // 0x95
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: false, execute: sub_hl }, // 0x96
    Opcode { mnemonic: "SUB", bytes: 1,    immediate: true, execute: sub_a }, // 0x97
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: sbc_b }, // 0x98
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: sbc_c }, // 0x99
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: sbc_d }, // 0x9A
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: sbc_e }, // 0x9B
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: sbc_h }, // 0x9C
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: sbc_l }, // 0x9D
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: false, execute: sbc_hl }, // 0x9E
    Opcode { mnemonic: "SBC", bytes: 1,    immediate: true, execute: sbc_a }, // 0x9F
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: and_b }, // 0xA0
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: and_c }, // 0xA1
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: and_d }, // 0xA2
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: and_e }, // 0xA3
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: and_h }, // 0xA4
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: and_l }, // 0xA5
    Opcode { mnemonic: "AND", bytes: 1,    immediate: false, execute: and_hl}, // 0xA6
    Opcode { mnemonic: "AND", bytes: 1,    immediate: true, execute: and_a }, // 0xA7
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: xor_b }, // 0xA8
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: xor_c }, // 0xA9
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: xor_d }, // 0xAA
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: xor_e }, // 0xAB
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: xor_h }, // 0xAC
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: xor_l }, // 0xAD
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: false, execute: xor_hl }, // 0xAE
    Opcode { mnemonic: "XOR", bytes: 1,    immediate: true, execute: xor_a}, // 0xAF
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: or_b }, // 0xB0
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: or_c }, // 0xB1
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: or_d }, // 0xB2
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: or_e }, // 0xB3
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: or_h }, // 0xB4
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: or_l }, // 0xB5
    Opcode { mnemonic: "OR", bytes: 1,     immediate: false, execute: or_hl }, // 0xB6
    Opcode { mnemonic: "OR", bytes: 1,     immediate: true, execute: or_a }, // 0xB7
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: cp_b }, // 0xB8
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: cp_c }, // 0xB9
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: cp_d }, // 0xBA
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: cp_e }, // 0xBB
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: cp_h }, // 0xBC
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: cp_l }, // 0xBD
    Opcode { mnemonic: "CP", bytes: 1,     immediate: false, execute: cp_hl }, // 0xBE
    Opcode { mnemonic: "CP", bytes: 1,     immediate: true, execute: cp_a }, // 0xBF
    Opcode { mnemonic: "RET", bytes: 1,    immediate: true, execute: ret_nz }, // 0xC0
    Opcode { mnemonic: "POP", bytes: 1,    immediate: true, execute: pop_bc }, // 0xC1
    Opcode { mnemonic: "JP", bytes: 3,     immediate: true, execute: jp_nz_a16 }, // 0xC2
    Opcode { mnemonic: "JP", bytes: 3,     immediate: true, execute: jp_a16}, // 0xC3
    Opcode { mnemonic: "CALL", bytes: 3,   immediate: true, execute: call_nz_a16 }, // 0xC4
    Opcode { mnemonic: "PUSH", bytes: 1,   immediate: true, execute: push_bc }, // 0xC5
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

// im pretty sure removing the cpu.set pc+1 here is causing issues when i implment my opcodes
// then I have to add the pc+1 individually/manually to each opcode. 
pub fn fetch_opcode(cpu: &mut Cpu) -> &'static Opcode
{
    let opcode_byte = read_u8_from_pc(cpu);
    //cpu.set_register_16( cpu.regs.PC.wrapping_add(1), "PC");

    if opcode_byte == 0xCB
    {
        let cb_byte = read_u8_from_pc(cpu);
        //cpu.set_register_16( cpu.regs.PC.wrapping_add(1), "PC");

        &CB_OPCODES[cb_byte as usize]
    }
    else  
    {
        &OPCODES[opcode_byte as usize]  
    }
}

//simplfy life
pub fn ld_r_r(cpu: &mut Cpu, dest: char, src: char) -> u8 
{
    let val = cpu.get_register_8(src);
    cpu.set_register_8(val, dest);
    4 
}

pub fn ld_hl_r(cpu: &mut Cpu, src: char) -> u8 
{
    let addr = cpu.get_register_16("HL");
    let value = cpu.get_register_8(src);
    cpu.inter.write_byte(addr, value);
    8 // cycles
}

pub fn add_8bit(cpu: &mut Cpu, a: u8, b: u8) -> u8 
{
    let (result, carry) = a.overflowing_add(b);

    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', false);
    cpu.set_flags('H', ((a & 0x0F) + (b & 0x0F)) > 0x0F);
    cpu.set_flags('C', carry);

    result
}

pub fn adc_8bit(cpu: &mut Cpu, a: u8, b: u8) -> u8 
{
    let carry_in = if cpu.get_flags('C') { 1 } else { 0 };
    let result = a.wrapping_add(b).wrapping_add(carry_in);

    let half_carry = ((a & 0x0F) + (b & 0x0F) + carry_in) > 0x0F;
    let carry_out = (a as u16) + (b as u16) + (carry_in as u16) > 0xFF;

    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', false);
    cpu.set_flags('H', half_carry);
    cpu.set_flags('C', carry_out);

    result

}

pub fn sub_8bit(cpu: &mut Cpu, a: u8, b: u8) -> u8 
{
    let (result, borrow) = a.overflowing_sub(b);

    let half_borrow = (a & 0x0F) < (b & 0x0F);

    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', true);
    cpu.set_flags('H', half_borrow);
    cpu.set_flags('C', borrow);

    result
}

pub fn sbc_8bit(cpu: &mut Cpu, a: u8, b: u8) -> u8 
{
    let carry_in = if cpu.get_flags('C') { 1 } else { 0 };

    // Subtract b and carry from a
    let (intermediate, borrow1) = a.overflowing_sub(b);
    let (result, borrow2) = intermediate.overflowing_sub(carry_in);

    // Half-carry (borrow from bit 4)
    let half_borrow = ((a & 0x0F).wrapping_sub((b & 0x0F) + carry_in)) & 0x10 != 0;

    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', true);
    cpu.set_flags('H', half_borrow);
    cpu.set_flags('C', borrow1 || borrow2);

    result
}

pub fn xor_8bit(cpu: &mut Cpu, a: u8, b:u8) -> u8
{
    let result =  a ^ b;
    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', false);
    cpu.set_flags('H', false);
    cpu.set_flags('C', false);

    result
}

pub fn and_8bit(cpu: &mut Cpu, a: u8, b:u8) -> u8
{
    let result = a & b;
    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', false);
    cpu.set_flags('H', true);
    cpu.set_flags('C', false);

    result
}

pub fn or_8bit(cpu: &mut Cpu, a:u8, b:u8) -> u8
{
    let result =  a | b;
    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', false);
    cpu.set_flags('H', false);
    cpu.set_flags('C', false);

    result
}

pub fn cp_8bit(cpu: &mut Cpu, a:u8, b:u8)
{
    let result = a - b;

    let half_borrow = (a & 0x0F) < (b & 0x0F);
    let carry = a < b;

    cpu.set_flags('Z', result == 0);
    cpu.set_flags('N', true);
    cpu.set_flags('H', half_borrow);
    cpu.set_flags('C', carry);

}

pub fn rst(cpu: &mut Cpu, addr: u16) 
{
    let pc = cpu.get_register_16("PC");
    cpu.push_word(pc);
    cpu.set_register_16(addr, "PC");
}

pub fn read_u16_from_pc(cpu: &mut Cpu) -> u16
{
    let low  = read_u8_from_pc(cpu) as u16;
    let high = read_u8_from_pc(cpu) as u16;

    ((high as u16) << 8) | low
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
    cpu.stop();

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
        let a = cpu.get_register_8('A');
        let result = !a;
        cpu.set_register_8(result, 'A');
    
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
    let val = cpu.inter.read_byte(addr);

    cpu.inter.write_byte(addr, val.wrapping_add(1));

    // Update flags
    cpu.set_flags('Z', val.wrapping_add(1) == 0);
    cpu.set_flags('N', false);
    cpu.set_flags('H', (val & 0x0F) + 1 > 0x0F);

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

//0x36
pub fn ld_hl_d8(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = read_u8_from_pc(cpu);
    cpu.inter.write_byte(hl, val);

    12
}

//0x37
pub fn scf(cpu: &mut Cpu) ->u8
{
    cpu.set_flags('C', true); 
    cpu.set_flags('N', false); 
    cpu.set_flags('H', false); 
    4                          
}

//0x38
pub fn jr_c_r8(cpu: &mut Cpu) -> u8 
{
    let offset = read_u8_from_pc(cpu) as i8; // signed offset
    if cpu.get_flags('C') 
    {
        let pc = cpu.get_register_16("PC");
        cpu.set_register_16( pc.wrapping_add(offset as u16), "PC");
        12 // jump taken
    } 
    else 
    {
        8  // jump not taken
    }
}

//0x39
pub fn add_hl_sp(cpu: &mut Cpu) -> u8
{
    let hl: u16 = cpu.get_register_16("HL");
    let sp: u16 = cpu.get_register_16("SP");

    let hl_add = hl.wrapping_add(sp);

    cpu.set_flags('H', (hl & 0x0fff) + (hl & 0xfff) > 0x0fff);
    cpu.set_flags('C', (hl as u32 + hl as u32) > 0xffff);

    cpu.set_register_16(hl_add , "HL");

    cpu.set_flags('N', false);

    8
}

//0x3A
pub fn ld_a_hl_dec(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'A');
    cpu.set_register_16(hl.wrapping_sub(1), "HL");

    8
}

//0x3B
pub fn dec_sp(cpu: &mut Cpu) -> u8
{
    let mut sp = cpu.get_register_16("SP");

    sp = sp.wrapping_sub(1);
    cpu.set_register_16(sp, "SP");

    8
}

//0x3C
pub fn inc_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');

    cpu.set_flags('H', (a & 0x0f) + 1 > 0x0f);

    cpu.set_register_8(a.wrapping_add(1), 'A');

    cpu.set_flags('Z', a == 0);
    cpu.set_flags('N', false);

    4
}

//0x3D
pub fn dec_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    cpu.set_flags('H', (a & 0x0f) + 1 > 0x0f); 

    cpu.set_register_8(a.wrapping_sub(1), 'A');

    cpu.set_flags('Z', a == 0);
    cpu.set_flags('N', true);

    4
}

//0x3E
pub fn ld_a_d8(cpu: &mut Cpu) -> u8
{
    let pc = cpu.get_register_16("PC").wrapping_add(1);
    cpu.set_register_16(pc, "PC");

    let v = read_u8_from_pc(cpu);
    cpu.set_register_8(v, 'A');
    
    8
}

//0x3F
pub fn ccf(cpu: &mut Cpu) -> u8
{
     let carry = cpu.get_flags('C');
    cpu.set_flags('N', false);
    cpu.set_flags('H', false);
    cpu.set_flags('C', !carry);
    4 
}

//0x40
pub fn ld_b_b(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'B', 'B');
}

//0x41
pub fn ld_b_c(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'B', 'C');
}

//0x42
pub fn ld_b_d(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'B', 'D');
}

//0x43
pub fn ld_b_e(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'B', 'E');
}

//0x44
pub fn ld_b_h(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'B', 'H');
}

//0x45
pub fn ld_b_l(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'B', 'L');
}

//0x46
pub fn ld_b_hl(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'B');

    8
}

//0x47
pub fn ld_b_a(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'B', 'A');
}

//0x48
pub fn ld_c_b(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'C', 'B');
}

//0x49
pub fn ld_c_c(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'C', 'C');
}

//0x4A
pub fn ld_c_d(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'C', 'D');
}

//0x4B
pub fn ld_c_e(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'C', 'E');
}

//0x4C
pub fn ld_c_h(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'C', 'H');
}

//0x4D
pub fn ld_c_l(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'C', 'L');
}

//0x4E
pub fn ld_c_hl(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'C');

    8
}

//0x4F
pub fn ld_c_a(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'C', 'A');
}

//0x50
pub fn ld_d_b(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'D', 'B');
}

//0x51
pub fn ld_d_c(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'D', 'C');
}

//0x52
pub fn ld_d_d(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'D', 'D');
}

//0x53
pub fn ld_d_e(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'D', 'E');
}

//0x54
pub fn ld_d_h(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'D', 'H');
}

//0x55
pub fn ld_d_l(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'D', 'L');
}

//0x56
pub fn ld_d_hl(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'D');

    8
}

//0x57
pub fn ld_d_a(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'D', 'A');
}

//0x58
pub fn ld_e_b(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'E', 'B');
}

//0x59
pub fn ld_e_c(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'E', 'C');
}

//0x5A
pub fn ld_e_d(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'E', 'D');
}

//0x5B
pub fn ld_e_e(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'E', 'E');
}

//0x5C
pub fn ld_e_h(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'E', 'H');
}

//0x5D
pub fn ld_e_l(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'E', 'L');
}

//0x5E
pub fn ld_e_hl(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'E');

    8
}

//0x5F
pub fn ld_e_a(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'E', 'A');
}

//0x60
pub fn ld_h_b(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'H', 'B');
}

//0x61
pub fn ld_h_c(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'H', 'C');
}

//0x62
pub fn ld_h_d(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'H', 'D');
}

//0x63
pub fn ld_h_e(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'H', 'E');
}

//0x64
pub fn ld_h_h(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'H', 'H');
}

//0x65
pub fn ld_h_l(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'H', 'L');
}

//0x66
pub fn ld_h_hl(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    println!("ld_h_hl: HL={:#06X}, read={:#04X}", hl, val);

    cpu.set_register_8(val, 'H');

    8
}

//0x67
pub fn ld_h_a(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'H', 'A');
}

//0x68
pub fn ld_l_b(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'L', 'B');
}

//0x69
pub fn ld_l_c(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'L', 'C');
}

//0x6A
pub fn ld_l_d(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'L', 'D');
}

//0x6B
pub fn ld_l_e(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'L', 'E');
}

//0x6C
pub fn ld_l_h(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'L', 'H');
}

//0x6D
pub fn ld_l_l(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'L', 'L');
}

//0x6E
pub fn ld_l_hl(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'L');

    8
}

//0x6F
pub fn ld_l_a(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'L', 'A');
}

//0x70
pub fn ld_hl_b(cpu: &mut Cpu) -> u8
{
    return ld_hl_r(cpu, 'B');
}

//0x71
pub fn ld_hl_c(cpu: &mut Cpu) -> u8
{
     return ld_hl_r(cpu, 'C');
}

//0x72
pub fn ld_hl_d(cpu: &mut Cpu) -> u8
{
    return ld_hl_r(cpu, 'D');
}

//0x73
pub fn ld_hl_e(cpu: &mut Cpu) -> u8
{
     return ld_hl_r(cpu, 'E');
}

//0x74
pub fn ld_hl_h(cpu: &mut Cpu) -> u8
{
     return ld_hl_r(cpu, 'H');
}

//0x75
pub fn ld_hl_l(cpu: &mut Cpu) -> u8
{
     return ld_hl_r(cpu, 'L');
}

//0x76
pub fn halt(cpu: &mut Cpu) -> u8
{
    cpu.set_halt(true);
    4
}

//0x77
pub fn ld_hl_a(cpu: &mut Cpu) -> u8
{
    return ld_hl_r(cpu, 'A');
}

//0x78
pub fn ld_a_b(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'A', 'B');
}

//0x79
pub fn ld_a_c(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'A', 'C');
}

//0x7A
pub fn ld_a_d(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'A', 'D');
}

//0x7B
pub fn ld_a_e(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'A', 'E');
}

//0x7C
pub fn ld_a_h(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'A', 'H');
}

//0x7D
pub fn ld_a_l(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'A', 'L');
}

//0x7E
pub fn ld_a_hl(cpu: &mut Cpu) -> u8
{
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cpu.set_register_8(val, 'A');

    8
}

//0x7F
pub fn ld_a_a(cpu: &mut Cpu) -> u8
{
    return ld_r_r(cpu, 'A', 'A');
}

//0x80
pub fn add_a_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b = cpu.get_register_8('B');

    let result = add_8bit(cpu, a, b);
    cpu.set_register_8(result, 'A');     

    4 
}

//0x81
pub fn add_a_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c = cpu.get_register_8('C');

    let result = add_8bit(cpu, a, c);
    cpu.set_register_8(result, 'A');     

    4 
}

//0x82
pub fn add_a_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d = cpu.get_register_8('D');

    let result = add_8bit(cpu, a, d);
    cpu.set_register_8(result, 'A');     

    4 
}

//0x83
pub fn add_a_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e = cpu.get_register_8('E');

    let result = add_8bit(cpu, a, e);
    cpu.set_register_8(result, 'A');     

    4 
}

//0x84
pub fn add_a_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h = cpu.get_register_8('H');

    let result = add_8bit(cpu, a, h);
    cpu.set_register_8(result, 'A');     

    4 
}

//0x85
pub fn add_a_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l = cpu.get_register_8('L');

    let result = add_8bit(cpu, a, l);
    cpu.set_register_8(result, 'A');     

    4 
}

//0x86
pub fn add_a_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);
    let result = add_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');
    8 
}

//0x87
pub fn add_a_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');

    let result = add_8bit(cpu, a, a);
    cpu.set_register_8(result, 'A');     

    4 
}


//0x88
pub fn adc_a_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b = cpu.get_register_8('B');

    println!("Carry in = {}", cpu.get_flags('C'));
    let result = adc_8bit(cpu, a, b);
    println!("Carry in = {}", cpu.get_flags('C'));


    cpu.set_register_8(result, 'A');     

    4 
}

//0x89
pub fn adc_a_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c = cpu.get_register_8('C');

    let result = adc_8bit(cpu, a, c);


    cpu.set_register_8(result, 'A');     

    4
}

// 0x8A
pub fn adc_a_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d = cpu.get_register_8('D');

    let result = adc_8bit(cpu, a, d);


    cpu.set_register_8(result, 'A');     

    4
}

//0x8B
pub fn adc_a_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e = cpu.get_register_8('E');

    let result = adc_8bit(cpu, a, e);


    cpu.set_register_8(result, 'A');     

    4
}

//0x8C
pub fn adc_a_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h = cpu.get_register_8('H');

    let result = adc_8bit(cpu, a, h);

    cpu.set_register_8(result, 'A');     

    4
}

//0x8D
pub fn adc_a_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l = cpu.get_register_8('L');

    let result = adc_8bit(cpu, a, l);

    cpu.set_register_8(result, 'A');     

    4
}

//0x8E
pub fn adc_a_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);
    let result = adc_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');
    8 
}

//0x8F
pub fn adc_a_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let result = adc_8bit(cpu, a, a);

    cpu.set_register_8(result, 'A');     

    4
}


//0x90
pub fn sub_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b = cpu.get_register_8('B');

    let result = sub_8bit(cpu, a, b);

    cpu.set_register_8(result, 'A');     

    4
}

//0x91
pub fn sub_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c = cpu.get_register_8('C');

    let result = sub_8bit(cpu, a, c);

    cpu.set_register_8(result, 'A');     

    4
}

//0x92
pub fn sub_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d = cpu.get_register_8('D');

    let result = sub_8bit(cpu, a, d);

    cpu.set_register_8(result, 'A');     

    4
}

//0x93
pub fn sub_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e = cpu.get_register_8('E');

    let result = sub_8bit(cpu, a, e);

    cpu.set_register_8(result, 'A');     

    4
}

//0x94
pub fn sub_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h = cpu.get_register_8('H');

    let result = sub_8bit(cpu, a, h);

    cpu.set_register_8(result, 'A');     

    4
}

//0x95
pub fn sub_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l = cpu.get_register_8('L');

    let result = sub_8bit(cpu, a, l);

    cpu.set_register_8(result, 'A');     

    4
}

//0x96
pub fn sub_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);
    let result = sub_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');
    8 
}

//0x97
pub fn sub_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let result = sub_8bit(cpu, a, a);

    cpu.set_register_8(result, 'A');     

    4
}

//0x98
pub fn sbc_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b  = cpu.get_register_8('B');

    let result = sbc_8bit(cpu, a, b);

    cpu.set_register_8(result, 'A');     

    4
}

//0x99
pub fn sbc_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c  = cpu.get_register_8('C');

    let result = sbc_8bit(cpu, a, c);

    cpu.set_register_8(result, 'A');     

    4
}

//0x9A
pub fn sbc_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d  = cpu.get_register_8('D');

    let result = sbc_8bit(cpu, a, d);

    cpu.set_register_8(result, 'A');     

    4
}

//0x9B
pub fn sbc_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e  = cpu.get_register_8('E');

    let result = sbc_8bit(cpu, a, e);

    cpu.set_register_8(result, 'A');     

    4
}

//0x9C
pub fn sbc_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h  = cpu.get_register_8('H');

    let result = sbc_8bit(cpu, a, h);

    cpu.set_register_8(result, 'A');     

    4
}

//0x9D
pub fn sbc_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l  = cpu.get_register_8('L');

    let result = sbc_8bit(cpu, a, l);

    cpu.set_register_8(result, 'A');     

    4
}

//0x9E
pub fn sbc_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);
    let result = sbc_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');
    8 
}

//0x9F
pub fn sbc_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let result = sbc_8bit(cpu, a, a);

    cpu.set_register_8(result, 'A');     

    4
}

//0xA0
pub fn and_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b = cpu.get_register_8('B');

    let result = and_8bit(cpu, a, b);
    cpu.set_register_8(result, 'A');

    4
}

//0xA1
pub fn and_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c = cpu.get_register_8('C');

    let result = and_8bit(cpu, a, c);
    cpu.set_register_8(result, 'A');

    4
}

//0xA2
pub fn and_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d = cpu.get_register_8('D');

    let result = and_8bit(cpu, a, d);
    cpu.set_register_8(result, 'A');

    4
}

//0xA3
pub fn and_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e = cpu.get_register_8('E');

    let result = and_8bit(cpu, a, e);
    cpu.set_register_8(result, 'A');

    4
}

//0xA4
pub fn and_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h = cpu.get_register_8('H');

    let result = and_8bit(cpu, a, h);
    cpu.set_register_8(result, 'A');

    4
}

//0xA5
pub fn and_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l = cpu.get_register_8('L');

    let result = and_8bit(cpu, a, l);
    cpu.set_register_8(result, 'A');

    4
}

//0xA6
pub fn and_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);
    let result = and_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');

    8 
}

//0xA7
pub fn and_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');

    let result = and_8bit(cpu, a, a);
    cpu.set_register_8(result, 'A');

    4
}

//0xA8
pub fn xor_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b = cpu.get_register_8('B');

    let result = xor_8bit(cpu, a, b);
    cpu.set_register_8(result, 'A');

    4
}

//0xA9
pub fn xor_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c = cpu.get_register_8('C');

    let result = xor_8bit(cpu, a, c);
    cpu.set_register_8(result, 'A');

    4
}

//0xAA
pub fn xor_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d = cpu.get_register_8('D');

    let result = xor_8bit(cpu, a, d);
    cpu.set_register_8(result, 'A');

    4
}

//0xAB
pub fn xor_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e = cpu.get_register_8('E');

    let result = xor_8bit(cpu, a, e);
    cpu.set_register_8(result, 'A');

    4
}

//0xAC
pub fn xor_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h = cpu.get_register_8('H');

    let result = xor_8bit(cpu, a, h);
    cpu.set_register_8(result, 'A');

    4
}

//0xAD
pub fn xor_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l = cpu.get_register_8('L');

    let result = xor_8bit(cpu, a, l);
    cpu.set_register_8(result, 'A');

    4
}

//0xAE
pub fn xor_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    let result = xor_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');

    8
}

//0xAF
pub fn xor_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');

    let result = xor_8bit(cpu, a, a);
    cpu.set_register_8(result, 'A');

    4
}

//0xB0
pub fn or_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b = cpu.get_register_8('B');

    let result = or_8bit(cpu, a, b);
    cpu.set_register_8(result, 'A');

    4
}

//0xB1
pub fn or_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c = cpu.get_register_8('C');

    let result = or_8bit(cpu, a, c);
    cpu.set_register_8(result, 'A');

    4
}

//0xB2
pub fn or_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d = cpu.get_register_8('D');

    let result = or_8bit(cpu, a, d);
    cpu.set_register_8(result, 'A');

    4
}

//0xB3
pub fn or_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e = cpu.get_register_8('E');

    let result = or_8bit(cpu, a, e);
    cpu.set_register_8(result, 'A');

    4
}

//0xB4
pub fn or_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h = cpu.get_register_8('H');

    let result = or_8bit(cpu, a, h);
    cpu.set_register_8(result, 'A');

    4
}

//0xB5
pub fn or_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l = cpu.get_register_8('L');

    let result = or_8bit(cpu, a, l);
    cpu.set_register_8(result, 'A');

    4
}

//0xB6
pub fn or_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    let result = xor_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');

    8
}

//0xB7
pub fn or_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');

    let result = or_8bit(cpu, a, a);
    cpu.set_register_8(result, 'A');

    4
}

//0xB8
pub fn cp_b(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let b = cpu.get_register_8('B');

    cp_8bit(cpu, a, b);

    4
}

//0xB9
pub fn cp_c(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let c = cpu.get_register_8('C');

    cp_8bit(cpu, a, c);

    4
}

//0xBA
pub fn cp_d(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let d = cpu.get_register_8('D');

    cp_8bit(cpu, a, d);

    4
}

//0xBB
pub fn cp_e(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let e = cpu.get_register_8('E');

    cp_8bit(cpu, a, e);

    4
}

//0xBC
pub fn cp_h(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let h = cpu.get_register_8('H');

    cp_8bit(cpu, a, h);

    4
}

//0xBD
pub fn cp_l(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let l = cpu.get_register_8('L');

    cp_8bit(cpu, a, l);

    4
}

//0xBE
pub fn cp_hl(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    let hl = cpu.get_register_16("HL");
    let val = cpu.inter.read_byte(hl);

    cp_8bit(cpu, a, val);

    8
}


//0xBF
pub fn cp_a(cpu: &mut Cpu) -> u8
{
    let a = cpu.get_register_8('A');
    cp_8bit(cpu, a, a);

    4
}

//0xC0
pub fn ret_nz(cpu: &mut Cpu) -> u8 
{
    if !cpu.get_flags('Z') 
    {
        let sp = cpu.get_register_16("SP");
        let low = cpu.inter.read_byte(sp);
        cpu.set_register_16(sp.wrapping_add(1), "SP");

        let sp1 = cpu.get_register_16("SP");
        let high = cpu.inter.read_byte(sp1);
        cpu.set_register_16(sp1.wrapping_add(1), "SP");

        let addr = ((high as u16) << 8) | (low as u16);
        cpu.set_register_16(addr, "PC");

        20 
    } 
    else 
    {
        8 
    }
}

//0xC1
pub fn pop_bc(cpu: &mut Cpu) -> u8
{
    let val = cpu.pop_word();
    cpu.set_8_to_16_conversion(val, "BC");

    12
}

// 0xC2
pub fn jp_nz_a16(cpu: &mut Cpu) -> u8 
{
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let addr = read_u16_from_pc(cpu);

    if !cpu.get_flags('Z') 
    {
        cpu.set_register_16(addr, "PC");
        16
    } 
    else 
    {
        12
    }
}


// 0xC3
pub fn jp_a16(cpu: &mut Cpu) -> u8
{

    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let val = read_u16_from_pc(cpu);

    cpu.set_register_16(val, "PC");

    16
}

//0xC4
pub fn call_nz_a16(cpu: &mut Cpu) -> u8
{
    
    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");


    let addr = read_u16_from_pc(cpu);

    if !cpu.get_flags('Z')
    {
        let pc = cpu.get_register_16("PC");

        cpu.push_word(pc);
        cpu.set_register_16(addr, "PC");

        24
    }
    else 
    {
        12
    }

}

//0xC5
pub fn push_bc(cpu: &mut Cpu) -> u8
{

    let pc = cpu.get_register_16("PC");
    cpu.set_register_16(pc.wrapping_add(1), "PC");

    let bc = cpu.get_8_to_16_conversion("BC");
    cpu.push_word(bc);

    12
}

//0xC6
pub fn add_a_d8(cpu: &mut Cpu) -> u8
{
    let val = read_u8_from_pc(cpu);
    let a = cpu.get_register_8('A');

    let result = add_8bit(cpu, a, val);
    cpu.set_register_8(result, 'A');

    8
}

//0xC7
pub fn rst_00h(cpu: &mut Cpu) -> u8
{
    rst(cpu, 0x0000);
    16
}

//0xC8
pub fn ret_z(cpu: &mut Cpu) -> u8
{
    //let pc = cpu.get_register_16("PC");
    //cpu.set_register_16(pc.wrapping_add(1), "PC");

    if cpu.get_flags('Z')
    {
        let addr = cpu.pop_word();
        cpu.set_register_16(addr, "PC");
        20
    }
    else 
    {
        8
    }
}

//0xC9
pub fn ret(cpu: &mut Cpu) -> u8
{
        let addr = cpu.pop_word();
        cpu.set_register_16(addr, "PC");
        16
}

//0xCA
pub fn jp_z_a16(cpu: &mut Cpu, addr: u16) -> u8 
{
    if cpu.get_flags('Z') 
    {
        cpu.set_register_16(addr, "PC");
        16
    } 
    else 
    {
        12
    }
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

    //0x35
    #[test]
    fn test_dec_hl_zero_result() 
    {
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

    //0x35
    #[test]
    fn test_dec_hl_normal_no_halfcarry() 
    {
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

    //0x36
    #[test]
    fn test_ld_hl_d8() 
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16( 0xC000, "HL");

        // Simulate d8 immediate at PC
        cpu.inter.write_byte(0x0100, 0x7F);
        cpu.set_register_16(0x0100, "PC");

        let cycles = ld_hl_d8(&mut cpu);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.inter.read_byte(0xC000), 0x7F);
        assert_eq!(cpu.get_register_16("PC"), 0x0101);
    }

    //0x37
    #[test]
    fn test_scf()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('Z', true);
        cpu.set_flags('N', true);
        cpu.set_flags('H', true);
        cpu.set_flags('C', false);

        let cycles = scf(&mut cpu);

        assert_eq!(cycles, 4);

        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), true);
    }

    //0x38
    #[test]
    fn test_jr_c_r8_taken() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16( 0x0100, "PC");
        cpu.set_flags('C', true);

        // r8 = 0x05 → jump forward 5 bytes
        cpu.inter.write_byte(0x0100, 0x05);

        let cycles = jr_c_r8(&mut cpu);
        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0106); 
    }

    //0x39
    #[test]
    fn test_add_hl_sp()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x1234, "HL");
        cpu.set_register_16(0x1234, "SP");

        let cycles = add_hl_sp(&mut cpu);

        assert_eq!(cycles, 0x08);
        assert_eq!(cpu.get_register_16("HL"), 0x2468);
    }

    //0x3A
    #[test]
    fn test_ld_a_hl_dec()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0xC000] = 0x7F;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC000, "HL");
        cpu.set_register_8(0x00, 'A');

        let cycles = ld_a_hl_dec(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('A'), 0x7F);
        assert_eq!(cpu.get_register_16("HL"), 0xBFFF);
    }

    //0x3B
    #[test]
    fn test_dec_sp()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x22, "SP");

        let cycles = dec_sp(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("SP"), 0x21);
    }

    //0x3C
    #[test]
    fn test_inc_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x34, 'A');

        let cycles = inc_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('A'), 0x35);
    }

    //0x3D
    #[test]
    fn test_dec_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x25, 'A');

        let cycles = dec_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_register_8('A'), 0x24);
    }

    //0x3E
    #[test]
    fn test_ld_a_d8()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];

        memory[0x0000] = 0x3E;
        memory[0x0001] = 0x23;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0000, "PC");

        let cycles = ld_a_d8(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_16("PC"), 0x0002);
        assert_eq!(cpu.get_register_8('A'), 0x23);
    }

    //0x3F
    #[test]
    fn test_ccf()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        // Case 1: Carry = true → becomes false
        cpu.set_flags('C', true);
        cpu.set_flags('N', true);
        cpu.set_flags('H', true);
        ccf(&mut cpu);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);

        // Case 2: Carry = false → becomes true
        cpu.set_flags('C', false);
        ccf(&mut cpu);
        assert_eq!(cpu.get_flags('C'), true);
    }

    //0x40
    #[test]
    fn test_ld_b_b()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x02, 'B');

        assert_eq!(cpu.get_register_8('B'), 0x02);

        cpu.set_register_8(0x03, 'B');
        let cycles = ld_b_b(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('B'), 0x03);
    }

    //0x41
    #[test]
    fn test_ld_b_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'C');

        let cycles = ld_b_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('B'), 0x03);
    }

    //0x42
    #[test]
    fn test_ld_b_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'D');

        let cycles = ld_b_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('B'), 0x03);
    }

    //0x43
    #[test]
    fn test_ld_b_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'E');

        let cycles = ld_b_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('B'), 0x03);
    }

    //0x44
    #[test]
    fn test_ld_b_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'H');

        let cycles = ld_b_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('B'), 0x03);
    }

    //0x45
    #[test]
    fn test_ld_b_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'L');

        let cycles = ld_b_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('B'), 0x03);
    }

    //0x46
    #[test]
    fn test_ld_b_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x00, 'B');
    
        let cycles = ld_b_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('B'), 0x7F);

    }

    //0x47
    #[test]
    fn test_ld_b_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'A');

        let cycles = ld_b_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('B'), 0x03);

    }

    //0x48
    #[test]
    fn test_ld_c_b()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'B');

        let cycles = ld_c_b(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('C'), 0x03);

    }

    //0x49
    #[test]
    fn test_ld_c_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'C');

        let cycles = ld_c_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('C'), 0x03);

    }

    //0x4A
    #[test]
    fn test_ld_c_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'D');

        let cycles = ld_c_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('C'), 0x03);

    }

    //0x4B
    #[test]
    fn test_ld_c_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'E');

        let cycles = ld_c_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('C'), 0x03);

    }

    //0x4C
    #[test]
    fn test_ld_c_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'H');

        let cycles = ld_c_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('C'), 0x03);

    }

    //0x4D
    #[test]
    fn test_ld_c_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'L');

        let cycles = ld_c_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('C'), 0x03);

    }

    //0x4E
    #[test]
    fn test_ld_c_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x00, 'C');
    
        let cycles = ld_c_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('C'), 0x7F);

    }

    //0x4F
    #[test]
    fn test_ld_c_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'A');

        let cycles = ld_c_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('C'), 0x03);

    }

    //0x50
    #[test]
    fn test_ld_d_b()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'B');

        let cycles = ld_d_b(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('D'), 0x03);
    }

    //0x51
    #[test]
    fn test_ld_d_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'C');

        let cycles = ld_d_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('D'), 0x03);
    }

    //0x52
    #[test]
    fn test_ld_d_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'D');

        let cycles = ld_d_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('D'), 0x03);
    }

    //0x53
    #[test]
    fn test_ld_d_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'E');

        let cycles = ld_d_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('D'), 0x03);
    }

    //0x54
    #[test]
    fn test_ld_d_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'H');

        let cycles = ld_d_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('D'), 0x03);
    }

    //0x55
    #[test]
    fn test_ld_d_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'L');

        let cycles = ld_d_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('D'), 0x03);
    }

    //0x56
    #[test]
    fn test_ld_d_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x00, 'D');
    
        let cycles = ld_d_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('D'), 0x7F);

    }

    //0x57
    #[test]
    fn test_ld_d_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'A');

        let cycles = ld_d_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('D'), 0x03);
    }

    //0x58
    #[test]
    fn test_ld_e_b()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'B');

        let cycles = ld_e_b(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('E'), 0x03);
    }

    //0x59
    #[test]
    fn test_ld_e_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'C');

        let cycles = ld_e_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('E'), 0x03);
    }

    //0x5A
    #[test]
    fn test_ld_e_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'D');

        let cycles = ld_e_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('E'), 0x03);
    }

    //0x5B
    #[test]
    fn test_ld_e_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'E');

        let cycles = ld_e_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('E'), 0x03);
    }

    //0x5C
    #[test]
    fn test_ld_e_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'H');

        let cycles = ld_e_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('E'), 0x03);
    }

    //0x5D
    #[test]
    fn test_ld_e_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'L');

        let cycles = ld_e_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('E'), 0x03);
    }

    //0x5E
    #[test]
    fn test_ld_e_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x00, 'E');
    
        let cycles = ld_e_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('E'), 0x7F);

    }

    //0x5F
    #[test]
    fn test_ld_e_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'A');

        let cycles = ld_e_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('E'), 0x03);
    }

    //0x60
    #[test]
    fn test_ld_h_b()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'B');

        let cycles = ld_h_b(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('H'), 0x03);
    }

    //0x61
    #[test]
    fn test_ld_h_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'C');

        let cycles = ld_h_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('H'), 0x03);
    }

    //0x62
    #[test]
    fn test_ld_h_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'D');

        let cycles = ld_h_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('H'), 0x03);
    }

    //0x63
    #[test]
    fn test_ld_h_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'E');

        let cycles = ld_h_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('H'), 0x03);
    }

    //0x64
    #[test]
    fn test_ld_h_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'H');

        let cycles = ld_h_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('H'), 0x03);
    }

    //0x65
    #[test]
    fn test_ld_h_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'L');

        let cycles = ld_h_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('H'), 0x03);
    }

    //0x66
    #[test]
    fn test_ld_h_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);

        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
    
        let cycles = ld_h_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('H'), 0x7F);
    }

    //0x67
    #[test]
    fn test_ld_h_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'A');

        let cycles = ld_h_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('H'), 0x03);
    }

    //0x68
    #[test]
    fn test_ld_l_b()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'B');

        let cycles = ld_l_b(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('L'), 0x03);
    }

    //0x69
    #[test]
    fn test_ld_l_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'C');

        let cycles = ld_l_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('L'), 0x03);
    }

    //0x6A
    #[test]
    fn test_ld_l_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'D');

        let cycles = ld_l_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('L'), 0x03);
    }

    //0x6B
    #[test]
    fn test_ld_l_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'E');

        let cycles = ld_l_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('L'), 0x03);
    }

    //0x6C
    #[test]
    fn test_ld_l_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'H');

        let cycles = ld_l_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('L'), 0x03);
    }

    //0x6D
    #[test]
    fn test_ld_l_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'L');

        let cycles = ld_l_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('L'), 0x03);
    }

    //0x6E
    #[test]
    fn test_ld_l_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
    
        let cycles = ld_l_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('L'), 0x7F);
    }

    //0x6F
    #[test]
    fn test_ld_l_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'A');

        let cycles = ld_l_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('L'), 0x03);
    } 

    //0x70
    #[test]
    fn test_ld_hl_b() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8( 0x42, 'B');
        cpu.set_register_16(0xC123, "HL");

        let cycles = ld_hl_b(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.inter.read_byte(0xC123), 0x42);
    }

    //0x71
    #[test]
    fn test_ld_hl_c() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8( 0x42, 'C');
        cpu.set_register_16(0xC123, "HL");

        let cycles = ld_hl_c(&mut cpu);

        assert_eq!(cycles, 8);
        assert_eq!(cpu.inter.read_byte(0xC123), 0x42);
    }

    //0x72
    #[test]
    fn test_ld_hl_d() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8( 0x42, 'D');
        cpu.set_register_16(0xC123, "HL");

        let cycles = ld_hl_d(&mut cpu);
        
        assert_eq!(cycles, 8);
        assert_eq!(cpu.inter.read_byte(0xC123), 0x42);
    }

    //0x73
    #[test]
    fn test_ld_hl_e() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8( 0x42, 'E');
        cpu.set_register_16(0xC123, "HL");

        let cycles = ld_hl_e(&mut cpu);
        
        assert_eq!(cycles, 8);
        assert_eq!(cpu.inter.read_byte(0xC123), 0x42);
    }

    //0x74
    #[test]
    fn test_ld_hl_h() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC100, "HL");
        cpu.set_register_8(0x42, 'H');


        let cycles = ld_hl_h(&mut cpu);

        let hl_addr = cpu.get_8_to_16_conversion("HL");
        let mem_val = cpu.inter.read_byte(hl_addr);
        
        assert_eq!(cycles, 8);
        assert_eq!(mem_val, 0x42);
    }

    //0x75
    #[test]
    fn test_ld_hl_l() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xC100, "HL");
        cpu.set_register_8(0x42, 'L');


        let cycles = ld_hl_l(&mut cpu);

        let hl_addr = cpu.get_8_to_16_conversion("HL");
        let mem_val = cpu.inter.read_byte(hl_addr);
        
        assert_eq!(cycles, 8);
        assert_eq!(mem_val, 0x42);
    }

    //0x76
    fn test_halt()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

    }

    //0x77
    #[test]
    fn test_ld_hl_a() 
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8( 0x42, 'A');
        cpu.set_register_16(0xC123, "HL");

        let cycles = ld_hl_a(&mut cpu);
        
        assert_eq!(cycles, 8);
        assert_eq!(cpu.inter.read_byte(0xC123), 0x42);
    }

    //0x78
    #[test]
    fn test_ld_a_b()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'B');

        let cycles = ld_a_b(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x03);
    }

    //0x79
    #[test]
    fn test_ld_a_c()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'C');

        let cycles = ld_a_c(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x03);
    }

    //0x7A
    #[test]
    fn test_ld_a_d()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'D');

        let cycles = ld_a_d(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x03);
    }

    //0x7B
    #[test]
    fn test_ld_a_e()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'E');

        let cycles = ld_a_e(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x03);
    }


    //0x7C
    #[test]
    fn test_ld_a_h()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'H');

        let cycles = ld_a_h(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x03);
    }

    //0x7D
    #[test]
    fn test_ld_a_l()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'L');

        let cycles = ld_a_l(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x03);
    }

    //0x7E
    #[test]
    fn test_ld_a_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x00, 'A');
    
        let cycles = ld_a_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('A'), 0x7F);
    }

    //0x7F
    #[test]
    fn test_ld_a_a()
    {
        let memory: Vec<u8> = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x03, 'A');

        let cycles = ld_a_a(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_register_8('A'), 0x03);
    }

    //0x80
    #[test]
    fn test_add_a_b() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x3C, 'A');
        cpu.set_register_8(0xC1, 'B');

        let cycles = add_a_b(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFD); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }
    
    //0x81
    #[test]
    fn test_add_a_c() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x3C, 'A');
        cpu.set_register_8(0xC1, 'C');

        let cycles = add_a_c(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFD); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x82
    #[test]
    fn test_add_a_d() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x3C, 'A');
        cpu.set_register_8(0xC1, 'D');

        let cycles = add_a_d(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFD); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }
    
    //0x83
    #[test]
    fn test_add_a_e() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x3C, 'A');
        cpu.set_register_8(0xC1, 'E');

        let cycles = add_a_e(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFD); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x84
    #[test]
    fn test_add_a_h() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x3C, 'A');
        cpu.set_register_8(0xC1, 'H');

        let cycles = add_a_h(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFD); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x85
    #[test]
    fn test_add_a_l() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x3C, 'A');
        cpu.set_register_8(0xC1, 'L');

        let cycles = add_a_l(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFD); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x86
    #[test]
    fn test_add_a_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x7F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x00, 'A');
    
        let cycles = add_a_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('A'), 0x7F);
    }

    //0x87
    #[test]
    fn test_add_a_a() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x01, 'A');

        let cycles = add_a_a(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x88
    #[test]
    fn test_adc_a_b() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x8F, 'A');
        cpu.set_register_8(0x91, 'B');

        cpu.set_flags('C',true);

        let cycles = adc_a_b(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x21); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), true);           
        assert_eq!(cycles, 4);
    }

    //0x89
    #[test]
    fn test_adc_a_c() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x8F, 'A');
        cpu.set_register_8(0x91, 'C');

        cpu.set_flags('C',true);

        let cycles = adc_a_c(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x21); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), true);           
        assert_eq!(cycles, 4);
    }

    //0x8A
    #[test]
    fn test_adc_a_d() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x8F, 'A');
        cpu.set_register_8(0x91, 'D');

        cpu.set_flags('C',true);

        let cycles = adc_a_d(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x21); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), true);           
        assert_eq!(cycles, 4);
    }

    //0x8B
    #[test]
    fn test_adc_a_e() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x8F, 'A');
        cpu.set_register_8(0x91, 'E');

        cpu.set_flags('C',true);

        let cycles = adc_a_e(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x21); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), true);           
        assert_eq!(cycles, 4);
    }

    //0x8C
    #[test]
    fn test_adc_a_h() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x8F, 'A');
        cpu.set_register_8(0x91, 'H');

        cpu.set_flags('C',true);

        let cycles = adc_a_h(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x21); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), true);           
        assert_eq!(cycles, 4);
    }

    //0x8D
    #[test]
    fn test_adc_a_l() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x8F, 'A');
        cpu.set_register_8(0x91, 'L');

        cpu.set_flags('C',true);

        let cycles = adc_a_l(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x21); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), true);           
        assert_eq!(cycles, 4);
    }

    #[test]
    fn test_adc_a_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x8F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('C',true);

    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x91, 'A');
    
        let cycles = adc_a_hl(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x21); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), false);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), true);   
        assert_eq!(cycles, 8);
    }

    //0x8F
    #[test]
    fn test_adc_a_a() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x8F, 'A');
        cpu.set_flags('C', false);

        // Execute ADC A, A (adds A + A + carry)
        let cycles = adc_a_a(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x1E); // 0x8F + 0x8F = 0x11E, low byte 0x1E
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), true);
        assert_eq!(cycles, 4);
    }

    //0x90
    #[test]
    fn test_sub_b()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x20, 'A');
        cpu.set_register_8(0x02, 'B');

        let cycles = sub_b(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x1E); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }


    //0x91
    #[test]
    fn test_sub_c()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x20, 'A');
        cpu.set_register_8(0x02, 'C');

        let cycles = sub_c(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x1E); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x92
    #[test]
    fn test_sub_d()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x20, 'A');
        cpu.set_register_8(0x02, 'D');

        let cycles = sub_d(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x1E); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x93
    #[test]
    fn test_sub_e()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x20, 'A');
        cpu.set_register_8(0x02, 'E');

        let cycles = sub_e(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x1E); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x94
    #[test]
    fn test_sub_h()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x20, 'A');
        cpu.set_register_8(0x02, 'H');

        let cycles = sub_h(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x1E); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x95
    #[test]
    fn test_sub_l()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x20, 'A');
        cpu.set_register_8(0x02, 'L');

        let cycles = sub_l(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x1E); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x96
    #[test]
    fn test_sub_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x02; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x20, 'A');
    
        let cycles = sub_hl(&mut cpu);
    
        assert_eq!(cycles, 8);
        assert_eq!(cpu.get_register_8('A'), 0x1E);
    }


    //0x97
    #[test]
    fn test_sub_a()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x20, 'A');

        let cycles = sub_a(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x0); 
        assert_eq!(cpu.get_flags('Z'), true);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), false);            
        assert_eq!(cpu.get_flags('C'), false);           
        assert_eq!(cycles, 4);
    }

    //0x98
    #[test]
    fn test_sbc_b() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x50, 'A'); 
        cpu.set_register_8(0x20, 'B'); 
        cpu.set_flags('C', true);      

        let cycles = sbc_b(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0x99
    #[test]
    fn test_sbc_c() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x50, 'A'); 
        cpu.set_register_8(0x20, 'C'); 
        cpu.set_flags('C', true);      

        let cycles = sbc_c(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0x9A
    #[test]
    fn test_sbc_d() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x50, 'A'); 
        cpu.set_register_8(0x20, 'D'); 
        cpu.set_flags('C', true);      

        let cycles = sbc_d(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0x9B
    #[test]
    fn test_sbc_e() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x50, 'A'); 
        cpu.set_register_8(0x20, 'E'); 
        cpu.set_flags('C', true);      

        let cycles = sbc_e(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0x9C
    #[test]
    fn test_sbc_h() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x50, 'A'); 
        cpu.set_register_8(0x20, 'H'); 
        cpu.set_flags('C', true);      

        let cycles = sbc_h(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0x9D
    #[test]
    fn test_sbc_l() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x50, 'A'); 
        cpu.set_register_8(0x20, 'L'); 
        cpu.set_flags('C', true);      

        let cycles = sbc_l(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0x9E
    #[test]
    fn test_sbc_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x20; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('C',true);

    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x50, 'A');
    
        let cycles = sbc_hl(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x2F); 
        assert_eq!(cpu.get_flags('Z'), false);          
        assert_eq!(cpu.get_flags('N'), true);           
        assert_eq!(cpu.get_flags('H'), true);            
        assert_eq!(cpu.get_flags('C'), false);   
        assert_eq!(cycles, 8);
    }

    //0x9F
    #[test]
    fn test_sbc_a() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x50, 'A'); 
        cpu.set_flags('C', true);      

        let cycles = sbc_a(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), true);
        assert_eq!(cycles, 4);
    }

    //0xA0
    #[test]
    fn test_and_b() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xFF, 'A'); 
        cpu.set_register_8(0xFF, 'B'); 

        let cycles = and_b(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xA1
    #[test]
    fn test_and_c() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xFF, 'A'); 
        cpu.set_register_8(0xFF, 'C'); 

        let cycles = and_c(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xA2
    #[test]
    fn test_and_d() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xFF, 'A'); 
        cpu.set_register_8(0xFF, 'D'); 

        let cycles = and_d(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xA3
    #[test]
    fn test_and_e() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xFF, 'A'); 
        cpu.set_register_8(0xFF, 'E'); 

        let cycles = and_e(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xA4
    #[test]
    fn test_and_h() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xFF, 'A'); 
        cpu.set_register_8(0xFF, 'H'); 

        let cycles = and_h(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xA5
    #[test]
    fn test_and_l() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xFF, 'A'); 
        cpu.set_register_8(0xFF, 'L'); 

        let cycles = and_l(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }


    //0xA6
    #[test]
    fn test_and_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0xFF; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0xFF, 'A');

        let cycles = and_hl(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 8);
    }

    //0xA7
    #[test]
    fn test_and_a() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xFF, 'A'); 

        let cycles = and_a(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), true);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }


    //0xA8
    #[test]
    fn test_xor_b() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A');
        cpu.set_register_8(0x0F, 'B'); 
 

        let cycles = xor_b(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xA9
    #[test]
    fn test_xor_c() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A');
        cpu.set_register_8(0x0F, 'C'); 
 

        let cycles = xor_c(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xAA
    #[test]
    fn test_xor_d() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A');
        cpu.set_register_8(0x0F, 'D'); 
 

        let cycles = xor_d(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xAB
    #[test]
    fn test_xor_e() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A');
        cpu.set_register_8(0x0F, 'E'); 
 

        let cycles = xor_e(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xAC
    #[test]
    fn test_xor_h() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A');
        cpu.set_register_8(0x0F, 'H'); 
 

        let cycles = xor_h(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xAD
    #[test]
    fn test_xor_l() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A');
        cpu.set_register_8(0x0F, 'L'); 
 

        let cycles = xor_l(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xAE
    #[test]
    fn test_xor_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x0F; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x0F, 'A');

        let cycles = xor_hl(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 8);
    }

    //0xAF
    #[test]
    fn test_xor_a() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 

        let cycles = xor_a(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x00); 
        assert_eq!(cpu.get_flags('Z'), true);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB0
    #[test]
    fn test_or_b() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0F, 'B'); 


        let cycles = or_b(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x0F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB1
    #[test]
    fn test_or_c() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0F, 'C'); 


        let cycles = or_c(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x0F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB2
    #[test]
    fn test_or_d() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0F, 'D'); 


        let cycles = or_d(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x0F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB3
    #[test]
    fn test_or_e() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0F, 'E'); 


        let cycles = or_e(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x0F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB4
    #[test]
    fn test_or_h() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0F, 'H'); 


        let cycles = or_h(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x0F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB5
    #[test]
    fn test_or_l() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0xF0, 'A'); 
        cpu.set_register_8(0x0F, 'L'); 


        let cycles = or_l(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB6
    #[test]
    fn test_or_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0xF0; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x0F, 'A');

        let cycles = or_hl(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0xFF); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 8);
    }

    //0xB7
    #[test]
    fn test_or_a() 
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 

        let cycles = or_a(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x0F); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);
    }

    //0xB8
    #[test]
    fn test_cp_b()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0B, 'B'); 


        let cycles = cp_b(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);

    }

    //0xB9
    #[test]
    fn test_cp_c()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0B, 'B'); 


        let cycles = cp_c(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);

    }

    //0xBA
    #[test]
    fn test_cp_d()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0B, 'D'); 


        let cycles = cp_d(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);

    }

    //0xBB
    #[test]
    fn test_cp_e()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0B, 'E'); 


        let cycles = cp_e(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);

    }

    //0xBC
    #[test]
    fn test_cp_h()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0B, 'H'); 


        let cycles = cp_h(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);

    }

    //0xBD
    #[test]
    fn test_cp_l()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 
        cpu.set_register_8(0x0B, 'L'); 


        let cycles = cp_l(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);

    }

    //0xBE
    #[test]
    fn test_cp_hl()
    {
        let mut memory: Vec<u8> = vec![0; 0x10000];
        memory[0xC123] = 0x0B; // value to load
    
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);
    
        cpu.set_register_16(0xC123, "HL");
        cpu.set_register_8(0x0F, 'A');

        let cycles = cp_hl(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 8);
    }

    //0xBF
    #[test]
    fn test_cp_a()
    {
        let memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_8(0x0F, 'A'); 

        let cycles = cp_l(&mut cpu);

        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), true);
        assert_eq!(cpu.get_flags('H'), false);
        assert_eq!(cpu.get_flags('C'), false);
        assert_eq!(cycles, 4);

    }

    //0xC0
    #[test]
    fn test_ret_nz_taken() 
    {
        let mut memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xFFFC, "SP");
        cpu.inter.write_byte(0xFFFC, 0x34);
        cpu.inter.write_byte(0xFFFD, 0x12);

        cpu.set_flags('Z', false); // NZ = true (condition met)

        let cycles = ret_nz(&mut cpu);

        assert_eq!(cpu.get_register_16("PC"), 0x1234);
        assert_eq!(cpu.get_register_16("SP"), 0xFFFE);
        assert_eq!(cycles, 20);
    }

    //0xC0
    #[test]
    fn test_ret_nz_not_taken() 
    {
        let mut memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('Z', true); // NZ = false (condition not met)
        let old_pc = cpu.get_register_16("PC");

        let cycles = ret_nz(&mut cpu);

        assert_eq!(cpu.get_register_16("PC"), old_pc);
        assert_eq!(cycles, 8);
    }

    //0xC1
    #[test]
    fn test_pop_bc() 
    {
        let mut memory = vec![0; 0x10000];
        memory[0xDFFE] = 0x34; 
        memory[0xDFFF] = 0x12; 

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0xDFFE, "SP");

        let cycles = pop_bc(&mut cpu);

        assert_eq!(cpu.get_register_16("BC"), 0x1234);
        assert_eq!(cpu.get_register_16("SP"), 0xE000);
        assert_eq!(cycles, 12);
    }

    //0xC2 
    #[test]
    fn test_jp_nz_a16_taken() {
        let mut memory = vec![0; 0x10000];
        memory[0x0100] = 0xC2;
        memory[0x0101] = 0x34; 
        memory[0x0102] = 0x12; 

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0100, "PC");
        cpu.set_flags('Z', false); 

        let cycles = jp_nz_a16(&mut cpu);

        assert_eq!(cpu.get_register_16("PC"), 0x1234);
        assert_eq!(cycles, 16); // correct GB cycle count when jump is taken
    }

    //0xC2
    #[test]
    fn test_jp_nz_a16_not_taken() {
        let mut memory = vec![0; 0x10000];
        memory[0x0100] = 0xC2;
        memory[0x0101] = 0x34;
        memory[0x0102] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0100, "PC");
        cpu.set_flags('Z', true); // Z → do NOT 


        let cycles = jp_nz_a16(&mut cpu);

        // PC moves past operand bytes: PC = 0x0103
        assert_eq!(cpu.get_register_16("PC"), 0x0103);
        assert_eq!(cycles, 12); // correct GB cycle count when jump is NOT taken
    }

    //0xC3
    #[test]
    fn test_jp_a16()
    {
        let mut memory = vec![0; 0x10000];
        memory[0x0100] = 0xC3;
        memory[0x0101] = 0x34;
        memory[0x0102] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0100, "PC"); 

        let cycles = jp_a16(&mut cpu);

        assert_eq!(cpu.get_register_16("PC"), 0x1234);
        assert_eq!(cycles, 16); 
    }

    //0xC4
    #[test]
    fn test_call_nz_a16_not_taken() 
    {
        let mut memory = vec![0; 0x10000];
        memory[0x0100] = 0xC4; 
        memory[0x0101] = 0x34;
        memory[0x0102] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0100, "PC");
        cpu.set_register_16(0xFFFE, "SP");
        cpu.set_flags('Z', true); 

        let cycles = call_nz_a16(&mut cpu);

        assert_eq!(cpu.get_register_16("PC"), 0x0103);
        assert_eq!(cpu.get_register_16("SP"), 0xFFFE);
        assert_eq!(cycles, 12);
    }

    //0xC4
    #[test]
    fn test_call_nz_a16_taken() 
    {
        let mut memory = vec![0; 0x10000];
        memory[0x0100] = 0xC4; // CALL NZ,a16
        memory[0x0101] = 0x34;
        memory[0x0102] = 0x12;

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0100, "PC");
        cpu.set_register_16(0xFFFE, "SP");
        cpu.set_flags('Z', false); 

        let cycles = call_nz_a16(&mut cpu);

        assert_eq!(cpu.get_register_16("SP"), 0xFFFC);

        let low  = cpu.inter.read_byte(0xFFFC);
        let high = cpu.inter.read_byte(0xFFFD);
        let ret  = ((high as u16) << 8) | low as u16;

        assert_eq!(ret, 0x0103); 

        assert_eq!(cpu.get_register_16("PC"), 0x1234);

        assert_eq!(cycles, 24);
    }

    //0xC5
    #[test]
    fn test_push_bc() 
    {
        let interconnect = Interconnect::new(vec![0; 0x10000]);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x1234, "BC"); 
        cpu.set_register_16(0xFFFE, "SP");  

        let cycles = push_bc(&mut cpu);

        assert_eq!(cpu.get_register_16("SP"), 0xFFFC);

        let low  = cpu.inter.read_byte(0xFFFC);
        let high = cpu.inter.read_byte(0xFFFD);
        let pushed = ((high as u16) << 8) | low as u16;
        assert_eq!(pushed, 0x1234);

        assert_eq!(cycles, 12)
    }

    //0xC6
    #[test]
    fn test_add_a_d8() 
    {
        let mut memory = vec![0; 0x10000];
        memory[0x0100] = 0x3C; 

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0100, "PC");
        cpu.set_register_8(0x20, 'A'); 
        cpu.set_flags('C', false);

        let cycles = add_a_d8(&mut cpu);

        assert_eq!(cpu.get_register_8('A'), 0x5C); 
        assert_eq!(cpu.get_flags('Z'), false);
        assert_eq!(cpu.get_flags('N'), false);
        assert_eq!(cpu.get_flags('H'), false);  
        assert_eq!(cpu.get_flags('C'), false);


        assert_eq!(cycles, 8); 
    }

    //0xC7
    #[test]
    fn test_rst_00h()
    {
        let mut memory = vec![0; 0x10000];
        memory[0x0100] = 0xC7; 

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_register_16(0x0100, "PC");
        cpu.set_register_16(0xFFFE, "SP");

        let cycles = rst_00h(&mut cpu);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.get_register_16("PC"), 0x0000)
    }

    //0xC8
    #[test]
    fn test_ret_z_taken() {
        let mut memory = vec![0; 0x10000];

        memory[0xFFFC] = 0x34; 
        memory[0xFFFD] = 0x12;  

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('Z', true);

        cpu.set_register_16(0xFFFC, "SP");

        cpu.set_register_16(0x0100, "PC");

        let cycles = ret_z(&mut cpu);

        assert_eq!(cycles, 20);
        assert_eq!(cpu.get_register_16("PC"), 0x1234);
        assert_eq!(cpu.get_register_16("SP"), 0xFFFE); 
    }

    //0xC9
    #[test]
    fn test_ret()
    {
        let mut memory = vec![0; 0x10000];

        memory[0xFFFC] = 0x34; 
        memory[0xFFFD] = 0x12;  

        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('Z', true);

        cpu.set_register_16(0xFFFC, "SP");

        cpu.set_register_16(0x0100, "PC");

        let cycles = ret_z(&mut cpu);

        assert_eq!(cycles, 20);
        assert_eq!(cpu.get_register_16("PC"), 0x1234);
        assert_eq!(cpu.get_register_16("SP"), 0xFFFE); 
    }

    //0xCA
    #[test]
    fn test_jp_z_a16_taken() 
    {
        let mut memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('Z', true);
        cpu.set_register_16(0x0100, "PC");

        let cycles = jp_z_a16(&mut cpu, 0x1234);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.get_register_16("PC"), 0x1234);
    }

    #[test]
    fn test_jp_z_not_taken() 
    {
        let mut memory = vec![0; 0x10000];
        let interconnect = Interconnect::new(memory);
        let mut cpu = Cpu::new(interconnect);

        cpu.set_flags('Z', false);
        cpu.set_register_16(0x0100, "PC");

        let cycles = jp_z_a16(&mut cpu, 0x1234);

        assert_eq!(cycles, 12);
        assert_eq!(cpu.get_register_16("PC"), 0x0100); 
    }
        


}