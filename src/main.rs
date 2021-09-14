mod abstract_op_codes;

fn main() {
    println!("Hello, world!");
    let reg = abstract_op_codes::Register::new(0, 0, 2, 0, 37);
    let imm = abstract_op_codes::Immediate::new(4, 8, 0, 3);
    let jmp = abstract_op_codes::Jump::new(2, 0x100001);
    println!("Register: {:#010x}", reg.as_bin());
    println!("Immediate: {:#010x}", imm.as_bin());
    println!("Jump: {:#010x}", jmp.as_bin());
}
