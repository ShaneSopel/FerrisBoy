use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
//use sdl2::Sdl;

use crate::interconnect::Interconnect;

pub enum PpuMode {
    HBlank,
    VBlank,
    Oam,
    // Vram,
}

pub struct Ppu {
    pub framebuffer: [u32; 160 * 144],
    pub mode: PpuMode,
    cycles: u64,
    scanline: u16,
    pub inter: Interconnect,
}

impl Ppu {
    pub fn new(interconnect: Interconnect) -> Self {
        Self {
            framebuffer: [0; 160 * 144],
            mode: PpuMode::Oam,
            cycles: 0,
            // scanline is one row of pixels on the screen.
            scanline: 0,
            inter: interconnect,
        }
    }

    pub fn step(&mut self, cpu_cycles: u64) {
        self.cycles += cpu_cycles;

        while self.cycles >= 456 {
            self.cycles -= 456;

            if self.scanline < 144 {
                self.render_scanline();
            }

            self.scanline += 1;

            if self.scanline == 144 {
                self.mode = PpuMode::VBlank;
            } else if self.scanline > 153 {
                self.scanline = 0;
                self.mode = PpuMode::Oam;
            } else {
                self.mode = PpuMode::HBlank;
            }
        }
    }

    fn fetch_tile_pixel(&self, tile_index: u8, x: usize, y: usize) -> u8 {
        let tile_addr = (tile_index as usize) * 16;
        let row_addr = tile_addr + y * 2;
        let low = self.inter.vram[row_addr];
        let high = self.inter.vram[row_addr + 1];

        let bit = 7 - x;
        ((high >> bit) & 1) << 1 | ((low >> bit) & 1)
    }

    fn map_color_to_rgb(&self, color: u8) -> u32 {
        match color {
            0 => 0xFFFFFFFF,
            1 => 0xAAAAAAFF,
            2 => 0x555555FF,
            3 => 0x000000FF,
            _ => 0xFFFFFFFF,
        }
    }

    /// Render one scanline (background only for now)
    fn render_scanline(&mut self) {
        let y = self.scanline as usize;
        let bg_map_base = 0x1800; // 0x9800 - 0x8000

        for x in 0..160 {
            let tile_x = x / 8;
            let tile_y = y / 8;
            let map_index = tile_y * 32 + tile_x;

            let tile_index = self.inter.vram[bg_map_base + map_index];
            let pixel_x = x % 8;
            let pixel_y = y % 8;

            let color_id = self.fetch_tile_pixel(tile_index, pixel_x, pixel_y);
            let color = self.map_color_to_rgb(color_id);

            self.framebuffer[y * 160 + x] = color;
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
