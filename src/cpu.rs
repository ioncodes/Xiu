use std::fs::File;
use std::io::Read;
use instructions::{Instructions, get_instruction, get_debug, get_prefixed_instruction, get_prefixed_debug};
use registers::{Registers, Register};
use memory::{Memory, IO};
use flags::Flags;
use stack::Stack;

pub struct CPU {
    rom: Vec<u8>,
    registers: Registers,
    memory: Memory,
    stack: Stack,
    verbose: bool
}

impl CPU {
    pub fn new(rom: String, verbose: bool) -> CPU {
        let registers = Registers::new();
        let memory = Memory::new();
        let stack = Stack::new();

        let mut file = File::open(rom).unwrap();
        let mut rom = Vec::<u8>::new();
        let _ = file.read_to_end(&mut rom);

        CPU {
            rom,
            registers,
            verbose,
            memory,
            stack,
        }
    }

    pub fn run(&mut self) {
        let mut prefixed = false;
        loop {
            let opcode = self.read_8();
            let instruction: &Instructions;
            if !prefixed {
                instruction = get_instruction(opcode);
            } else {
                instruction = get_prefixed_instruction(opcode);
            }
            let data = match instruction {
                Instructions::Prefixed => { prefixed = true; None },
                Instructions::LD_SP_D16 => self.ld_sp_d16(),
                Instructions::XOR_A => self.xora(),
                Instructions::LD_HL_D16 => self.ld_hl_d16(),
                Instructions::LD_HLD_A => self.ld_hld_a(),
                Instructions::BIT_7_H => self.bit_h(Flags::Z as u8),
                Instructions::JR_NZ_8 => self.jr_nz_8(),
                Instructions::LD_C_D8 => self.ld_x_d8(Register::C),
                Instructions::LD_A_D8 => self.ld_a_d8(),
                Instructions::LD_FFC_A => self.ld_ffc_a(),
                Instructions::INC_C => self.inc_c(),
                Instructions::LD_HL_A => self.ld_hl_a(),
                Instructions::LDH_D8_A => self.ldh_d8_a(),
                Instructions::LD_DE_D16 => self.ld_de_d16(),
                Instructions::LD_A_DE => self.ld_a_de(),
                Instructions::CALL_A16 => self.call_a16(),
                Instructions::LD_C_A => self.ld_c_a(),
                Instructions::LD_B_D8 => self.ld_x_d8(Register::B),
                Instructions::PUSH_BC => self.push(Register::BC),
                Instructions::RL_C => self.rl(Register::C),
                Instructions::Unknown => self.panic(opcode)
            };
            if self.verbose && *instruction != Instructions::Prefixed {
                if data.is_some() {
                    if prefixed {
                        println!("{}", get_prefixed_debug(opcode, data.unwrap()));
                        prefixed = false;
                    } else {
                        println!("{}", get_debug(opcode, data.unwrap()));
                    }
                } else {
                    if prefixed {
                        println!("{}", get_prefixed_debug(opcode, vec![]));
                        prefixed = false;
                    } else {
                        println!("{}", get_debug(opcode, vec![]));
                    }
                }
                //self.registers.dump();
            }
        }
    }

    fn panic(&mut self, opcode: u8) -> Option<Vec<usize>> {
        self.registers.dump();
        panic!("0x{:02x} Unknown opcode!", opcode);
    }

    pub fn read_8(&mut self) -> u8 {
        let byte = self.rom[self.registers.pc as usize];
        self.registers.step(1);
        byte
    }

    pub fn read_16(&mut self) -> u16 {
        let x = self.read_8();
        let y = self.read_8();
        (y as u16) << 8 | x as u16
    }

    fn ld_sp_d16(&mut self) -> Option<Vec<usize>> {
        let data = self.read_16();
        self.registers.sp = data;
        Some(vec![data as usize])
    }

    fn rl(&mut self, register: Register) -> Option<Vec<usize>> {
        // TODO: implement this
        /*
        match register {
            Register::A => ,
            Register::B => ,
            Register::C => ,
            Register::D => ,
            Register::E => ,
            Register::F => ,
            Register::H => ,
            Register::L => ,
            _ => panic!("Invalid register provided!"),
        }
        */
        None
    }

    fn push(&mut self, register: Register) -> Option<Vec<usize>> {
        match register {
            Register::AF => self.stack.push(self.registers.get_af()),
            Register::BC => self.stack.push(self.registers.get_bc()),
            Register::DE => self.stack.push(self.registers.get_de()),
            Register::HL => self.stack.push(self.registers.get_hl()),
            _ => panic!("Invalid register provided!"),
        }
        self.registers.sp -= 2;
        None
    }

    fn call_a16(&mut self) -> Option<Vec<usize>> {
        let data = self.read_16();
        let address = self.registers.pc;
        self.stack.push(address);
        self.registers.jump(data);
        Some(vec![data as usize])
    }

    fn ld_de_d16(&mut self) -> Option<Vec<usize>> {
        let data = self.read_16();
        self.registers.set_de(data);
        Some(vec![data as usize])
    }

    fn ld_hl_d16(&mut self) -> Option<Vec<usize>> {
        let data = self.read_16();
        self.registers.set_hl(data);
        Some(vec![data as usize])
    }

    fn ld_hld_a(&mut self) -> Option<Vec<usize>> {
        let hl = self.registers.get_hl();
        let a = self.registers.get_a();
        self.memory.write(hl as usize, a);
        self.registers.dec_hl();
        None
    }

    fn ld_x_d8(&mut self, register: Register) -> Option<Vec<usize>> {
        let byte = self.read_8();
        match register {
            Register::A => self.registers.set_a(byte),
            Register::B => self.registers.set_b(byte),
            Register::C => self.registers.set_c(byte),
            Register::D => self.registers.set_d(byte),
            Register::E => self.registers.set_e(byte),
            Register::F => self.registers.set_f(byte),
            Register::H => self.registers.set_h(byte),
            Register::L => self.registers.set_l(byte),
            _ => panic!("Invalid register provided!"),
        }
        Some(vec![byte as usize])
    }

    fn ldh_d8_a(&mut self) -> Option<Vec<usize>> {
        let byte = self.read_8() as usize;
        let a = self.registers.get_a();
        self.memory.write((IO.0 as usize) + byte, a);
        Some(vec![byte])
    }

    fn ld_ffc_a(&mut self) -> Option<Vec<usize>> {
        let c = self.registers.get_c() as usize;
        let a = self.registers.get_a();
        self.memory.write((IO.0 as usize) + c, a);
        None
    }

    fn ld_c_a(&mut self) -> Option<Vec<usize>> {
        let a = self.registers.get_a();
        self.registers.set_c(a);
        None
    }

    fn ld_hl_a(&mut self) -> Option<Vec<usize>> {
        // TODO: Not so sure about this tbh
        let hl = self.registers.get_hl();
        let a = self.registers.get_a();
        self.memory.write(hl as usize, a);
        None
    }

    fn ld_a_d8(&mut self) -> Option<Vec<usize>> {
        let byte = self.read_8();
        self.registers.set_a(byte);
        Some(vec![byte as usize])
    }

    fn ld_a_de(&mut self) -> Option<Vec<usize>> {
        // TODO: check this, i think it's wrong.
        let c = self.registers.get_c() as usize;
        let byte = self.memory.read((IO.0 as usize) + c);
        self.registers.set_a(byte);
        None
    }

    fn xora(&mut self) -> Option<Vec<usize>> {
        let result = self.registers.get_a() ^ self.registers.get_a();
        if result == 0 {
            self.registers.set_flag_z();
        }
        self.registers.set_a(result);
        self.registers.clear_flag_n();
        self.registers.clear_flag_h();
        self.registers.clear_flag_c();
        None
    }

    fn bit_h(&mut self, bit: u8) -> Option<Vec<usize>> {
        let h = self.registers.get_h();
        let n = self.registers.get_bit(h, bit);
        if n == 0 {
            self.registers.set_flag_z();
        }
        self.registers.clear_flag_n();
        self.registers.set_flag_h();
        None
    }

    fn jr_nz_8(&mut self) -> Option<Vec<usize>> {
        let n = self.read_8();
        let z = self.registers.get_flag_z();
        let signed_n = self.registers.to_signed_byte(n) as isize;
        if z == 0 {
            self.registers.step(signed_n);
        }
        Some(vec![n as usize])
    }

    fn inc_c(&mut self) -> Option<Vec<usize>> {
        self.registers.inc_c();
        // TODO: implement flags
        None
    }
}
