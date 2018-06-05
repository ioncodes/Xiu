#![feature(untagged_unions)]

mod registers;
mod instructions;
mod cpu;

use std::env;
use cpu::CPU;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut verbose = false;
    if args.contains(&String::from("-v")) {
        verbose = true;
    }
    let mut cpu = CPU::new(String::from("DMG_ROM.bin"), verbose);

    cpu.run();
}
