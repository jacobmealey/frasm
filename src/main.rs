mod abstract_op_codes;
mod opcodes;

use std::collections::HashMap;


fn main() {
    let line = "j 2 2 4";
    let i_format : HashMap<&str, u32> = opcodes::I_FORMAT.iter().cloned().collect();
    let j_format : HashMap<&str, u32> = opcodes::J_FORMAT.iter().cloned().collect();
    let r_funcs: HashMap<&str, u32> = opcodes::R_FUNCS.iter().cloned().collect();

    let split_line: Vec<&str> = line.split(' ').collect();

    if i_format.get(split_line[0]) != None {
        println!("{} is I format", split_line[0]);
        let opcode: u32 = *i_format.get(split_line[0]).unwrap();
        let rs: u32 = split_line[1].parse().unwrap();
        let rt: u32 = split_line[2].parse().unwrap();
        let imm: u32 = split_line[3].parse().unwrap();
        let i_form = abstract_op_codes::Immediate::new(opcode, rs, rt, imm);
        println!("{} --> {:#010x}", line, i_form.as_bin());

    } else if r_funcs.get(split_line[0]) != None {
        if split_line.len() == 4 {
            let func: u32 = *r_funcs.get(split_line[0]).unwrap();
            let rd: u32 = split_line[1].parse().unwrap();
            let rs: u32 = split_line[2].parse().unwrap();
            let rt: u32 = split_line[3].parse().unwrap();
            let reg = abstract_op_codes::Register::new(rs, rt, rd, 0, func);
            println!("{} --> {:#010x}", line, reg.as_bin());
        }
    } else {
        let opcode: u32 = *j_format.get(split_line[0]).unwrap();
        let addr: u32 = split_line[1].parse().unwrap();
        let j_form = abstract_op_codes::Jump::new(opcode, addr);
        println!("{} --> {:#010x}", line, j_form.as_bin());
    }
}
