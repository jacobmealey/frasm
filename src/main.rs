struct Register {
    rs : u32,
    rt : u32,
    rd : u32,
    shamt : u32,
    func : u32
}

impl Register {
    fn as_bin(&self) -> u32 {
        let mut bin : u32 = 0; 
        // offset buy n bits and cutoff anything over 5 bits
        bin |= (self.rs & 31) << 21; 
        bin |= (self.rt & 31) << 16;
        bin |= (self.rd & 31) << 11;
        bin |= (self.shamt & 31) << 6;
        bin |= self.func & 63;
        return bin;
    }
}

struct Immediate {
    op : u32,
    rs : u32,
    rt : u32,
    i  : u32
}

impl Immediate {
    fn as_bin(&self) -> u32 {
        let mut bin : u32 = 0;
        bin |= (self.op & 63) << 26;
        bin |= (self.rs & 31) << 21;
        bin |= (self.rt & 31) << 16;
        bin |= self.i & 65535;
        return bin;
    }
}

struct Jump {
    op : u32,
    addr : u32
}

impl Jump {
    fn as_bin(&self) -> u32 {
        let mut bin : u32 = 0;
        bin |= (self.op & 63) << 26;
        bin |= self.addr;
        return bin;
    }
}

fn main() {
    println!("Hello, world!");
    let reg = Register {
        rs: 0,
        rt: 0,
        rd: 2,
        shamt: 0,
        func: 37
    };

    let imm = Immediate {
        op: 4,
        rs: 8,
        rt: 0,
        i: 3
    };
    println!("Register: {:#010x}", reg.as_bin());
    println!("Immediate: {:#010x}", imm.as_bin());
}
