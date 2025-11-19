use crate::cpu::{self, Cpu};

pub struct Alu;

impl Alu
{
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
}