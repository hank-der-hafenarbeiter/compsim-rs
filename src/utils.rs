
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

pub fn op_to_instr(op_code:i64) -> Instruction {
    match get_nth_byte(opcode, 0) {
        0x0000 => Instruction::Nop, //Nop
        0x0001 => Instruction::Add(get_reg1(opcode),get_reg2(opcode)), //Add
        0x0010 => Instruction::Mul(get_reg1(opcode),get_reg2(opcode)), //Mul
        0x0011 => Instruction::Ld(get_reg1(opcode), get_addr(opcode)), //Ld
        0x0100 => Instruction::Sav(get_addr(opcode),get_reg1(opcode)), //Sav
        0x0101 => Instruction::Push(get_reg1(opcode)), //Push
        0x0110 => Instruction::Pop(get_reg1(opcode)), //Pop
        0x0111 => { match get_nth_byte(opcode as u64, 1)  {
                        0x0001 => Instruction::Jz(get_addr(opcode)),
                        0x0010 => Instruction::Jgz(get_addr(opcode)),
                        0x0011 => Instruction::Jlz(get_addr(opcode)),
                             _ => panic!("Unrecognized jump: {}", opcode),
                    }
        }, 
             _ => panic!("Unrecognized instruction: {}", opcode),
    }
}

pub enum MemAddr {
    Addr(i64),
    Nullptr,
}

pub enum Reg {
    EAX,    //0x0001
    EBX,    //0x0010
    ECX,    //0x0011
    EDX,    //0x0100
    ESP,    //0x0101
    EBP,    //0x0110
    ISP,    //0x0111
}

pub enum CPUBusOP {
    RequestBlock(MemAddr, usize),
    GiveBlock(Vector<MemAddr, usize>),
    Error(String),
}

fn get_nth_byte(num:u64, nth:usize) -> u8 {
    let mask =  0x00_00_00_00_00_00_00_ffi64;
    num >> (7-num)*8;
    (mask && num) as u8
}

fn get_reg1(num:u64) -> Reg { //reg1 is always coded into bits [4..7]
    match get_nth_byte(num, 2) {
        0x0001 => Reg::EAX,
        0x0010 => Reg::EBX,
        0x0011 => Reg::ECX,
        0x0100 => Reg::EDX,
        0x0101 => Reg::ESP,
        0x0110 => Reg::EBP,
        0x0111 => Reg::ISP,
             _ => panic!("Unknown register code: {}", num),
    }
}

fn get_reg2(num:u64) -> Reg { //reg2 is always coded into bits [8..11]
    match get_nth_byte(num, 3) {
        0x0001 => Reg::EAX,
        0x0010 => Reg::EBX,
        0x0011 => Reg::ECX,
        0x0100 => Reg::EDX,
        0x0101 => Reg::ESP,
        0x0110 => Reg::EBP,
        0x0111 => Reg::ISP,
             _ => panic!("Unknown register code: {}", num),
    }
}

fn get_addr(num:u64) -> MemAddr {
    MemAddr::Addr(num && 0x00_0f_ff_ff_ff_ff_ff_ffu64)
}













