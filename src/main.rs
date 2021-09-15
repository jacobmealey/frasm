mod abstract_op_codes;
use std::collections::HashMap;



fn main() {
    let opcodes : HashMap<&str, u32> = [
        ("bltz", 1),
        ("j", 2),
        ("jal", 3),
        ("beq", 4),
        ("bne", 5),
        ("blez", 6),
        ("bgtz", 7),
        ("addi", 8),
        ("addiu", 9),
        ("slti", 10),
        ("sltiu", 11),
        ("andi", 12),
        ("ori", 13),
        ("xori", 14),
        ("lui", 15),
        ("TLB", 16),
        ("FLPt", 17),
        ("lb", 32),
        ("lh", 33),
        ("lwl", 34),
        ("lw", 35),
        ("lbu", 36),
        ("lhu", 37),
        ("lwr", 38),
        ("sb", 40),
        ("sh", 41),
        ("swl", 42),
        ("sw", 43),
        ("swr", 46),
        ("cache", 47),
        ("ll", 48),
        ("lwc1", 49),
        ("lwc2", 50),
        ("pref", 51),
        ("ldc1", 53),
        ("ldc2", 54),
        ("sc", 56),
        ("swc1", 57),
        ("swc2", 58),
        ("sdc1", 61),
        ("sdc2", 62)
    ].iter().cloned().collect();

    let r_funcs : HashMap<&str, u32> = [
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
    ].iter().cloned().collect();

    let line = "add 1 2 2";

    let split_line: Vec<&str> = line.split(' ').collect();

    if opcodes.get(split_line[0]) != None {
        println!("{} is opcode", split_line[0]);
    }else{
        println!("{} is R func", split_line[0]);
        if split_line.len() == 4 {
            let func: u32 = *r_funcs.get(split_line[0]).unwrap();
            let rd: u32 = split_line[1].parse().unwrap();
            let rs: u32 = split_line[2].parse().unwrap();
            let rt: u32 = split_line[3].parse().unwrap();
            let reg = abstract_op_codes::Register::new(rs, rt, rd, 0, func);
            println!("Register: {:#010x}", reg.as_bin());
        }
    }


    println!("Hello, world!");
    let reg = abstract_op_codes::Register::new(0, 0, 2, 0, 37);
    let imm = abstract_op_codes::Immediate::new(4, 8, 0, 3);
    let jmp = abstract_op_codes::Jump::new(2, 0x100001);
    println!("Register: {:#010x}", reg.as_bin());
    println!("Immediate: {:#010x}", imm.as_bin());
    println!("Jump: {:#010x}", jmp.as_bin());
    println!("{}", opcodes.get("j").unwrap());
    println!("{}", r_funcs.get("add").unwrap());
}
