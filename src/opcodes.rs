use std::collections::HashMap;
pub mod abstract_op_codes;

pub static I_FORMAT: &'static [(&str, u32)]= &[
    ("addi", 8),
    ("addiu", 9),
    ("slti", 10),
    ("sltiu", 11),
    ("andi", 12),
    ("ori", 13),
    ("xori", 14),
    ("sb", 40),
    ("sh", 41),
    ("sw", 43),
    ("beq", 4),
    ("bne", 5),
    ("lui", 15),
    ("lhu", 37),
    ("lw", 35),
    ("swc1", 57),
    ("swc2", 58),
    ("sdc1", 61),
    ("sdc2", 62),
    ("ldc1", 53),
    ("ldc2", 54),
    ("lwc1", 49),
    ("lwc2", 50),
    ("blez", 6),
    ("bgtz", 7),
    ("TLB", 16),
    ("FLPt", 17),
    ("lb", 32),
    ("lh", 33),
    ("lwl", 34),
    ("lbu", 36),
    ("lwr", 38),
    ("swl", 42),
    ("swr", 46),
    ("cache", 47),
    ("ll", 48),
    ("pref", 51),
    ("sc", 56),
    ("bltz", 1),
];

pub static J_FORMAT: &'static[(&str, u32)] = &[
    ("j", 2),
    ("jal", 3),
];

// this is straight up missing subtraction?? 
// Not sure what number its supposed to be but lol
pub static R_FUNCS: &'static [(&str, u32)]  = &[
    ("ssl", 0),
    ("srl", 2),
    ("sra", 3),
    ("sllv", 4),
    ("srlv", 6),
    ("srav", 7),
    ("jr", 8),
    ("jalr", 9),
    ("movz", 10),
    ("movn", 11),
    ("syscall", 12),
    ("break", 13),
    ("sync", 15),
    ("mfhi", 16),
    ("mthi", 17),
    ("mflo", 18),
    ("mtlo", 19),
    ("mult", 24),
    ("multu", 25),
    ("div", 26),
    ("divu", 27),
    ("add", 32),
    ("addu", 33),
    ("div", 34),
    ("divu", 35),
    ("and", 36),
    ("or", 37),
    ("xor", 38),
    ("nor", 39),
    ("slt", 42),
    ("sltu", 43),
    ("tge", 48),
    ("tgeu", 49),
    ("tlt", 50),
    ("tltu", 51),
    ("teq", 52),
    ("tne", 54)
];


pub fn asm_to_bin(string: &str) -> Result<u32, ()> {
    let split_line: Vec<&str> = string.split(' ').collect();

    let i_format : HashMap<&str, u32> = I_FORMAT.iter().cloned().collect();
    let j_format : HashMap<&str, u32> = J_FORMAT.iter().cloned().collect();
    let r_funcs: HashMap<&str, u32> = R_FUNCS.iter().cloned().collect();

    // Check if split line is an i type op-code
    if i_format.get(split_line[0]) != None {
        println!("{} is I format", split_line[0]);
        let opcode: u32 = *i_format.get(split_line[0]).unwrap();
        let rs: u32 = split_line[1].parse().unwrap();
        let rt: u32 = split_line[2].parse().unwrap();
        let imm: u32 = split_line[3].parse().unwrap();
        let i_form = abstract_op_codes::Immediate::new(opcode, rs, rt, imm);
        return Ok(i_form.as_bin());

    // check if split_line is and r type op code
    } else if r_funcs.get(split_line[0]) != None {
        if split_line.len() == 4 {
            let func: u32 = *r_funcs.get(split_line[0]).unwrap();
            let rd: u32 = split_line[1].parse().unwrap();
            let rs: u32 = split_line[2].parse().unwrap();
            let rt: u32 = split_line[3].parse().unwrap();
            let reg = abstract_op_codes::Register::new(rs, rt, rd, 0, func);
            return Ok(reg.as_bin());
        }
    // LOL must be J
    } else if j_format.get(split_line[0]) != None {
        let opcode: u32 = *j_format.get(split_line[0]).unwrap();
        let addr: u32 = split_line[1].parse().unwrap();
        let j_form = abstract_op_codes::Jump::new(opcode, addr);
        return Ok(j_form.as_bin());
    }
    return Err(());
}