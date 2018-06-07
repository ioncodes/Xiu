use flags::Flags;

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

pub struct Registers {
    pub af: AF,
    pub bc: BC,
    pub de: DE,
    pub hl: HL,
    pub sp: u16,
    pub pc: u16,
}

pub struct AFPair {
    pub a: u8,
    pub f: u8,
}

pub struct BCPair {
    pub b: u8,
    pub c: u8,
}

pub struct DEPair {
    pub d: u8,
    pub e: u8,
}

pub struct HLPair {
    pub h: u8,
    pub l: u8,
}

pub union AF {
    pub pair: AFPair,
    pub af: u16,
}

pub union BC {
    pub pair: BCPair,
    pub bc: u16,
}

pub union DE {
    pub pair: DEPair,
    pub de: u16,
}

pub union HL {
    pub pair: HLPair,
    pub hl: u16,
}

#[allow(dead_code)]
impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: AF {
                pair: AFPair { a: 0, f: 0 },
            },
            bc: BC {
                pair: BCPair { b: 0, c: 0 },
            },
            de: DE {
                pair: DEPair { d: 0, e: 0 },
            },
            hl: HL {
                pair: HLPair { h: 0, l: 0 },
            },
            sp: 0,
            pc: 0,
        }
    }

    pub fn get_af(&mut self) -> u16 {
        unsafe {
            self.af.af
        }
    }

    pub fn get_a(&self) -> u8 {
        unsafe {
            self.af.pair.a
        }
    }

    pub fn get_f(&self) -> u8 {
        unsafe {
            self.af.pair.f
        }
    }

    pub fn get_bc(&mut self) -> u16 {
        unsafe {
            self.bc.bc
        }
    }

    pub fn get_b(&self) -> u8 {
        unsafe {
            self.bc.pair.b
        }
    }

    pub fn get_c(&self) -> u8 {
        unsafe {
            self.bc.pair.c
        }
    }

    pub fn get_de(&mut self) -> u16 {
        unsafe {
            self.de.de
        }
    }

    pub fn get_d(&self) -> u8 {
        unsafe {
            self.de.pair.d
        }
    }

    pub fn get_e(&self) -> u8 {
        unsafe {
            self.de.pair.e
        }
    }

    pub fn get_hl(&mut self) -> u16 {
        unsafe {
            self.hl.hl
        }
    }

    pub fn get_h(&self) -> u8 {
        unsafe {
            self.hl.pair.h
        }
    }

    pub fn get_l(&self) -> u8 {
        unsafe {
            self.hl.pair.l
        }
    }

    pub fn set_af(&mut self, data: u16) {
        self.af.af = data;
    }

    pub fn set_a(&mut self, byte: u8) {
        unsafe {
            self.af.pair.a = byte;
        }
    }

    pub fn set_f(&mut self, byte: u8) {
        unsafe {
            self.af.pair.f = byte;
        }
    }

    pub fn set_bc(&mut self, data: u16) {
        self.bc.bc = data;
    }

    pub fn set_b(&mut self, byte: u8) {
        unsafe {
            self.bc.pair.b = byte;
        }
    }

    pub fn set_c(&mut self, byte: u8) {
        unsafe {
            self.bc.pair.c = byte;
        }
    }

    pub fn set_de(&mut self, data: u16) {
        self.de.de = data;
    }

    pub fn set_d(&mut self, byte: u8) {
        unsafe {
            self.de.pair.d = byte;
        }
    }

    pub fn set_e(&mut self, byte: u8) {
        unsafe {
            self.de.pair.e = byte;
        }
    }

    pub fn set_hl(&mut self, data: u16) {
        self.hl.hl = data;
    }

    pub fn set_h(&mut self, byte: u8) {
        unsafe {
            self.hl.pair.h = byte;
        }
    }

    pub fn set_l(&mut self, byte: u8) {
        unsafe {
            self.hl.pair.l = byte;
        }
    }

    pub fn dec_hl(&mut self) {
        unsafe {
            self.hl.hl -= 1;
        }
    }

    pub fn inc_c(&mut self) {
        unsafe {
            self.bc.pair.c += 1;
        }
    }

    pub fn set_bit(&mut self, byte: u8, n: u8) -> u8 {
        byte | 1 << n
    }

    pub fn clear_bit(&mut self, byte: u8, n: u8) -> u8 {
        byte & !(1 << n)
    }

    pub fn set_flag_z(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.set_bit(f, Flags::Z as u8);
        }
    }

    pub fn clear_flag_z(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.clear_bit(f, Flags::Z as u8);
        }
    }

    pub fn set_flag_n(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.set_bit(f, Flags::N as u8);
        }
    }

    pub fn clear_flag_n(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.clear_bit(f, Flags::N as u8);
        }
    }

    pub fn set_flag_h(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.set_bit(f, Flags::H as u8);
        }
    }

    pub fn clear_flag_h(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.clear_bit(f, Flags::H as u8);
        }
    }

    pub fn set_flag_c(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.set_bit(f, Flags::C as u8);
        }
    }

    pub fn clear_flag_c(&mut self) {
        let f = self.get_f();
        unsafe {
            self.af.pair.f = self.clear_bit(f, Flags::C as u8);
        }
    }

    pub fn get_bit(&self, byte: u8, bit: u8) -> u8 {
        (byte & ( 1 << bit )) >> bit
    }

    pub fn get_flag_z(&self) -> u8 {
        let f = self.get_f();
        self.get_bit(f, Flags::Z as u8)
    }

    pub fn get_flag_n(&self) -> u8 {
        let f = self.get_f();
        self.get_bit(f, Flags::N as u8)
    }

    pub fn get_flag_h(&self) -> u8 {
        let f = self.get_f();
        self.get_bit(f, Flags::H as u8)
    }

    pub fn get_flag_c(&self) -> u8 {
        let f = self.get_f();
        self.get_bit(f, Flags::C as u8)
    }

    pub fn to_signed_byte(&self, byte: u8) -> i8 {
        -((256 + (!byte + 1)) as i8)
    }

    pub fn step(&mut self, length: isize) {
        self.pc = (self.pc as isize + length) as u16;
    }

    pub fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn dump(&mut self) {
        println!("A:  ${:02x}", self.get_a());
        println!("B:  ${:02x}", self.get_b());
        println!("C:  ${:02x}", self.get_c());
        println!("D:  ${:02x}", self.get_d());
        println!("E:  ${:02x}", self.get_e());
        println!("F:  ${:02x}", self.get_f());
        println!("H:  ${:02x}", self.get_h());
        println!("L:  ${:02x}", self.get_l());
        println!("AF: ${:04x}", self.get_af());
        println!("BC: ${:04x}", self.get_bc());
        println!("DE: ${:04x}", self.get_de());
        println!("HL: ${:04x}", self.get_hl());
        println!("PC: ${:04x}", self.pc);
        println!("SP: ${:04x}", self.sp);
        println!("ZNHC3210");
        println!("{}{}{}{}{}{}{}{}",
            self.get_flag_z(),
            self.get_flag_n(),
            self.get_flag_h(),
            self.get_flag_c(),
            self.get_bit(self.get_f(), 3),
            self.get_bit(self.get_f(), 2),
            self.get_bit(self.get_f(), 1),
            self.get_bit(self.get_f(), 0)
        );
    }
}
