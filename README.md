# frasm
MIPS assembler written in Rust

## About
frasm is an assembler written in Rust speicifally for the MIPs architecture. This is my first time writing anything bigger than a 
`Hello, World!` in Rust. As of right now (Oct 2021) it is a relatively dumb assembler that is still being developed.

I am building this project for the purpose of learning rust, I have done little toy examples and hello worlds but I have yet to sit down and 
build out a project with it yet. 

to see whats being worked on visit the [projects](https://github.com/jacobmealey/frasm/projects/1) page. 

## Building and Running
frasm is built in rust and uses cargo for building and package management. 

To build it you just run: `cargo build` 

To run: `./target/debug/frasm /path/to/file.asm` 

To build and run: `cargo run /path/to/file.asm`. 

As of right now it takes one command line argument which is the assembly file. 
