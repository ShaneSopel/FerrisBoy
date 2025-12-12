pub struct Alu;

pub struct AluResult8 {
    pub result: u8,
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

pub struct AluResult16 {
    pub result: u16,
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Alu {
    pub fn new() -> Self {
        let alu = Self {};

        alu
    }

    pub fn add_8bit(&self, a: u8, b: u8) -> AluResult8 {
        let (result, carry) = a.overflowing_add(b);

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: ((a & 0x0F) + (b & 0x0F)) > 0x0F,
            c: carry,
        }
    }

    pub fn add_16bit(&self, a: u16, b: u16) -> AluResult16 {
        let result = a.wrapping_add(b);

        let half_carry = (((a & 0x0FFF) + (b & 0x0FFF)) & 0x1000) != 0;
        let carry = (a as u32 + b as u32) > 0xFFFF;

        AluResult16 {
            result,
            z: result == 0,
            n: false,
            h: half_carry,
            c: carry,
        }
    }

    pub fn adc_8bit(&self, cpu_flag: bool, a: u8, b: u8) -> AluResult8 {
        //let carry_in = if cpu.flags.get_flag('C') { 1 } else { 0 };
        let carry_in = if cpu_flag { 1 } else { 0 };
        let result = a.wrapping_add(b).wrapping_add(carry_in);

        let half_carry = ((a & 0x0F) + (b & 0x0F) + carry_in) > 0x0F;
        let carry_out = (a as u16) + (b as u16) + (carry_in as u16) > 0xFF;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: half_carry,
            c: carry_out,
        }
    }

    pub fn sub_8bit(&self, a: u8, b: u8) -> AluResult8 {
        let (result, borrow) = a.overflowing_sub(b);

        let half_borrow = (a & 0x0F) < (b & 0x0F);

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: half_borrow,
            c: borrow,
        }
    }

    pub fn sbc_8bit(&self, cpu_flag: bool, a: u8, b: u8) -> AluResult8 {
        //let carry_in = if cpu.flags.get_flag('C') { 1 } else { 0 };
        let carry_in = if cpu_flag { 1 } else { 0 };

        // Subtract b and carry from a
        let (intermediate, borrow1) = a.overflowing_sub(b);
        let (result, borrow2) = intermediate.overflowing_sub(carry_in);

        // Half-carry (borrow from bit 4)
        let half_borrow = ((a & 0x0F).wrapping_sub((b & 0x0F) + carry_in)) & 0x10 != 0;

        AluResult8 {
            result,
            z: result == 0,
            n: true,
            h: half_borrow,
            c: borrow1 || borrow2,
        }
    }

    pub fn xor_8bit(&self, a: u8, b: u8) -> AluResult8 {
        let result = a ^ b;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: false,
        }
    }

    pub fn and_8bit(&self, a: u8, b: u8) -> AluResult8 {
        let result = a & b;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: true,
            c: false,
        }
    }

    pub fn or_8bit(&self, a: u8, b: u8) -> AluResult8 {
        let result = a | b;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: false,
        }
    }

    pub fn cp_8bit(&self, a: u8, b: u8) -> AluResult8 {
        let result = a - b;

        let half_borrow = (a & 0x0F) < (b & 0x0F);
        let carry = a < b;

        AluResult8 {
            result,
            z: result == 0,
            n: true,
            h: half_borrow,
            c: carry,
        }
    }

    pub fn rlc_byte(&mut self, v: u8) -> AluResult8 {
        let carry = (v & 0x80) != 0;
        let result = v.rotate_left(1);

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: carry,
        }
    }

    pub fn rrc_byte(&mut self, v: u8) -> AluResult8 {
        let carry = (v & 0x01) != 0;
        let result = v.rotate_right(1);

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: carry,
        }
    }

    pub fn rl_byte(&mut self, v: u8, cpu_flag: bool) -> AluResult8 {
        let old_c = cpu_flag as u8;
        let new_carry = (v & 0x80) != 0;
        let result = (v << 1) | old_c;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: new_carry,
        }
    }

    pub fn rr_byte(&mut self, v: u8, cpu_flag: bool) -> AluResult8 {
        let old_c = cpu_flag as u8;
        let new_carry = (v & 0x01) != 0;
        let result = (v >> 1) | (old_c << 7);

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: new_carry,
        }
    }

    pub fn sla_byte(&mut self, v: u8) -> AluResult8 {
        let carry = (v & 0x80) != 0;
        let result = v << 1;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: carry,
        }
    }

    pub fn sra_byte(&mut self, v: u8) -> AluResult8 {
        let carry = (v & 0x01) != 0;
        let msb = v & 0x80;
        let result = (v >> 1) | msb;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: carry,
        }
    }

    pub fn srl_byte(&mut self, v: u8) -> AluResult8 {
        let carry = (v & 0x01) != 0;
        let result = v >> 1;

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: carry,
        }
    }

    pub fn swap_byte(&mut self, v: u8) -> AluResult8 {
        let result = (v << 4) | (v >> 4);

        AluResult8 {
            result,
            z: result == 0,
            n: false,
            h: false,
            c: false,
        }
    }

    /*pub fn rst(&self, cpu: &mut Cpu, addr: u16)
    {
        let pc = cpu.get_register_16("PC");
        cpu.push_word(pc);
        cpu.set_register_16(addr, "PC");
    }*/
}
