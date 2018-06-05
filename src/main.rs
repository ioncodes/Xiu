#![feature(untagged_unions)]

mod registers;
mod instructions;
mod cpu;

use cpu::CPU;

fn main() {
    let mut cpu = CPU::new(String::from("DMG_ROM.bin"), true);

    cpu.run();
}
