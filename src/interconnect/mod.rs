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

//pub const LCDC: u16 = 0xFF40;
pub const STAT: u16 = 0xFF41;
//pub const SCY: u16 = 0xFF42;
//pub const SCX: u16 = 0xFF43;
pub const LY: u16 = 0xFF44;
//pub const LYC: u16 = 0xFF45;

pub struct Interconnect {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Interconnect {
    pub fn new(rom: Vec<u8>) -> Self {

        let mut ram = vec![0; 0x10000];
        ram[0xFF40] = 0x91; // LCDC
        ram[0xFF41] = 0x85; // STAT
        ram[0xFF42] = 0x00; // SCY
        ram[0xFF43] = 0x00; // SCX
        ram[0xFF44] = 0x00; // LY
        ram[0xFF47] = 0xFC; // BGP
        ram[0xFF0F] = 0xE1; // IF

        Self { rom, ram }
    }

pub fn read_byte(&self, addr: u16) -> u8 {
    match addr {
        0x0000..=0x7FFF => self.rom[addr as usize],
         0x8000..=0xFDFF => self.ram[addr as usize],
        0xFE00..=0xFFFF => 0x00,
        _ => self.ram[addr as usize],
    }
}


pub fn write_byte(&mut self, addr: u16, value: u8) {
    match addr {
        0x0000..=0x7FFF => {} // ROM is read-only
        _ => self.ram[addr as usize] = value,
    }
}

fn read_io(&self, addr: u16) -> u8 {
    match addr {
        0xFF04 => self.ram[0xFF04], // DIV
        0xFF44 => self.ram[0xFF44], // LY
        _ => self.ram[addr as usize],
    }
}

fn write_io(&mut self, addr: u16, value: u8) {
    match addr {
        0xFF04 => self.ram[0xFF04] = 0,
        0xFF44 => {}
        _ => self.ram[addr as usize] = value,
    }
}

}

