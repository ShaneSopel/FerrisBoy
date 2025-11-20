use crate::cpu::{ Cpu};

pub struct Alu;

impl Alu
{
    pub fn new() -> Self
    {
        let alu = Self
        {

        };

        alu
    }

    pub fn add_8bit(&self, cpu: &mut Cpu, a: u8, b: u8) -> u8 
    {
        let (result, carry) = a.overflowing_add(b);

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', false);
        cpu.flags.set_flag('H', ((a & 0x0F) + (b & 0x0F)) > 0x0F);
        cpu.flags.set_flag('C', carry);

        result
    }

    pub fn add_16bit(&self, cpu: &mut Cpu, a: u16, b: u16) -> u16
    {
        let result = a.wrapping_add(b);

        let h = (((a & 0x0FFF) + (b & 0x0FFF)) & 0x1000) != 0;
        let c = (a as u32 + b as u32) > 0xFFFF;
    
        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', false);
        cpu.flags.set_flag('H', h);
        cpu.flags.set_flag('C', c);

        result
    }

    pub fn adc_8bit(&self, cpu: &mut Cpu, a: u8, b: u8) -> u8 
    {
        let carry_in = if cpu.flags.get_flag('C') { 1 } else { 0 };
        let result = a.wrapping_add(b).wrapping_add(carry_in);

        let half_carry = ((a & 0x0F) + (b & 0x0F) + carry_in) > 0x0F;
        let carry_out = (a as u16) + (b as u16) + (carry_in as u16) > 0xFF;

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', false);
        cpu.flags.set_flag('H', half_carry);
        cpu.flags.set_flag('C', carry_out);

        result

    }

    pub fn sub_8bit(&self, cpu: &mut Cpu, a: u8, b: u8) -> u8 
    {
        let (result, borrow) = a.overflowing_sub(b);

        let half_borrow = (a & 0x0F) < (b & 0x0F);

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', false);
        cpu.flags.set_flag('H', half_borrow);
        cpu.flags.set_flag('C', borrow);

        result
    }

    pub fn sbc_8bit(&self, cpu: &mut Cpu, a: u8, b: u8) -> u8 
    {
        let carry_in = if cpu.get_flags('C') { 1 } else { 0 };

        // Subtract b and carry from a
        let (intermediate, borrow1) = a.overflowing_sub(b);
        let (result, borrow2) = intermediate.overflowing_sub(carry_in);

        // Half-carry (borrow from bit 4)
        let half_borrow = ((a & 0x0F).wrapping_sub((b & 0x0F) + carry_in)) & 0x10 != 0;

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', true);
        cpu.flags.set_flag('H', half_borrow);
        cpu.flags.set_flag('C', borrow1 || borrow2);

        result
    }

    pub fn xor_8bit(&self, cpu: &mut Cpu, a: u8, b:u8) -> u8
    {
        let result =  a ^ b;

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', false);
        cpu.flags.set_flag('H', false);
        cpu.flags.set_flag('C', false);

        result
    }

    pub fn and_8bit(&self, cpu: &mut Cpu, a: u8, b:u8) -> u8
    {
        let result = a & b;

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', false);
        cpu.flags.set_flag('H', true);
        cpu.flags.set_flag('C', false);

        result
    }

    pub fn or_8bit(&self, cpu: &mut Cpu, a:u8, b:u8) -> u8
    {
        let result =  a | b;

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', false);
        cpu.flags.set_flag('H', false);
        cpu.flags.set_flag('C', false);

        result
    }

    pub fn cp_8bit(&self, cpu: &mut Cpu, a:u8, b:u8)
    {
        let result = a - b;

        let half_borrow = (a & 0x0F) < (b & 0x0F);
        let carry = a < b;

        cpu.flags.set_flag('Z', result == 0);
        cpu.flags.set_flag('N', true);
        cpu.flags.set_flag('H', half_borrow);
        cpu.flags.set_flag('C', carry);
    }

    /*pub fn rst(&self, cpu: &mut Cpu, addr: u16) 
    {
        let pc = cpu.get_register_16("PC");
        cpu.push_word(pc);
        cpu.set_register_16(addr, "PC");
    }*/
}