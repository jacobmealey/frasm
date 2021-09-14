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
        // offset buy n bits and cutoff anything over 5 bits
        bin |= (self.rs & 31) << 21; 
        bin |= (self.rt & 31) << 16;
        bin |= (self.rd & 31) << 11;
        bin |= (self.shamt & 31) << 6;
        bin |= self.func & 63;
        return bin;
    }
    
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

