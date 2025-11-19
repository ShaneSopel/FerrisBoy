//0x0000 - 0x3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
//0x4000 - 0x7FFF	16 KiB ROM Bank 01–NN	From cartridge, switchable bank via mapper (if any)
//0x8000 - 0x9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1 // (0x8000 - 0x97FF BG Map 1) (0x9C00 - 0x9FFF BM Map 2)
//0xA000 - 0xBFFF	8 KiB External RAM	From cartridge, switchable bank if any
//0xC000 - 0xCFFF	4 KiB Work RAM (WRAM)	RAM BANK 0
//0xD000 - 0xDFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1–7
//0xE000 - 0xFDFF	Echo RAM (mirror of C000–DDFF)	Nintendo says use of this area is prohibited.
//0xFE00 - 0xFE9F	Object attribute memory (OAM)	
//0xFEA0 - 0xFEFF	Not Usable	Nintendo says use of this area is prohibited.
//0xFF00 - 0xFF7F	I/O Registers	
//0xFF80 - 0xFFFE	High RAM (HRAM) (zero page)	
//FFFF	FFFF	Interrupt Enable register (IE)	

use crate::interconnect;


pub struct Interconnect
{


    pub rom: [u8; 0x8000],
    pub vram: [u8; 0x2000],
    pub wram: [u8; 0x2000],
    pub oam: [u8; 0xA0],
    pub io: [u8; 0x80],
    pub hram: [u8; 0x7F],
    pub ie_register: u8,
}

impl Interconnect {
    /// Initialize with a test Vec<u8> memory
    pub fn new(memory: Vec<u8>) -> Self 
    {
        let mut inter = Self 
        {
            rom: [0; 0x8000],
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie_register: 0,
        };

        for (addr, &val) in memory.iter().enumerate() {
            let address = addr as u16;
            match address {
                0x0000..=0x7FFF => inter.rom[address as usize] = val,
                0x8000..=0x9FFF => inter.vram[(address - 0x8000) as usize] = val,
                0xC000..=0xDFFF => inter.wram[(address - 0xC000) as usize] = val,
                0xFE00..=0xFE9F => inter.oam[(address - 0xFE00) as usize] = val,
                0xFF00..=0xFF7F => inter.io[(address - 0xFF00) as usize] = val,
                0xFF80..=0xFFFE => inter.hram[(address - 0xFF80) as usize] = val,
                0xFFFF => inter.ie_register = val,
                _ => {}
            }
        }
        inter
    }

    pub fn read_byte(&mut self, address: u16) -> u8
    {
        match address
        {
            0x0000..=0x7FFF => self.rom[address as usize],
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF           => self.ie_register,

            _ => 0xFF 
        }

    }

    pub fn write_byte(&mut self, address: u16, value: u8)
    {
        match address
        {
         
                0x0000..=0x7FFF => self.rom[address as usize] = value,
                0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
                0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] =  value,
                0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
                0xFF00..=0xFF7F => self.io[(address - 0xFF00) as usize] = value,
                0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
                0xFFFF          => self.ie_register = value,
    
                _ => (),
        }
    }

   // pub fn fetch_byte(&self, mem_addr: u16) -> u8
   // {
    
   //     return 0;

   // }

   // pub fn store_byte(&mut self, mem_addr: u16, val: u8)
   // {


   // }
    
}
