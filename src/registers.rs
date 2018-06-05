pub struct Registers {
    pub af: AF,
    pub bc: BC,
    pub de: DE,
    pub hl: HL,
    pub flags: u8,
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
            flags: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn get_af(&mut self) -> &u16 {
        unsafe {
            &self.af.af
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

    pub fn get_bc(&mut self) -> &u16 {
        unsafe {
            &self.bc.bc
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

    pub fn get_de(&mut self) -> &u16 {
        unsafe {
            &self.de.de
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

    pub fn get_hl(&mut self) -> &u16 {
        unsafe {
            &self.hl.hl
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

    pub fn step(&mut self, length: usize) {
        self.pc += length as u16;
    }
}
