use std::fs;
use std::io::{Error, ErrorKind, Result};


#[derive(Debug, Clone)]
pub struct RomHeader
{
    //commented out values are values I am not currently implementing
    //pub entry: String,
    //pub logo: String,
    pub title: String,
    //pub new_lic_code: u16,
    //pub sgb_flag: u8,
    pub type_val: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub dest_code: u8,
    pub lic_code: u8,
    pub version: u8,
    pub checksum: u8,
    pub global_checksum: u16,
}

pub struct Cart
{
    pub filename: String, //[char; 1024],
    pub rom_data: Vec<u8>,
    pub rom_head: Option<RomHeader> //rc<rom_header>
}

impl Cart
{

    pub fn new() -> Cart
    {
        Cart 
        {
            filename: "none".to_string(),
            rom_data: Vec::new(),
            rom_head: None,
        }

    }

    pub fn rom_size_bytes(code: u8) -> &'static str
    {
        match code 
        {
           
            0x00 => "32KB",
            0x01 => "64KB",
            0x02 => "128KB",
            0x03 => "256KB",
            0x04 => "512KB",
            0x05 => "1MB",
            0x06 => "2MB",
            0x07 => "4MB",
            0x08 => "8MB",
            0x52 => "1.1MB",
            0x53 => "1.2MB",
            0x54 => "1.5MB",
            _ => "Unknown",
        }
    }

    pub fn cart_type_name(value:u8) -> &'static str
    {
        match value
        {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1 + RAM",
            0x03 => "MBC1 + RAM + BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2 + BATTERY",
            0x08 => "ROM + RAM",
            0x09 => "ROM + RAM + BATTERY",
            0x0B => "MMM01",
            0x0C => "MMM01 + RAM",
            0x0D => "MMM01 + RAM + BATTERY",
            _ => "UNKOWN",
        }
    }

    pub fn license_name(code: u8) -> &'static str {
        match code {
            0x00 => "None",
            0x01 => "Nintendo",
            0x08 => "Capcom",
            0x09 => "Hot-B",
            0x0A => "Jaleco",
            0x0B => "Coconuts Japan",
            0x0C => "Elite Systems",
            0x13 => "EA (Electronic Arts)",
            0x18 => "Hudson Soft",
            0x19 => "ITC Entertainment",
            0x1A => "Yanoman",
            0x1D => "Japan Clary",
            0x1F => "Virgin Interactive",
            0x24 => "PCM Complete",
            0x25 => "San-X",
            0x28 => "Kotobuki Systems",
            0x29 => "Seta",
            0x30 => "Infogrames",
            0x31 => "Nintendo",
            0x32 => "Bandai",
            0x33 => "Check New Licensee Code", // special marker for 0x144-0x145 field
            0x34 => "Konami",
            0x35 => "HectorSoft",
            0x38 => "Capcom",
            0x39 => "Banpresto",
            0x3C => "Entertainment i",
            0x3E => "Gremlin",
            0x41 => "Ubisoft",
            0x42 => "Atlus",
            0x44 => "Malibu",
            0x46 => "Angel",
            0x47 => "Spectrum Holobyte",
            0x49 => "Irem",
            0x4A => "Virgin Games",
            0x4D => "Malibu",
            0x4F => "U.S. Gold",
            0x50 => "Absolute",
            0x51 => "Acclaim",
            0x52 => "Activision",
            0x53 => "American Sammy",
            0x54 => "Konami",
            0x55 => "Hi Tech",
            0x56 => "LJN",
            0x57 => "Matchbox",
            0x59 => "Milton Bradley",
            0x5A => "Mindscape",
            0x5B => "Romstar",
            0x5C => "Naxat Soft",
            0x5D => "Tradewest",
            0x60 => "Titus",
            0x61 => "Virgin",
            0x67 => "Ocean",
            0x69 => "EA (Electronic Arts)",
            0x6E => "Elite Systems",
            0x6F => "Electro Brain",
            0x70 => "Infogrames",
            0x71 => "Interplay",
            0x72 => "Broderbund",
            0x73 => "Sculptered Soft",
            0x75 => "The Sales Curve",
            0x78 => "THQ",
            0x79 => "Accolade",
            0x7A => "Triffix Entertainment",
            0x7C => "Microprose",
            0x7F => "Kemco",
            0x80 => "Misawa Entertainment",
            0x83 => "Lozc",
            0x86 => "Tokuma Shoten",
            0x8B => "Bullet-Proof Software",
            0x8C => "Vic Tokai",
            0x8E => "Ape",
            0x8F => "I'Max",
            0x91 => "Chun Soft",
            0x92 => "Video System",
            0x93 => "Tsuburava",
            0x95 => "Varie",
            0x96 => "Yonezawa/S'Pal",
            0x97 => "Kaneko",
            0x99 => "Pack-In-Video",
            0x9A => "Nichibutsu",
            0x9B => "Tecmo",
            0x9C => "Imagineer",
            0xA4 => "Konami (Yu-Gi-Oh!)",
            0xA6 => "Kawada",
            0xA7 => "Takara",
            0xA9 => "Technos Japan",
            0xAA => "Broderbund",
            0xAC => "Toei Animation",
            0xAD => "Toho",
            0xAF => "Namco",
            0xB0 => "Acclaim",
            0xB1 => "ASCII or Nexoft",
            0xB2 => "Bandai",
            0xB4 => "Enix",
            0xB6 => "HAL Laboratory",
            0xB7 => "SNK",
            0xB9 => "Pony Canyon",
            0xBA => "Culture Brain",
            0xBB => "Sunsoft",
            0xBD => "Sony Imagesoft",
            0xBF => "Sammy",
            0xC0 => "Taito",
            0xC2 => "Kemco",
            0xC3 => "Squaresoft",
            0xC4 => "Tokuma Shoten Intermedia",
            0xC5 => "Data East",
            0xC6 => "Tonkin House",
            0xC8 => "Koei",
            0xC9 => "UFL",
            0xCA => "Ultra",
            0xCB => "Vap",
            0xCC => "Use Co., Ltd.",
            0xCD => "Meldac",
            0xCE => "Pony Canyon or",
            0xCF => "Angel",
            0xD0 => "Taito",
            0xD1 => "Sofel",
            0xD2 => "Quest",
            0xD3 => "Sigma Enterprises",
            0xD4 => "Ask Kodansha",
            0xD6 => "Naxat Soft",
            0xD7 => "Copya Systems",
            0xD9 => "Banpresto",
            0xDA => "Tomy",
            0xDB => "LJN",
            0xDD => "NCS",
            0xDE => "Human",
            0xDF => "Altron",
            0xE0 => "Jaleco",
            0xE1 => "Towachiki",
            0xE2 => "Uutaka",
            0xE5 => "Epoch",
            0xE7 => "Athena",
            0xE8 => "Asmik",
            0xE9 => "Natsume",
            0xEA => "King Records",
            0xEB => "Atlus",
            0xEC => "Epic/Sony Records",
            0xEE => "IGS",
            0xF0 => "A Wave",
            0xF3 => "Extreme Entertainment",
            0xFF => "LJN",
            _ => "Unknown",
        }
    }

    pub fn cart_load(&mut self) -> Result<()>
    {

        // requesting memory for rom size. 
        self.rom_data = fs::read(&self.filename).unwrap();
        self.rom_head = Some(Self::parse_header(&self.rom_data)?);

        Ok(())

    }

    fn parse_header(rom: &[u8]) -> Result<RomHeader>
    {
        if rom.len() < 0x150
        {
            return Err(Error::new(ErrorKind::UnexpectedEof, "ROM too small"));
        }

        //let entry = &rom[0x100..0x104];
        //let logo  = &rom[0x104..0x134];
        let title = &rom[0x134..0x144];
        //let new_lic_code = u16::from_be_bytes([rom[0x144], rom[0x145]]);
        //let sgb_flag = rom[0x146];
        let type_val = rom[0x147];
        let rom_size = rom[0x148];
        let ram_size = rom[0x149];
        let dest_code = rom[0x14A];
        let lic_code = rom[0x14B];
        let version = rom[0x14C];
        let checksum = rom[0x14D];
        let global_checksum =u16::from_be_bytes([rom[0x14E], rom[0x14F]]);

        // Convert binary data into hex string representations
        //let entry_str = entry.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
        //let logo_str = logo.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");

        // Title as UTF-8 string
        let title_str = std::str::from_utf8(title)
        .unwrap_or("")
        .trim_end_matches('\0')
        .to_string();

        

        Ok(RomHeader {
            //entry: entry_str,
            //logo: logo_str,
            title: title_str,
            //new_lic_code,
            //sgb_flag,
            type_val,
            rom_size,
            ram_size,
            dest_code,
            lic_code,
            version,
            checksum,
            global_checksum,
        })

    }

}