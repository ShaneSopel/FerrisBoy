/*extern crate sdl2;
mod cpu;
mod cart;

use std::env; 

pub struct EmuContext
{
    pub paused : bool,
    pub running : bool,
    pub ticks : u64
}

impl EmuContext
{

    pub fn emu_run(&mut self, args: Vec<String>) -> i32
    {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2
        {
            println!("Usage: emu <rom_file> \n");
            return -1;
        }

        if (!cart::cart_load(args[1]))
        {
            println!("Failed to load Rom file: {}\n", args[1]);
            return 2;
        }

        println!("Cart Loaded... \n");

        let sdl_context = sdl2::init().unwrap();
        println!("SDL INIT\n");
       // let ttf_context = sdl2::ttf::init().unwrap();
       // println!("TTF INIT\n");

        while self.running == true
        {
            if self.paused
            {
                continue;
            }

            if !cpu::cpu_step()
            {
                println!("CPU Stopped");
                return -3;
            }

            self.ticks = self.ticks + 1;
            println!("number of ticks is: {}", self.ticks);
    
        } 

        return 0;
    }

    fn delay(ms: u32)
    {

    }
}*/
