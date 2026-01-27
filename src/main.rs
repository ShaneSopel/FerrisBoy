extern crate sdl2;

mod cart;
mod cpu;
mod interconnect;
mod ppu;

use std::io::Result;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use crate::cart::Cart;
//use crate::ppu::{Ppu, init_sdl};
use crate::ppu::PpuMode;

fn main() -> Result<()> {
    let mut cart = cart::Cart::new();

    cart.filename = "/home/shanesopel/rust/FerrisBoy/roms/dmg-acid2.gb".to_string();
    cart.cart_load()?;

    println!(
        "Cart ROM[0x100..0x110] = {:02X?}",
        &cart.rom_data[0x100..0x110]
    );

    let inter = interconnect::Interconnect::new(cart.rom_data);
    println!(
        "inter ROM[0x0100..0x0110]: {:02X?}",
        &inter.rom[0x0100..0x0110]
    );

    if let Some(header) = &cart.rom_head {
        let type2 = Cart::cart_type_name(header.type_val);
        let lic = Cart::license_name(header.lic_code);
        let rom_size = Cart::rom_size_bytes(header.rom_size);

        println!("Cartridge Loaded");
        println!("Title: {}", header.title);
        println!("Cart Type: {:02X} {}", header.type_val, type2);
        println!("Rom Size: {}", rom_size);
        println!("Ram Size: {}", header.ram_size);
        println!("Destination Code {:02x}", header.dest_code);
        println!("Checksum: {:02x}", header.checksum);
        println!("Lic Code: {:02x} {} ", header.lic_code, lic);
        println!("Rom Version: {}", header.version);
        println!("Global Checksum: {:04X}", header.global_checksum);
    }

    let mut cpu = cpu::Cpu::new(inter);
    let mut ppu = ppu::Ppu::new();

    println!("RESET PC = {:04X}", cpu.regs.pc);

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("FerrisBoy", 160 * 4, 144 * 4)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();

    'emu: loop {
        // 1️⃣ Run one CPU instruction
        let cpu_cycles = cpu.step();

        // 2️⃣ Step PPU (Game Boy PPU runs at 4x CPU speed)
        ppu.step(cpu_cycles * 4u64, &mut cpu.inter);

        // 3️⃣ When we hit VBlank, draw
        if matches!(ppu.mode, PpuMode::VBlank) {
            ppu.draw(&mut canvas);
        }
    }
}
