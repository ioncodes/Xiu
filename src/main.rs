#![feature(untagged_unions)]

mod registers;
mod instructions;
#[allow(dead_code)] // consts are mostly unused atm
mod memory;
mod cpu;

use std::env;
use cpu::CPU;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut verbose = false;
    if args.contains(&String::from("-v")) {
        verbose = true;
    }

    let mut cpu = CPU::new(args.get(1).unwrap().to_string(), verbose);
    cpu.run();
}
