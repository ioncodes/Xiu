use std::fs::File;
use std::io::Read;
use instructions::{Instructions, get_instruction, get_debug};
use registers::Registers;

pub struct CPU {
    rom: Vec<u8>,
    registers: Registers,
    verbose: bool
}

impl CPU {
    pub fn new(rom: String, verbose: bool) -> CPU {
        let registers = Registers::new();

        let mut file = File::open(rom).unwrap();
        let mut rom = Vec::<u8>::new();
        let _ = file.read_to_end(&mut rom);

        CPU {
            rom,
            registers,
            verbose
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_8();
            let instruction = get_instruction(opcode);
            let data = match instruction {
                Instructions::LDSP16 => self.ld_sp_d16(),
                Instructions::Unknown => panic!("0x{:x} Unknown opcode!", opcode)
            };
            if self.verbose && data.is_some() {
                println!("{:?}", get_debug(opcode, data.unwrap()));
            }
        }
    }

    fn read_8(&mut self) -> u8 {
        let byte = self.rom[self.registers.pc as usize];
        self.registers.step(1);
        byte
    }

    fn read_16(&mut self) -> u16 {
        let x = self.read_8();
        let y = self.read_8();
        (y as u16) << 8 | x as u16
    }

    fn ld_sp_d16(&mut self) -> Option<Vec<usize>> {
        let data = self.read_16();
        self.registers.sp = data;
        Some(vec![data as usize])
    }
}
