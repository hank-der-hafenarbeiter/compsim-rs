use std::ops::Sub;
use snowflake::ProcessUniqueId;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    //ARITHMETICAL OPERATION
                                
    Add(Reg, Reg),          //Operation     
                            //1R_R0_00_00_00_00_00_00
    Mul(Reg, Reg),          //Operation
                            //2R_R0_00_00_00_00_00_00
    //LOADING AND SAVING
    Ld(Reg, MemAddr),       //Operation
                            //3R_0A_AA_AA_AA_AA_AA_AA
    Sav(MemAddr, Reg),      //Operation 
                            //4R_0A_AA_AA_AA_AA_AA_AA
    //STACK
    Push(Reg),              //Operation
                            //5R_00_00_00_00_00_00_00
    Pop(Reg),               //Operation
                            //6R_00_00_00_00_00_00_00
    //JUMPS
    Jz(MemAddr),            //Operation
                            //71_0A_AA_AA_AA_AA_AA_AA
    Jgz(MemAddr),           //Operation
                            //72_0A_AA_AA_AA_AA_AA_AA
    Jlz(MemAddr),           //Operation
                            //73_0A_AA_AA_AA_AA_AA_AA

    Nop,                    //Operation
                            //00_00_00_00_00_00_00_00
}

pub fn op_to_instr(opcode:i64) -> Result<Instruction, String> {
    match get_opcode(opcode) {
        0x00 => Ok(Instruction::Nop), //Nop
        0x01 => Ok(Instruction::Add(get_reg1(opcode),get_reg2(opcode))), //Add
        0x02 => Ok(Instruction::Mul(get_reg1(opcode ),get_reg2(opcode ))), //Mul
        0x03 => Ok(Instruction::Ld(get_reg1(opcode ), MemAddr::Addr(opcode & 0x00_0f_ff_ff_ff_ff_ff_ff))), //Ld
        0x04 => Ok(Instruction::Sav(get_addr(opcode ),get_reg1(opcode ))), //Sav
        0x05 => Ok(Instruction::Push(get_reg1(opcode ))), //Push
        0x06 => Ok(Instruction::Pop(get_reg1(opcode ))), //Pop
        0x07 => match (opcode & 0xff_00_00_00_00_00_00_00i64) >> 56 {
            0x71 => Ok(Instruction::Jz(MemAddr::Addr(opcode & 0x00_0f_ff_ff_ff_ff_ff_ff))),
            0x72 => Ok(Instruction::Jgz(MemAddr::Addr(opcode & 0x00_0f_ff_ff_ff_ff_ff_ff))),
            0x73 => Ok(Instruction::Jlz(MemAddr::Addr(opcode & 0x00_0f_ff_ff_ff_ff_ff_ff))),
            op => Err("Unkown Jump!!: ".to_string() + &(opcode & 0xff_00_00_00_00_00_00_00i64).to_string()),
        }, 
        op => Err("Unkown Instruction: ".to_string() + &op.to_string()),
    }
}

pub fn instruction_to_opcode(inst:Instruction) -> Result<i64, String> {
    match inst {
        Instruction::Nop                => Ok(0),
        Instruction::Add(reg1, reg2)    => Ok(0x10_00_00_00_00_00_00i64 | encode_reg_n(reg1, 0) | encode_reg_n(reg2, 1)),
        Instruction::Mul(reg1, reg2)    => Ok(0x20_00_00_00_00_00_00i64 | encode_reg_n(reg1, 0) | encode_reg_n(reg2, 1)),
        Instruction::Ld(reg1, addr)     => Ok(0x30_00_00_00_00_00_00i64 | encode_reg_n(reg1, 0) | encode_addr(addr)),
        Instruction::Sav(addr, reg)     => Ok(0x40_00_00_00_00_00_00i64 | encode_addr(addr) | encode_reg_n(reg, 0)),
        Instruction::Push(reg)          => Ok(0x50_00_00_00_00_00_00i64 | encode_reg_n(reg, 0)),
        Instruction::Pop(reg)           => Ok(0x60_00_00_00_00_00_00i64 | encode_reg_n(reg, 0)),
        Instruction::Jz(addr)           => Ok(0x71_00_00_00_00_00_00i64 | encode_addr(addr)),
        Instruction::Jgz(addr)          => Ok(0x72_00_00_00_00_00_00i64 | encode_addr(addr)),
        Instruction::Jlz(addr)          => Ok(0x73_00_00_00_00_00_00i64 | encode_addr(addr)),
    }
}




#[derive(Copy,Clone, PartialOrd, PartialEq, Debug)]
pub enum MemAddr {
    Addr(i64),
    Nullptr,
}

impl Sub for MemAddr {
    type Output = MemAddr;

    fn sub(self, _rhs: MemAddr) -> MemAddr {
        if let MemAddr::Addr(lhs) = self {
            if let MemAddr::Addr(rhs) = _rhs {
                return MemAddr::Addr(lhs-rhs)
            }
        }
        return MemAddr::Nullptr;

    }
}


    

#[derive(Copy, Clone,PartialEq, Debug)]
pub enum Reg {
    EAX,    //0x01
    EBX,    //0x02
    ECX,    //0x03
    EDX,    //0x04
    ESP,    //0x05
    EBP,    //0x06
    ISP,    //0x07
}

#[derive(Debug)]
pub enum CPUBusOp {
    RequestBlock(MemAddr, usize),
    GiveBlock(Vec<(MemAddr, i64)>),
    Sleep,
    WakeUp,
    ExecAt(MemAddr),
    Error(String),
}

#[derive(Debug)]
pub enum MemBusOp {
    RequestBlock(ProcessUniqueId, MemAddr, usize),
    GiveBlock(ProcessUniqueId, Vec<(MemAddr, i64)>),
    Error(String)
}

pub fn get_nth_byte(num:i64, nth:usize) -> i8 {
    let mask =  0x00_00_00_00_00_00_00_ffi64;
    let shifted = num >> (7-nth)*8;
    (mask & shifted) as i8
}

fn get_reg1(num:i64) -> Reg { //reg1 is always coded into bits [4..7]
    match get_nth_byte(num, 0) & 0x0f { //reg in 4 most significant bits
        0x01 => Reg::EAX,
        0x02 => Reg::EBX,
        0x03 => Reg::ECX,
        0x04 => Reg::EDX,
        0x05 => Reg::ESP,
        0x06 => Reg::EBP,
        0x07 => Reg::ISP,
        c    => panic!("Unknown register code2: {}", c),
    }
}

fn get_reg2(num:i64) -> Reg { //reg2 is always coded into bits [8..11]
    println!("get_reg1({:#x})", num);
    println!("get_nth_byte(num, 1) & 0xf0 = {:#x}",get_nth_byte(num, 1) & 0xf0 );
    match get_nth_byte(num, 1) & 0xf0 {
        0x10 => Reg::EAX,
        0x20 => Reg::EBX,
        0x30 => Reg::ECX,
        0x40 => Reg::EDX,
        0x50 => Reg::ESP,
        0x60 => Reg::EBP,
        0x70 => Reg::ISP,
        c    => panic!("Unknown register code2: {}", c),
    }
}

fn encode_reg_n(_reg:Reg, n:usize) -> i64 {
    match _reg {
        Reg::EAX => 0x01_00_00_00_00_00_00_00i64 >> 4*n, 
        Reg::EBX => 0x02_00_00_00_00_00_00_00i64 >> 4*n,
        Reg::ECX => 0x03_00_00_00_00_00_00_00i64 >> 4*n,
        Reg::EDX => 0x04_00_00_00_00_00_00_00i64 >> 4*n,
        Reg::ESP => 0x05_00_00_00_00_00_00_00i64 >> 4*n,
        Reg::EBP => 0x06_00_00_00_00_00_00_00i64 >> 4*n,
        Reg::ISP => 0x07_00_00_00_00_00_00_00i64 >> 4*n,
    }
}

fn encode_addr(_addr:MemAddr) -> i64 {
    if let MemAddr::Addr(addr_as_i64) = _addr {
        addr_as_i64 >> 12  //only lower 52 bit of opcodes can be used for addressing
    } else {
        0 //MemAddr::Nullptr
    }
}




fn get_addr(num:i64) -> MemAddr {
    MemAddr::Addr((num & 0x00_0f_ff_ff_ff_ff_ff_ffi64) )
}


fn get_opcode(num:i64)-> i8 { //return 0000_xxxx opcode
    (num >> 60) as i8
}

