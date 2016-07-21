use std::ops::Sub;
use snowflake::ProcessUniqueId;

#[derive(Debug)]
pub enum Instruction {
    //ARITHMETICAL OPERATION
                                
    Add(Reg, Reg),          //Operation     Reg1 Reg2
                            //0001          xxxx xxxx 000...
    Mul(Reg, Reg),          //Operation     Reg1
                            //0010          xxxx xxxx 000...
    //LOADING AND SAVING
    Ld(Reg, MemAddr), //Operation     Reg       Addr
                            //0011          xxxx 0000 xxx...(52bit)
    Sav(MemAddr, Reg),//Operation     Reg       Addr
                            //0100          xxxx 0000 xxx...(52bit) 
    //STACK
    Push(Reg),              //Operation     Reg
                            //0101          xxxx 000...
    Pop(Reg),               //Operation     Reg
                            //0110          xxxx 000...
    //JUMPS
    Jz(MemAddr),      //Operation               Addr
                            //0111 0001     0000 0000 xxx...(52bit)           
    Jgz(MemAddr),     //Operation               Addr
                            //0111 0010     0000 0000 xxx...(52bit) 
    Jlz(MemAddr),     //Operation               Addr
                            //0111 0011     0000 0000 xxx...(52bit) 

    Nop,                    //Operation
                            //000...(64bit)
}

pub fn op_to_instr(opcode:i64) -> Result<Instruction, ()> {
    match get_get_opcode(opcode as u64) {
        0b0000_0000 => Ok(Instruction::Nop), //Nop
        0b0000_0001 => Ok(Instruction::Add(get_reg1(opcode as u64),get_reg2(opcode as u64))), //Add
        0b0000_0010 => Ok(Instruction::Mul(get_reg1(opcode as u64),get_reg2(opcode as u64))), //Mul
        0b0000_0011 => Ok(Instruction::Ld(get_reg1(opcode as u64), get_addr(opcode as u64))), //Ld
        0b0000_0100 => Ok(Instruction::Sav(get_addr(opcode as u64),get_reg1(opcode as u64))), //Sav
        0b0000_0101 => Ok(Instruction::Push(get_reg1(opcode as u64))), //Push
        0b0000_0110 => Ok(Instruction::Pop(get_reg1(opcode as u64))), //Pop
        0b0000_0111 => { match get_nth_byte(opcode as u64 as u64, 1)  {
                        0x0001 => Ok(Instruction::Jz(get_addr(opcode as u64))),
                        0x0010 => Ok(Instruction::Jgz(get_addr(opcode as u64))),
                        0x0011 => Ok(Instruction::Jlz(get_addr(opcode as u64))),
                             _ => Err(()),
                    }
        }, 
             _ => Err(()),
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


    

#[derive(Debug)]
pub enum Reg {
    EAX,    //0x0001
    EBX,    //0x0010
    ECX,    //0x0011
    EDX,    //0x0100
    ESP,    //0x0101
    EBP,    //0x0110
    ISP,    //0x0111
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

fn get_nth_byte(num:u64, nth:usize) -> u8 {
    let mask =  0x00_00_00_00_00_00_00_ffu64;
    num >> (7-num)*8;
    (mask & num) as u8
}

fn get_reg1(num:u64) -> Reg { //reg1 is always coded into bits [4..7]
    match get_nth_byte(num, 2) & 0xf0 { //reg in 4 most significant bits
        0x10 => Reg::EAX,
        0x20 => Reg::EBX,
        0x30 => Reg::ECX,
        0x40 => Reg::EDX,
        0x50 => Reg::ESP,
        0x60 => Reg::EBP,
        0x70 => Reg::ISP,
             _ => panic!("Unknown register code: {}", num),
    }
}

fn get_reg2(num:u64) -> Reg { //reg2 is always coded into bits [8..11]
    match get_nth_byte(num, 2) & 0x0f {
        0x01 => Reg::EAX,
        0x02 => Reg::EBX,
        0x03 => Reg::ECX,
        0x04 => Reg::EDX,
        0x05 => Reg::ESP,
        0x06 => Reg::EBP,
        0x07 => Reg::ISP,
             _ => panic!("Unknown register code: {}", num),
    }
}

fn get_addr(num:u64) -> MemAddr {
    MemAddr::Addr((num & 0x00_0f_ff_ff_ff_ff_ff_ffu64) as i64)
}


fn get_get_opcode(num:u64)->u8 { //return 0000_xxxx opcode
    (num >> 56) as u8
}

