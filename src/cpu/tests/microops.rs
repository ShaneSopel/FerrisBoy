use crate::cpu::microops::MicroOp;
use crate::cpu::{Cpu, Reg16, Reg8};
use crate::interconnect::Interconnect;

fn setup_cpu() -> Cpu {
    let memory = vec![0; 0x100];
    let inter = Interconnect::new(memory);
    Cpu::new(inter)
}

#[test]
fn ld_reg8_from_reg8() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::B, 0x42);

    cpu.execute_microop(MicroOp::LdReg8FromReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x42);
}

#[test]
fn ld_reg8_from_mem() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x99);

    cpu.execute_microop(MicroOp::LdReg8FromMem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x99);
}

#[test]
fn ld_reg8_from_imm() {
    let mut cpu = setup_cpu();
    let imm = 0x42;

    cpu.inter.write_byte(0x0000, imm);
    cpu.regs.pc = 0x0000;
    cpu.regs.set8(Reg8::B, 0x00);

    cpu.execute_microop(MicroOp::LdReg8FromImm { dst: (Reg8::B) });

    assert_eq!(cpu.regs.get8(Reg8::B), 0x42);
}

#[test]
fn ld_reg8_from_mem_inc_hl() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x99);

    cpu.execute_microop(MicroOp::LdReg8FromMemIncHL { dst: (Reg8::A) });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x99);
    assert_eq!(cpu.regs.get16(Reg16::HL), 0x8001);
}

#[test]
fn ld_mem_from_reg8_inc_hl() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.regs.set8(Reg8::A, 0x99);

    cpu.execute_microop(MicroOp::LdMemFromReg8IncHL { src: (Reg8::A) });

    assert_eq!(cpu.inter.read_byte(0x8000), 0x99);
    assert_eq!(cpu.regs.get16(Reg16::HL), 0x8001);
}

#[test]
fn ld_reg8_from_mem_dec_hl() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x99);

    cpu.execute_microop(MicroOp::LdReg8FromMemDecHL { dst: (Reg8::A) });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x99);
    assert_eq!(cpu.regs.get16(Reg16::HL), 0x7FFF);
}

#[test]
fn ld_mem_from_reg8_dec_hl() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.regs.set8(Reg8::A, 0x99);

    cpu.execute_microop(MicroOp::LdMemFromReg8DecHL { src: (Reg8::A) });

    assert_eq!(cpu.inter.read_byte(0x8000), 0x99);
    assert_eq!(cpu.regs.get16(Reg16::HL), 0x7FFF);
}

#[test]
fn ld_mem_from_reg8() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.regs.set8(Reg8::A, 0x99);

    cpu.execute_microop(MicroOp::LdMemFromReg8 {
        addr: (Reg16::HL),
        src: (Reg8::A),
    });

    assert_eq!(cpu.inter.read_byte(0x8000), 0x99);
    assert_eq!(cpu.regs.get16(Reg16::HL), 0x8000);
}

#[test]
fn ld_c_from_a() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x99);
    cpu.regs.set8(Reg8::C, 0x10);

    cpu.execute_microop(MicroOp::LdCFromA);

    let addr = 0xFF00u16 + 0x10;

    assert_eq!(cpu.inter.read_byte(addr), 0x99);
    assert_eq!(cpu.regs.get8(Reg8::A), 0x99);
}

#[test]
fn ld_a_from_c() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::C, 0x10);
    let addr = 0xFF00u16 + 0x10;
    cpu.inter.write_byte(addr, 0x81);
    cpu.execute_microop(MicroOp::LdAFromC);

    assert_eq!(cpu.regs.get8(Reg8::A), 0x81);
}

#[test]
fn ld_mem_from_a() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x99);

    cpu.execute_microop(MicroOp::LdMemFromA { addr: (0x8000) });

    assert_eq!(cpu.inter.read_byte(0x8000), 0x99);
}

#[test]
fn ld_reg16_from_mem() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x99);

    cpu.execute_microop(MicroOp::LdReg16FromMem {
        dst: Reg16::BC,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get16(Reg16::BC), 0x99);
}

#[test]
fn ld_mem_imm16_from_reg16() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0xFEED);

    cpu.regs.pc = 0x0000;
    cpu.inter.write_byte(0x0000, 0x00);
    cpu.inter.write_byte(0x0001, 0xC0);

    cpu.execute_microop(MicroOp::LdMemImm16FromReg16 { src: Reg16::HL });

    assert_eq!(cpu.inter.read_byte(0xC000), 0xED);
    assert_eq!(cpu.inter.read_byte(0xC001), 0xFE);

    assert_eq!(cpu.regs.pc, 0x0002);
}

#[test]
fn ld_reg8_from_reg16() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x99);

    cpu.execute_microop(MicroOp::LdReg8FromReg16 {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x99);
}

#[test]
fn ld_mem_from_imm8() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0xC000);
    cpu.regs.pc = 0x0000;

    cpu.inter.write_byte(0x0000, 0x99);

    cpu.execute_microop(MicroOp::LdMemFromImm8 { addr: Reg16::HL });

    assert_eq!(cpu.inter.read_byte(0xC000), 0x99);
    assert_eq!(cpu.regs.get16(Reg16::HL), 0xC000);
    assert_eq!(cpu.regs.pc, 0x0001);
}

#[test]
fn inc_reg8() {
    let mut cpu = setup_cpu();
    cpu.regs.set8(Reg8::A, 0x80);

    cpu.execute_microop(MicroOp::IncReg8 { reg: (Reg8::A) });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x81);
}

#[test]
fn dec_reg8() {
    let mut cpu = setup_cpu();
    cpu.regs.set8(Reg8::A, 0x80);

    cpu.execute_microop(MicroOp::DecReg8 { reg: (Reg8::A) });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x7F);
}

#[test]
fn inc_reg16() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x9A);

    cpu.execute_microop(MicroOp::IncReg16 { reg: (Reg16::HL) });

    assert_eq!(cpu.regs.get16(Reg16::HL), 0x8001);
}

#[test]
fn dec_reg16() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x9A);

    cpu.execute_microop(MicroOp::DecReg16 { reg: (Reg16::HL) });

    assert_eq!(cpu.regs.get16(Reg16::HL), 0x7FFF);
}

#[test]
fn add_reg8() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.regs.set8(Reg8::B, 0x03);

    cpu.execute_microop(MicroOp::AddReg8 {
        dst: (Reg8::A),
        src: (Reg8::B),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x04);
}

#[test]
fn add_reg8_mem() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.regs.set16(Reg16::HL, 0x8000);

    cpu.inter.write_byte(0x8000, 0x9A);

    cpu.execute_microop(MicroOp::AddReg8Mem {
        dst: (Reg8::A),
        src: (Reg16::HL),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x9B);
}

#[test]
fn add_reg8_imm() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.execute_microop(MicroOp::AddReg8Imm {
        dst: (Reg8::A),
        addr: (0x80),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x81);
}

#[test]
fn add_reg16() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0x0001);
    cpu.regs.set16(Reg16::BC, 0x0003);

    cpu.execute_microop(MicroOp::AddReg16 {
        dst: (Reg16::HL),
        src: (Reg16::BC),
    });

    assert_eq!(cpu.regs.get16(Reg16::HL), 0x0004);
}

#[test]
fn add_carry8_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.regs.set8(Reg8::B, 0x02);
    cpu.flags.c = false;

    cpu.execute_microop(MicroOp::AddCarry8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x03);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
}

#[test]
fn add_carry8_with_carry_in() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0xFF);
    cpu.regs.set8(Reg8::B, 0x00);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::AddCarry8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
}

#[test]
fn add_carry8_half_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x0F);
    cpu.regs.set8(Reg8::B, 0x00);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::AddCarry8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x10);
    assert!(cpu.flags.h);
}

#[test]
fn add_carry8_mem_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x02);

    cpu.flags.c = false;

    cpu.execute_microop(MicroOp::AddCarry8Mem {
        dst: (Reg8::A),
        src: (Reg16::HL),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x03);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
}

#[test]
fn add_carry8_mem_with_carry_in() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0xFF);
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x00);

    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::AddCarry8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
}

#[test]
fn add_carry8_mem_half_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x0F);
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x00);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::AddCarry8Mem {
        dst: (Reg8::A),
        src: (Reg16::HL),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x10);
    assert!(cpu.flags.h);
}

#[test]
fn add_carry8_imm_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.flags.c = false;

    cpu.execute_microop(MicroOp::AddCarry8Imm {
        dst: (Reg8::A),
        addr: (0x02),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x03);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
}

#[test]
fn add_carry8_imm_with_carry_in() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0xFF);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::AddCarry8Imm {
        dst: (Reg8::A),
        addr: (0x00),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(cpu.flags.c);
    assert!(cpu.flags.z);
}

#[test]
fn add_carry8_imm_half_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x0F);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::AddCarry8Imm {
        dst: (Reg8::A),
        addr: (0x00),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x10);
    assert!(cpu.flags.h);
}

#[test]
fn sub_reg8() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x05);
    cpu.regs.set8(Reg8::B, 0x03);

    cpu.execute_microop(MicroOp::SubReg8 {
        dst: (Reg8::A),
        src: (Reg8::B),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x02);
}

#[test]
fn sub_carry8_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x03);
    cpu.regs.set8(Reg8::B, 0x01);
    cpu.flags.c = false;

    cpu.execute_microop(MicroOp::SubCarry8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x02);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn sub_carry8_with_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.regs.set8(Reg8::B, 0x00);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::SubCarry8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
}

#[test]
fn sub_carry8_half_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x10);
    cpu.regs.set8(Reg8::B, 0x00);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::SubCarry8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x0F);
    assert!(cpu.flags.h);
}

#[test]
fn sub_carry8_mem_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x03);
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x01);
    cpu.flags.c = false;

    cpu.execute_microop(MicroOp::SubCarry8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x02);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn sub_carry8_mem_with_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x00);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::SubCarry8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
}

#[test]
fn sub_carry8_mem_half_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x10);
    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x00);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::SubCarry8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x0F);
    assert!(cpu.flags.h);
}

#[test]
fn sub_carry8_imm_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x03);
    cpu.flags.c = false;

    cpu.execute_microop(MicroOp::SubCarry8Imm {
        dst: (Reg8::A),
        addr: (0x01),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x02);
    assert!(!cpu.flags.c);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
}

#[test]
fn sub_carry8_imm_with_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x01);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::SubCarry8Imm {
        dst: (Reg8::A),
        addr: (0x00),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(!cpu.flags.c);
    assert!(cpu.flags.z);
}

#[test]
fn sub_carry8_imm_half_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x10);
    cpu.flags.c = true;

    cpu.execute_microop(MicroOp::SubCarry8Imm {
        dst: (Reg8::A),
        addr: (0x00),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x0F);
    assert!(cpu.flags.h);
}

#[test]
fn xor_reg8_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_1010);
    cpu.regs.set8(Reg8::B, 0b1100_1100);

    cpu.execute_microop(MicroOp::XorReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0110_0110);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn xor_reg8_result_zero() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_1010);

    cpu.execute_microop(MicroOp::XorReg8 {
        dst: Reg8::A,
        src: Reg8::A,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn xor_reg8_mem_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_1010);

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0b1100_1100);

    cpu.execute_microop(MicroOp::XorReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0110_0110);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn xor_reg8_mem_result_zero() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_1010);

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0b1010_1010);

    cpu.execute_microop(MicroOp::XorReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn xor_reg8_imm_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_1010);

    cpu.execute_microop(MicroOp::XorReg8Imm {
        dst: (Reg8::A),
        addr: (0b1100_1100),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0110_0110);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn xor_reg8_imm_result_zero() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_1010);

    cpu.execute_microop(MicroOp::XorReg8Imm {
        dst: (Reg8::A),
        addr: (0b1010_1010),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn cp_reg8_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x3C);
    cpu.regs.set8(Reg8::B, 0x2F);

    cpu.execute_microop(MicroOp::CpReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x3C);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn cp_reg8_zero_and_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x10);
    cpu.regs.set8(Reg8::B, 0x20);

    cpu.execute_microop(MicroOp::CpReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x10);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    assert!(cpu.flags.c);
}

#[test]
fn cp_reg8_mem_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x3C);

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x2F);

    cpu.execute_microop(MicroOp::CpReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x3C);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn cp_reg8_mem_zero_and_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x10);

    cpu.regs.set16(Reg16::HL, 0x8000);
    cpu.inter.write_byte(0x8000, 0x20);

    cpu.execute_microop(MicroOp::CpReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x10);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    assert!(cpu.flags.c);
}

#[test]
fn cp_reg8_imm_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x3C);
    cpu.execute_microop(MicroOp::CpReg8Imm {
        dst: (Reg8::A),
        addr: (0x2F),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x3C);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn cp_reg8_imm_zero_and_carry() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x10);
    cpu.execute_microop(MicroOp::CpReg8Imm {
        dst: (Reg8::A),
        addr: (0x20),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x10);
    assert!(!cpu.flags.z);
    assert!(cpu.flags.n);
    assert!(cpu.flags.c);
}

#[test]
fn or_reg8_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_0001);
    cpu.regs.set8(Reg8::B, 0b0101_0010);

    cpu.execute_microop(MicroOp::OrReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b1111_0011);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn or_reg8_zero_flag() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x00);
    cpu.regs.set8(Reg8::B, 0x00);

    cpu.execute_microop(MicroOp::OrReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn or_reg8_mem_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_0001);
    cpu.regs.set16(Reg16::HL, 0x8000);

    cpu.inter.write_byte(0x8000, 0b0101_0010);

    cpu.execute_microop(MicroOp::OrReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b1111_0011);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn or_reg8_mem_zero_flag() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x00);
    cpu.regs.set16(Reg16::HL, 0x8000);

    cpu.inter.write_byte(0x8000, 0x00);

    cpu.execute_microop(MicroOp::OrReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn or_reg8_imm_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1010_0001);

    cpu.execute_microop(MicroOp::OrReg8Imm {
        dst: Reg8::A,
        addr: 0b0101_0010,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b1111_0011);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn or_reg8_imm_zero_flag() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0x00);

    cpu.execute_microop(MicroOp::OrReg8Imm {
        dst: (Reg8::A),
        addr: (0x00),
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0x00);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(!cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn and_reg8_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1100_1010);
    cpu.regs.set8(Reg8::B, 0b1010_1111);

    cpu.execute_microop(MicroOp::AndReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b1000_1010);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn and_reg8_zero_flag() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b0000_1010);
    cpu.regs.set8(Reg8::B, 0b0000_0101);

    cpu.execute_microop(MicroOp::AndReg8 {
        dst: Reg8::A,
        src: Reg8::B,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0000_0000);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn and_reg8_mem_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1100_1010);
    cpu.regs.set16(Reg16::HL, 0x8000);

    cpu.inter.write_byte(0x8000, 0b1010_1111);

    cpu.execute_microop(MicroOp::AndReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b1000_1010);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn and_reg8_mem_zero_flag() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b0000_1010);
    cpu.regs.set16(Reg16::HL, 0x8000);

    cpu.inter.write_byte(0x8000, 0b0000_0101);

    cpu.execute_microop(MicroOp::AndReg8Mem {
        dst: Reg8::A,
        src: Reg16::HL,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0000_0000);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn and_reg8_imm_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1100_1010);

    cpu.execute_microop(MicroOp::AndReg8Imm {
        dst: Reg8::A,
        addr: 0b1010_1111,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b1000_1010);
    assert!(!cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn and_reg8_imm_zero_flag() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b0000_1010);

    cpu.execute_microop(MicroOp::AndReg8Imm {
        dst: Reg8::A,
        addr: 0b0000_0101,
    });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0000_0000);
    assert!(cpu.flags.z);
    assert!(!cpu.flags.n);
    assert!(cpu.flags.h);
    assert!(!cpu.flags.c);
}

#[test]
fn push_reg16_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::HL, 0xABCD);
    cpu.regs.set16(Reg16::SP, 0xFFFE);

    cpu.execute_microop(MicroOp::PushReg16 { reg: Reg16::HL });

    assert_eq!(cpu.regs.get16(Reg16::SP), 0xFFFC);
    assert_eq!(cpu.inter.read_byte(0xFFFC), 0xAB);
    assert_eq!(cpu.inter.read_byte(0xFFFD), 0xCD);
}

#[test]
fn pop_reg16_basic() {
    let mut cpu = setup_cpu();

    cpu.regs.set16(Reg16::SP, 0xFFFC);
    cpu.inter.write_byte(0xFFFC, 0x34);
    cpu.inter.write_byte(0xFFFD, 0x12);

    cpu.execute_microop(MicroOp::PopReg16 { reg: Reg16::HL });
    assert_eq!(cpu.regs.get16(Reg16::HL), 0x1234);
    assert_eq!(cpu.regs.get16(Reg16::SP), 0xFFFE);
}

#[test]
fn jump_absolute_sets_pc() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x0000);

    cpu.execute_microop(MicroOp::JumpAbsolute { addr: 0x1234 });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1234);
}

#[test]
fn jump_absolute_if_takes_jump_when_flag_matches() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x0000);
    cpu.flags.set_flag('z', true);

    cpu.execute_microop(MicroOp::JumpAbsoluteIf {
        addr: 0x2000,
        flag: 'z',
        expected: true,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x2000);
}

#[test]
fn jump_absolute_if_does_not_jump_when_flag_mismatches() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x0000);
    cpu.flags.set_flag('z', false);

    cpu.execute_microop(MicroOp::JumpAbsoluteIf {
        addr: 0x2000,
        flag: 'z',
        expected: true,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x0000);
}

#[test]
fn jump_relative_adds_offset() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x1000);

    cpu.execute_microop(MicroOp::JumpRelative { offset: 0x10 });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1010);

    cpu.execute_microop(MicroOp::JumpRelative { offset: -0x10 });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1000);
}

#[test]
fn jump_relative_if_conditional() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x1000);
    cpu.flags.set_flag('z', true);

    cpu.execute_microop(MicroOp::JumpRelativeIf {
        offset: 0x20,
        flag: 'z',
        expected: true,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1020);

    cpu.execute_microop(MicroOp::JumpRelativeIf {
        offset: 0x10,
        flag: 'z',
        expected: false,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1020);
}

#[test]
fn jump_hl_sets_pc() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::HL, 0x1234);

    cpu.execute_microop(MicroOp::JumpHL);
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1234);
}

#[test]
fn call_absolute_pushes_pc_and_jumps() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x1000);
    cpu.regs.set16(Reg16::SP, 0xFFFE);

    cpu.execute_microop(MicroOp::CallAbsolute { addr: 0x2000 });

    assert_eq!(cpu.regs.get16(Reg16::PC), 0x2000);

    let sp = cpu.regs.get16(Reg16::SP);
    assert_eq!(cpu.inter.read_byte(sp), 0x00); 
    assert_eq!(cpu.inter.read_byte(sp + 1), 0x10); 
}

#[test]
fn call_absolute_if_conditional_jumps_only_if_flag_matches() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x1000);
    cpu.regs.set16(Reg16::SP, 0xFFFE);

    cpu.flags.set_flag('z', true);
    cpu.execute_microop(MicroOp::CallAbsoluteIf {
        addr: 0x2000,
        flag: 'z',
        expected: true,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x2000);

    cpu.regs.set16(Reg16::PC, 0x1000);
    cpu.flags.set_flag('z', false);
    cpu.execute_microop(MicroOp::CallAbsoluteIf {
        addr: 0x3000,
        flag: 'z',
        expected: true,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1000);
}

#[test]
fn return_sets_pc_from_stack() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::SP, 0xFFFC);

    cpu.inter.write_byte(0xFFFC, 0x34);
    cpu.inter.write_byte(0xFFFD, 0x12); 

    cpu.execute_microop(MicroOp::Return);

    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1234);
    assert_eq!(cpu.regs.get16(Reg16::SP), 0xFFFE);
}

#[test]
fn return_if_conditional_only_pops_if_flag_matches() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::SP, 0xFFFC);

    cpu.inter.write_byte(0xFFFC, 0x34); 
    cpu.inter.write_byte(0xFFFD, 0x12); 

    cpu.flags.set_flag('z', true);
    cpu.execute_microop(MicroOp::ReturnIf {
        flag: 'z',
        expected: true,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1234);

    cpu.regs.set16(Reg16::PC, 0x0000);
    cpu.flags.set_flag('z', false);
    cpu.execute_microop(MicroOp::ReturnIf {
        flag: 'z',
        expected: true,
    });
    assert_eq!(cpu.regs.get16(Reg16::PC), 0x0000);
}

#[test]
fn reti_behaves_like_return_and_sets_ime() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::SP, 0xFFFC);

    cpu.inter.write_byte(0xFFFC, 0x34); 
    cpu.inter.write_byte(0xFFFD, 0x12); 

    cpu.execute_microop(MicroOp::Reti);

    assert_eq!(cpu.regs.get16(Reg16::PC), 0x1234);
    assert!(cpu.interrupt);
}

#[test]
fn restart_pushes_pc_and_jumps_to_vector() {
    let mut cpu = setup_cpu();
    cpu.regs.set16(Reg16::PC, 0x1000);
    cpu.regs.set16(Reg16::SP, 0xFFFE);

    cpu.execute_microop(MicroOp::Restart { vector: 0x08 });

    assert_eq!(cpu.regs.get16(Reg16::PC), 0x0008);

    let sp = cpu.regs.get16(Reg16::SP);
    assert_eq!(cpu.inter.read_byte(sp), 0x00); 
    assert_eq!(cpu.inter.read_byte(sp + 1), 0x10); 
}


#[test]
fn rlc_reg8() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1000_0001);

    cpu.execute_microop(MicroOp::RlcReg8 { dst: Reg8::A });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0000_0011);
    assert!(cpu.flags.c);
}
