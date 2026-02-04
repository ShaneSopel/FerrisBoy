use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
//use sdl2::Sdl;

use crate::interconnect::Interconnect;

#[derive(Debug, Clone)]
pub enum PpuMode {
    HBlank,
    VBlank,
    Oam,
    Vram,
}

#[derive(Debug, Clone)]
pub struct Ppu {
    pub framebuffer: [u32; 160 * 144],
    pub mode: PpuMode,
    cycles: u64,
    scanline: u16,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            framebuffer: [0; 160 * 144],
            mode: PpuMode::Oam,
            cycles: 0,
            // scanline is one row of pixels on the screen.
            scanline: 0,
        }
    }

    pub fn step(&mut self, cpu_cycles: u64, inter: &mut Interconnect) {
        self.cycles += cpu_cycles;

        let lcdc = inter.read_byte(0xFF40);
        if lcdc & 0x80 == 0 {
            // LCD disabled → reset scanline
            self.cycles = 0;
            self.scanline = 0;
            inter.write_ly(0);
            self.mode = PpuMode::HBlank;
            return;
        }

        while self.cycles >= 1 {
            let mode_duration = match self.mode {
                PpuMode::Oam => 80,
                PpuMode::Vram => 172,
                PpuMode::HBlank => 204,
                PpuMode::VBlank => 456,
            };

            if self.cycles < mode_duration {
                break;
            }

            self.cycles -= mode_duration;

            match self.mode {
                PpuMode::Oam => {
                    self.render_scanline(inter);
                    self.mode = PpuMode::HBlank;
                }

                PpuMode::Vram => {
                    self.render_scanline(inter);
                    self.mode = PpuMode::HBlank;
                }
                PpuMode::HBlank => {
                    self.scanline += 1;
                    inter.write_ly(self.scanline as u8);

                    if self.scanline == 144 {
                        self.mode = PpuMode::VBlank;
                        let iflag = inter.read_byte(0xFF0F);
                        inter.write_byte(0xFF0F, iflag | 0x01); // Request VBlank interrupt
                    } else {
                        self.mode = PpuMode::Oam;
                    }
                }
                PpuMode::VBlank => {
                    self.scanline += 1;
                    inter.write_ly(self.scanline as u8);

                    if self.scanline > 153 {
                        // End of VBlank, start new frame
                        self.scanline = 0;
                        self.mode = PpuMode::Oam;
                    }
                }
            }

            // Update STAT register
            let stat_mode_val = match self.mode {
                PpuMode::HBlank => 0,
                PpuMode::Vram => 1,
                PpuMode::VBlank => 2,
                PpuMode::Oam => 3,
            };
            inter.set_stat_mode(stat_mode_val);
        }
    }

    pub fn render_scanline(&mut self, inter: &mut Interconnect) {
        let lcdc = inter.read_byte(0xFF40);

        if lcdc & 0x80 == 0 {
            return;
        }

        let scy = inter.read_byte(0xFF42) as usize;
        let scx = inter.read_byte(0xFF43) as usize;

        let ly = self.scanline as usize;
        let y = (ly + scy) & 0xFF;

        let bg_map_base: u16 = if lcdc & 0x08 != 0 { 0x9C00 } else { 0x9800 };

        let tile_data_base: u16 = if lcdc & 0x10 != 0 { 0x8000 } else { 0x8800 };

        for x in 0..160 {
            let x_bg = (x + scx) & 0xFF;

            let tile_x = x_bg / 8;
            let tile_y = y / 8;

            let map_index = tile_y * 32 + tile_x;
            let tile_id = inter.read_byte(bg_map_base + map_index as u16);

            let tile_index: i16 = if tile_data_base == 0x8000 {
                tile_id as i16
            } else {
                (tile_id as i8 as i16) + 128
            };

            if self.scanline == 0 && x == 0 {
                println!("Tile ID {:02X} from map {:04X}", tile_id, bg_map_base);
            }

            let tile_addr = tile_data_base + (tile_index as u16 * 16);

            let row = y % 8;
            let lo = inter.read_byte(tile_addr + (row * 2) as u16);
            let hi = inter.read_byte(tile_addr + (row * 2 + 1) as u16);

            let bit = 7 - (x_bg % 8);
            let color = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);

            let pixel = Self::dmg_color_to_rgba(color);
            self.framebuffer[ly * 160 + x] = pixel;
        }
    }

    fn dmg_color_to_rgba(color: u8) -> u32 {
        match color {
            0 => 0xFFFFFFFF, // white
            1 => 0xFFAAAAAA, // light gray
            2 => 0xFF555555, // dark gray
            3 => 0xFF000000, // black
            _ => 0xFFFFFFFF,
        }
    }

    /// Draw the framebuffer to an SDL canvas
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, 160, 144)
            .unwrap();

        texture
            .update(None, bytemuck::cast_slice(&self.framebuffer), 160 * 4)
            .unwrap();

        canvas.clear();
        canvas
            .copy(&texture, None, Some(Rect::new(0, 0, 160 * 4, 144 * 4)))
            .unwrap();
        canvas.present();
    }
}

// Initialize SDL and return a canvas
/*pub fn init_sdl(title: &str) -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(title, 160 * 4, 144 * 4)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().accelerated().build().unwrap();

    (sdl_context, canvas)
}*/
