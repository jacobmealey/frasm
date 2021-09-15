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
    println!("Hello, world!");
    let reg = abstract_op_codes::Register::new(0, 0, 2, 0, 37);
    let imm = abstract_op_codes::Immediate::new(4, 8, 0, 3);
    let jmp = abstract_op_codes::Jump::new(2, 0x100001);
    println!("Register: {:#010x}", reg.as_bin());
    println!("Immediate: {:#010x}", imm.as_bin());
    println!("Jump: {:#010x}", jmp.as_bin());
    println!("{}", opcodes.get("j").unwrap());
}
