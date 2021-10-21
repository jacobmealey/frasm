extern crate regex;
mod opcodes;

use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
            
// macro for checking if a string is blank?
fn is_blank(string: &str) -> bool {
    let mut count = 0;
    for c in string.chars() {
        if c.is_whitespace() {
            count += 1;
        }
    }
    return count == string.len()
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading: {}", filename);
    let file_string = fs::read_to_string(filename)
        .expect("Couldn't read file");
    
    let lines: Vec<&str> = file_string.split('\n').collect();

    let mut binary: Vec<u32> = Vec::new();
    // Traverse through all lines in the file!
    let mut line_count = 0;


    // preprocess pipeline
    // stage 1 -- filter comments and trailing whitespace and newlines
    let mut post_stage1: Vec<&str> = Vec::new();
    for line in &lines {
        let split: Vec<&str> = line.split('#').collect();
        if is_blank(split[0]) {                 // empty line
            continue;
        } else if split.len() > 1 {             // ends in a comment
            post_stage1.push(split[0].trim());
        }else{                                  // regular line
            post_stage1.push(split[0].trim());
        }
    }  
    
    // stage 2 -- generate list of labels 
    let mut post_stage2: Vec<&str> = Vec::new();
    let mut labels: HashMap<&str, u32> = HashMap::new();
    for line in &post_stage1 {
        let split: Vec<&str> = line.split(':').collect();
        line_count = line_count + 1;
        if split.len() > 1 {                    // line starts with a branch
            labels.insert(split[0].trim(), line_count);
            post_stage2.push(split[1].trim());
        } else {                                // Does not start with a branch
            post_stage2.push(split[0].trim());
        }

    }
    
    println!("post_stage2 len: {}", post_stage2.len());
    println!("label count: {}", labels.keys().len());

    // Go through and replace all the labels.
    let mut stage_2_post_labels: Vec<String> = Vec::new();
    for label in labels.keys() {
        let rg = Regex::new(&format!(r"{}", label)).unwrap();
        let mut replaced_line: String;
        for lines in &post_stage2 {
            replaced_line = rg.replace(lines, labels[label].to_string()).to_string();
            println!("{}", replaced_line);
            stage_2_post_labels.push(replaced_line);
        }
    }

    // stage 3 -- convert symbolic registers to integer values (no dollar signs)
    // first thing we want to do is filter out any $ before the register 
    // second we want to convert any of the symbolic names to the decimal value
    let registersLookUp: HashMap<&str, u32> = [
        ("zero", 0),
        ("at", 1),
        ("v0", 2),
        ("v1", 3),
        ("a0", 4),
        ("a1", 5),
        ("a2", 6),
        ("a3", 7),
        ("t0", 8),
        ("t1", 9),
        ("t2", 10),
        ("t3", 11),
        ("t4", 12),
        ("t5", 13),
        ("t6", 14),
        ("t7", 15),
        ("s0", 16),
        ("s1", 17),
        ("s2", 18),
        ("s3", 19),
        ("s4", 20),
        ("s5", 21),
        ("s6", 22),
        ("s7", 23),
        ("t8", 24),
        ("t9", 25),
        ("k0", 26),
        ("k1", 27),
        ("gp", 28),
        ("sp", 29),
        ("fp", 30),
        ("ra", 31),
    ].iter().cloned().collect();
    // This is for already processed lines. not lines can have comments
    // no lines can have symbolic names 
    // valid lines:
    //      add 3 2 1
    // where those are the values of the registers. or in an I type
    // it would be the calcalated immediate value
    // invalid lines:
    //      add $t0 $v1 $v2 # adding or something
    for line in stage_2_post_labels{
        line_count += 1;
        let line_as_bin = opcodes::asm_to_bin(&line);

        match line_as_bin{
            Ok(bin) => binary.push(bin),
            Err(e) => {
                if e == opcodes::ERR_UNKNOWN_OP {
                    println!("Unknown operator on line {}\n\t {}", line_count, line);
                    std::process::exit(1);
                }else if e == opcodes::ERR_MISMATCH_PARAM_N{
                    println!("Mismatch paramater count on line {}:\n\t {}", 
                        line_count, line);
                    std::process::exit(1);
                }
            }
        }
    }

    for bin in binary {
        println!("{:#010x}", bin);
    }
}
