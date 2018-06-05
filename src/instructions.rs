pub enum Instructions {
    LDSP16,
    Unknown
}

pub static instructions: [(u8, &'static str, &'static str, Instructions); 1] = [
    (0x31, "LD SP, d16", "LD SP, ${}", Instructions::LDSP16),
];

fn find_instruction(instr: u8) -> &'static (u8, &'static str, &'static str, Instructions) {
    for instruction in instructions.iter() {
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

pub fn get_debug(instr: u8, data: Vec<usize>) -> String {
    let i = find_instruction(instr);
    let mut val = String::from(i.2);
    for d in data {
        val = val.replacen("{}", &format!("{:x}", d), 1)
    }
    val
}

pub fn get_instruction(instr: u8) -> &'static Instructions {
    let i = find_instruction(instr);
    &i.3
}
