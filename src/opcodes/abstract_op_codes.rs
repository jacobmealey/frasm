// Abstract Op Codes 
// Author: Jacob Mealey <jacobmealey.maine@gmail.com>
//
// Object: 
//  The purpose of this file is to maintain the three types of op codes in 
//  a MIPS architecture (Register, Immediate and Jump) each have a struct 
//  has two member functions, as_bin and new
//  ----- function definitions: -----
//  as_bin() takes no arguments and returns an unsigned 32 bit int
//  that is the binary representation of the given op code
// 
//  I think there must be a way to abstract the as_bin function so that the same
//  one can be used for all three :)
//  
//  new() takes a different amount of arguments depending on the op code
//  and returns the op code as a struct. 

pub struct Register {
    rs : u32,
    rt : u32,
    rd : u32,
    shamt : u32,
    func : u32
}

impl Register {
    pub fn as_bin(&self) -> u32 {
        let mut bin : u32 = 0; 
        // offset buy n bits and filter any bits that shouldn't be high
        bin |= (self.rs & 31) << 21; 
        bin |= (self.rt & 31) << 16;
        bin |= (self.rd & 31) << 11;
        bin |= (self.shamt & 31) << 6;
        bin |= self.func & 63;
        return bin;
    }
    
    // Register::new()
    // rs - source register 1; rt - source register 2, rd desitination register,
    // shamt - shift amount, func - Register function
    pub fn new(rs: u32, rt: u32, rd: u32, shamt: u32, func: u32) -> Register{
        return Register{
            rs: rs,
            rt: rt,
            rd: rd,
            shamt: shamt,
            func: func
        };
    }
}

pub struct Immediate {
    op : u32,
    rs : u32,
    rt : u32,
    i  : u32
}

impl Immediate {
    pub fn as_bin(&self) -> u32 {
        let mut bin : u32 = 0;
        bin |= (self.op & 63) << 26;
        bin |= (self.rs & 31) << 21;
        bin |= (self.rt & 31) << 16;
        bin |= self.i & 65535;
        return bin;
    }

    pub fn new(op: u32, rs: u32, rt: u32, i: u32) -> Immediate{
        return Immediate {
            op: op,
            rs: rs,
            rt: rt,
            i: i
        };
    }
}

pub struct Jump {
    op : u32,
    addr : u32
}

impl Jump {
    pub fn as_bin(&self) -> u32 {
        let mut bin : u32 = 0;
        bin |= (self.op & 63) << 26;
        bin |= self.addr;
        return bin;
    }

    pub fn new(op: u32, addr: u32) -> Jump{
        return Jump {
            op: op,
            addr: addr
        };
    }
}
