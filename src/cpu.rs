use std::fs::File;
use std::io::Read;
use instructions::{Instructions, get_instruction, get_debug};
use registers::Registers;
use memory::Memory;

pub struct CPU {
    rom: Vec<u8>,
    registers: Registers,
    memory: Memory,
    verbose: bool
}

impl CPU {
    pub fn new(rom: String, verbose: bool) -> CPU {
        let registers = Registers::new();
        let memory = Memory::new();

        let mut file = File::open(rom).unwrap();
        let mut rom = Vec::<u8>::new();
        let _ = file.read_to_end(&mut rom);

        CPU {
            rom,
            registers,
            verbose,
            memory
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_8();
            let instruction = get_instruction(opcode);
            let data = match instruction {
                Instructions::LD_SP_D16 => self.ld_sp_d16(),
                Instructions::XOR_A => self.xora(),
                Instructions::LD_HL_D16 => self.ld_hl_d16(),
                Instructions::LD_HLD_A => self.ld_hld_a(),
                Instructions::Unknown => panic!("0x{:x} Unknown opcode!", opcode)
            };
            if self.verbose {
                if data.is_some() {
                    println!("{:?}", get_debug(opcode, data.unwrap()));
                } else {
                    println!("{:?}", get_debug(opcode, vec![]));
                }
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

    fn ld_hl_d16(&mut self) -> Option<Vec<usize>> {
        let data = self.read_16();
        self.registers.hl.hl = data;
        Some(vec![data as usize])
    }

    fn ld_hld_a(&mut self) -> Option<Vec<usize>> {
        let hl = self.registers.get_hl();
        let a = self.registers.get_a();
        self.memory.write(hl as usize, a);
        self.registers.dec_hl();
        None
    }

    fn xora(&mut self) -> Option<Vec<usize>> {
        self.memory.clear_vram();
        None
    }
}
