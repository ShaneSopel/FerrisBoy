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
pub const LYC: u16 = 0xFF45;

#[derive(Debug, Clone)]
pub struct Interconnect {
    pub rom: [u8; 0x8000], // Cartridge ROM
    pub vram: [u8; 0x2000],
    pub wram: [u8; 0x2000],
    pub oam: [u8; 0xA0],
    pub io: [u8; 0x80],
    pub hram: [u8; 0x7F],
    pub ie_register: u8,
    pub boot_enabled: bool,
}

impl Interconnect {
    pub fn new(cart_rom: Vec<u8>) -> Self {
        let mut inter = Self {
            //boot_rom: [0; 0x100],
            rom: [0; 0x8000],
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie_register: 0,
            boot_enabled: true,
        };

        // Load cartridge ROM (truncate if larger than 32KB)
        let rom_len = usize::min(cart_rom.len(), 0x8000);
        inter.rom[..rom_len].copy_from_slice(&cart_rom[..rom_len]);

        inter
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            // Writes to boot enable register
            0xFF50 if value == 1 => self.boot_enabled = false,

            0x0000..=0x7FFF => self.rom[addr as usize] = value, // usually MBC handles this
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => (),
        }
    }

    pub fn write_ly(&mut self, value: u8) {
        self.io[(LY - 0xFF00) as usize] = value;
    }

    pub fn read_ly(&self) -> u8 {
        self.io[(LY - 0xFF00) as usize]
    }

    pub fn update_lyc(&mut self) {
        let ly = self.read_ly();
        let lyc = self.io[(LYC - 0xFF00) as usize];

        let stat = &mut self.io[(STAT - 0xFF00) as usize];

        if ly == lyc {
            *stat |= 0x04;
        } else {
            *stat &= !0x04;
        }
    }

    pub fn set_stat_mode(&mut self, mode: u8) {
        let stat = &mut self.io[(STAT - 0xFF00) as usize];
        *stat = (*stat & 0xFC) | (mode & 0x03);
    }
}
