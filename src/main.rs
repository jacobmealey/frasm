mod abstract_op_codes;
mod opcodes;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading: {}", filename);
    let file_string = fs::read_to_string(filename)
        .expect("Couldn't read file");
    
    let lines: Vec<&str> = file_string.split('\n').collect();

    let mut binary: Vec<u32> = Vec::new();

    let i_format : HashMap<&str, u32> = opcodes::I_FORMAT.iter().cloned().collect();
    let j_format : HashMap<&str, u32> = opcodes::J_FORMAT.iter().cloned().collect();
    let r_funcs: HashMap<&str, u32> = opcodes::R_FUNCS.iter().cloned().collect();

    // Traverse through all lines in the file!
    let mut line_count = 0;

    for line in lines {
        line_count += 1;
        // most of this should probably put into a function lol
        let split_line: Vec<&str> = line.split(' ').collect();
        let mut bin_as_line: u32 = 0;
        // Check if split line is an i type op-code
        if i_format.get(split_line[0]) != None {
            println!("{} is I format", split_line[0]);
            let opcode: u32 = *i_format.get(split_line[0]).unwrap();
            let rs: u32 = split_line[1].parse().unwrap();
            let rt: u32 = split_line[2].parse().unwrap();
            let imm: u32 = split_line[3].parse().unwrap();
            let i_form = abstract_op_codes::Immediate::new(opcode, rs, rt, imm);
            bin_as_line = i_form.as_bin();

        // check if split_line is and r type op code
        } else if r_funcs.get(split_line[0]) != None {
            if split_line.len() == 4 {
                let func: u32 = *r_funcs.get(split_line[0]).unwrap();
                let rd: u32 = split_line[1].parse().unwrap();
                let rs: u32 = split_line[2].parse().unwrap();
                let rt: u32 = split_line[3].parse().unwrap();
                let reg = abstract_op_codes::Register::new(rs, rt, rd, 0, func);
                bin_as_line = reg.as_bin();
            }
        // LOL must be J
        } else if j_format.get(split_line[0]) != None {
            let opcode: u32 = *j_format.get(split_line[0]).unwrap();
            let addr: u32 = split_line[1].parse().unwrap();
            let j_form = abstract_op_codes::Jump::new(opcode, addr);
            bin_as_line = j_form.as_bin();
        } else {
            println!("Error in line {};\n \t{} \n exiting.", line_count, line);
            process::exit(1);
        }

        binary.push(bin_as_line);
    }

    for bin in binary {
        println!("{}", bin);
    }
}
