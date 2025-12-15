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
fn rlc_reg8() {
    let mut cpu = setup_cpu();

    cpu.regs.set8(Reg8::A, 0b1000_0001);

    cpu.execute_microop(MicroOp::RlcReg8 { dst: Reg8::A });

    assert_eq!(cpu.regs.get8(Reg8::A), 0b0000_0011);
    assert!(cpu.flags.c);
}
