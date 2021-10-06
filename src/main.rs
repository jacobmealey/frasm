mod opcodes;

use std::env;
use std::fs;
            
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


    let mut post_stage1: Vec<&str> = Vec::new();
    // preprocess pipeline
    // stage 1 -- filter comments and trailing whitespace and newlines
    for line in lines {
        let split: Vec<&str> = line.split('#').collect();
        if is_blank(split[0]) {                 // empty line
            continue;
        } else if split.len() > 1 {             // ends in a comment
            post_stage1.push(split[0].trim());
        }else{                                  // regular line
            post_stage1.push(split[0].trim());
        }
    }  
    
    for line in &post_stage1 {
        println!("{}", line);
    }
    // stage 2 -- convert labels to hex values

    // stage 3 -- convert symbolic registers to integer values (no dollar signs)

    // This is for already processed lines. not lines can have comments
    // no lines can have symbolic names 
    // valid lines:
    //      add 3 2 1
    // where those are the values of the registers. or in an I type
    // it would be the calcalated immediate value
    // invalid lines:
    //      add $t0 $v1 $v2 # adding or something
    for line in post_stage1{
        line_count += 1;
        let line_as_bin = opcodes::asm_to_bin(line);

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
        println!("{}", bin);
    }
}
