extern crate sdl2;

mod cart;
mod cpu;
mod interconnect;
mod ppu;

//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use std::io::Result;

use std::time::{Duration, Instant};

use crate::cart::Cart;
//use crate::ppu::{Ppu, init_sdl};
use crate::ppu::PpuMode;

use crate::interconnect::Interconnect;

use crate::cpu::Cpu;


use crate::cpu::registers::{Reg8,Reg16};



fn main() -> Result<()> {
    let mut cart = cart::Cart::new();

    cart.filename = "/home/shanesopel/rust/FerrisBoy/roms/dmg-acid2.gb".to_string();
    cart.cart_load()?;

    println!(
        "Cart ROM[0x100..0x110] = {:02X?}",
        &cart.rom_data[0x100..0x110]
    );

    let rom_slice = &cart.rom_data[0x4808..0x480A];
println!("{:02X?}", rom_slice);


    let mut ppu = ppu::Ppu::new();

    let mut inter = interconnect::Interconnect::new(cart.rom_data);

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

    println!("RESET PC = {:04X}", cpu.regs.pc);

    cpu.run_boot_rom();

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("FerrisBoy", 160 * 4, 144 * 4)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let frame_duration = Duration::from_micros(16_666);
    let mut last_frame = Instant::now();

    loop {
        let mut cycles_this_frame = 0;
        while cycles_this_frame < 69905 {
            let cpu_cycles = cpu.step();
            cycles_this_frame += cpu_cycles;

            ppu.step(cpu_cycles * 4, &mut cpu.inter);

            // println!("Scanline: {}", ppu.scanline);
        }

        if matches!(ppu.mode, PpuMode::VBlank) {
            ppu.draw(&mut canvas);
        }

        let now = Instant::now();
        if let Some(remaining) = frame_duration.checked_sub(now - last_frame) {
            std::thread::sleep(remaining);
        }
        last_frame = Instant::now();
    }
}
