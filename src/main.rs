mod opcodes;

use std::env;
use std::fs;

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
    // stage 1 -- filter comments
    
    // stage 2 -- convert labels to hex values

    // stage 3 -- convert symbolic registers to integer values (no dollar signs)

    // This is for already processed lines. not lines can have comments
    // no lines can have symbolic names 
    // valid lines:
    //      add 3 2 1
    // where those are the values of the registers. or in an I type
    // it would be the calcalated immediate value
    // invalid lines:
    //      add $t0 $v1 $v2 ; adding or something
    for line in lines {
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
