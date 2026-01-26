extern crate sdl2;

mod cart;
mod cpu;
mod interconnect;
mod ppu;

use std::{env, io::Result};
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use crate::cart::Cart;
//use crate::ppu::{Ppu, init_sdl};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut cart = cart::Cart::new();

    cart.filename = args[1].clone();
    cart.cart_load()?;

    let inter = interconnect::Interconnect::new(cart.rom_data);

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
    let mut ppu = ppu::Ppu::new(cpu.inter.clone());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("FerrisBoy", 160 * 4, 144 * 4)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut last_cpu_cycles = cpu.cycles;

    'outer: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            if matches!(
                event,
                Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
            ) {
                break 'outer;
            }
        }

        // Step CPU
        cpu.step();

        // Calculate cycles since last CPU step
        let delta_cycles = cpu.cycles - last_cpu_cycles;
        last_cpu_cycles = cpu.cycles;

        // Step PPU
        ppu.step(delta_cycles * 4); // GB PPU is 4x CPU cycles

        // Draw framebuffer
        ppu.draw(&mut canvas);
    }

    Ok(())
}
