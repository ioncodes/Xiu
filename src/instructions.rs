// http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
// http://www.devrs.com/gb/files/GBCPU_Instr.html
// http://www.devrs.com/gb/files/opcodes.html

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Instructions {
    LD_SP_D16,
    LD_HL_D16,
    LD_HLD_A,
    XOR_A,
    Prefixed,
    BIT_7_H,
    JR_NZ_8,
    LD_C_D8,
    Unknown
}

pub static INSTRUCTIONS: [(u8, &'static str, &'static str, Instructions); 7] = [
    (0xcb, "", "", Instructions::Prefixed),
    (0x21, "LD HL, d16", "LD HL, ${}", Instructions::LD_HL_D16),
    (0x31, "LD SP, d16", "LD SP, ${}", Instructions::LD_SP_D16),
    (0x32, "LD (HL-), A", "LD (HL-), A", Instructions::LD_HLD_A),
    (0xaf, "XOR A", "XOR A", Instructions::XOR_A),
    (0x20, "JR NZ, r8", "JR NZ, ${}", Instructions::JR_NZ_8),
    (0x0e, "LD C, d8", "LD C, ${}", Instructions::LD_C_D8)
];

pub static PREFIXED: [(u8, &'static str, &'static str, Instructions); 1] = [
    (0x7c, "BIT 7, H", "BIT 7, H", Instructions::BIT_7_H),
];

fn find_instruction(instr: u8) -> &'static (u8, &'static str, &'static str, Instructions) {
    for instruction in INSTRUCTIONS.iter() {
        if instr == instruction.0 {
            return instruction;
        }
    }
    &(0x00, "", "", Instructions::Unknown)
}

fn find_prefixed_instruction(instr: u8) -> &'static (u8, &'static str, &'static str, Instructions) {
    for instruction in PREFIXED.iter() {
        if instr == instruction.0 {
            return instruction;
        }
    }
    &(0x00, "", "", Instructions::Unknown)
}

pub fn get_assembly(instr: u8) -> String {
    let i = find_instruction(instr);
    String::from(i.1)
}

pub fn get_prefixed_assembly(instr: u8) -> String {
    let i = find_prefixed_instruction(instr);
    String::from(i.1)
}

pub fn get_debug(instr: u8, data: Vec<usize>) -> String {
    let i = find_instruction(instr);
    let mut val = String::from(i.2);
    for d in data {
        val = val.replacen("{}", &format!("{:02x}", d), 1)
    }
    val
}

pub fn get_prefixed_debug(instr: u8, data: Vec<usize>) -> String {
    let i = find_prefixed_instruction(instr);
    let mut val = String::from(i.2);
    for d in data {
        val = val.replacen("{}", &format!("{:02x}", d), 1)
    }
    val
}

pub fn get_instruction(instr: u8) -> &'static Instructions {
    let i = find_instruction(instr);
    &i.3
}

pub fn get_prefixed_instruction(instr: u8) -> &'static Instructions {
    let i = find_prefixed_instruction(instr);
    &i.3
}
