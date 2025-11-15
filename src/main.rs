extern crate sdl2;

mod cpu;
mod interconnect;
mod cart;

use std::env;

use std::io::Result;

use crate::cart::Cart;

fn main() -> Result<()>
{
    let mut cart = cart::Cart::new();

    cart.filename = "/home/shane/rust/FerrisBoy/roms/dmg-acid2.gb".to_string();
    cart.cart_load()?;

    let inter = interconnect::Interconnect::new(cart.rom_data);

    if let Some(header) = &cart.rom_head
    {
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

    for _ in 0..10
    {
        cpu.step();
    }

    Ok(())

}
