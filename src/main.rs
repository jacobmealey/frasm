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

    for line in lines {
        line_count += 1;
        // most of this should probably put into a function lol
        let bin_as_line = opcodes::asm_to_bin(line);

        binary.push(bin_as_line);
    }

    for bin in binary {
        println!("{}", bin);
    }
}
