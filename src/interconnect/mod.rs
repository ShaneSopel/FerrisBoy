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
    pub memory: Vec<u8>, // 64KB
}

impl Interconnect {
    pub fn new(memory: Vec<u8>) -> Self {
        Self { memory }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // Boot ROM, ROM banks
            0x0000..=0x7FFF => self.memory[addr as usize],

            // VRAM
            0x8000..=0x9FFF => self.memory[addr as usize],

            // External RAM (cartridge)
            0xA000..=0xBFFF => self.memory[addr as usize],

            // Work RAM
            0xC000..=0xDFFF => self.memory[addr as usize],

            // Echo RAM (mirror of C000-DDFF)
            0xE000..=0xFDFF => self.memory[(addr - 0x2000) as usize],

            // OAM
            0xFE00..=0xFE9F => self.memory[addr as usize],

            // Unusable area
            0xFEA0..=0xFEFF => 0xFF, // returns 0xFF

            // I/O registers
            0xFF00..=0xFF7F => self.read_io(addr),

            // High RAM
            0xFF80..=0xFFFE => self.memory[addr as usize],

            // Interrupt Enable
            0xFFFF => self.memory[0xFFFF],

            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            // ROM is read-only
            0x0000..=0x7FFF => {}

            // VRAM
            0x8000..=0x9FFF => self.memory[addr as usize] = value,

            // External RAM
            0xA000..=0xBFFF => self.memory[addr as usize] = value,

            // Work RAM
            0xC000..=0xDFFF => self.memory[addr as usize] = value,

            // Echo RAM
            0xE000..=0xFDFF => self.memory[(addr - 0x2000) as usize] = value,

            // OAM
            0xFE00..=0xFE9F => self.memory[addr as usize] = value,

            // Unusable
            0xFEA0..=0xFEFF => {}

            // I/O
            0xFF00..=0xFF7F => self.write_io(addr, value),

            // High RAM
            0xFF80..=0xFFFE => self.memory[addr as usize] = value,

            // Interrupt Enable
            0xFFFF => self.memory[0xFFFF] = value,

            _ => {}
        }
    }

        fn read_io(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => 0,       // DIV counter, placeholder
            0xFF44 => 0,       // LY register, read-only
            _ => self.memory[addr as usize], // other IO
        }
    }

    fn write_io(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF04 => self.memory[0xFF04] = 0, // writing resets DIV
            0xFF44 => {}                        // LY read-only
            _ => self.memory[addr as usize] = value,
        }
    }
}

