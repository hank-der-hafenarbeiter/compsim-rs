use std::ops::Sub;
use std::str::FromStr;
use snowflake::ProcessUniqueId;
use num::FromPrimitive;
use std::fmt;
    
use parser::ParseError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Instruction (pub u64);

impl Instruction {
    pub fn new() -> Instruction {
        Instruction(0)
    }

    pub fn opcode(&self) -> Opcode {
        Opcode::from_u64(self.0 >> 60).expect("Unknown Instruction")
    }

    pub fn reg1(&self) -> Reg {
        Reg::from_u8(get_nth_byte(self.0, 0) & 0x0f).expect("Unkown Register 1")
    }

    pub fn reg2(&self) -> Reg {
        Reg::from_u8(get_nth_byte(self.0, 1) >> 4).expect("Unkown Register 2")
    }

    pub fn addr(&self) -> u64 {
        self.0 & 0x00_0f_ff_ff_ff_ff_ff_ffu64
    } 
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}",  self.opcode())
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let mut iter = _s.split_whitespace();

        let operation = iter.next().expect("Read empty line");
        if let Some(operant1) = iter.next() {
            if let Some(operant2) = iter.next() {
                match operation {
                    "ADD"   => {
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Add)
                            .set_reg1(operant1.parse().unwrap())
                            .set_reg2(operant2.parse().unwrap())
                            .finalize())
                    },

                    "MUL"   => {
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Mul)
                            .set_reg1(operant1.parse().unwrap())
                            .set_reg2(operant2.parse().unwrap())
                            .finalize())
 
                    },

                    "LD"    => {
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Ld)
                            .set_reg1(operant1.parse().unwrap())
                            .set_addr(operant2.parse().unwrap())
                            .finalize())

                    },

                    "SAV"   => {
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Sav)
                            .set_addr(operant1.parse().unwrap())
                            .set_reg1(operant2.parse().unwrap())
                            .finalize())

                    },

                    s      => Err(ParseError::UnkownInstruction(s.to_string())),
                }
            } else {
                match operation {
                    "PUSH"  => {
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Push)
                            .set_reg1(operant1.parse().unwrap())
                            .finalize())

                    },
                    "POP"   => {
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Pop)
                            .set_reg1(operant1.parse().unwrap())
                            .finalize())
                    },
                    "JZ"    => {
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Jz)
                            .set_addr(operant1.parse().unwrap())
                            .finalize())
                    },
                    "JGZ"   => { 
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Jgz)
                            .set_addr(operant1.parse().unwrap())
                            .finalize())
                    },
                    "JLZ"   => { 
                        Ok(InstructionBuilder::new()
                            .set_opcode(Opcode::Jlz)
                            .set_addr(operant1.parse().unwrap())
                            .finalize())
                    },
                    s       => Err(ParseError::UnkownInstruction(s.to_string())),
                }
            }
        } else {
            match operation {
                "NOP" => Ok(Instruction(0)),
                s       => Err(ParseError::UnkownInstruction(s.to_string())),
            }
        }
    }
}

pub struct InstructionBuilder(u64);

impl InstructionBuilder {

    pub fn new() -> InstructionBuilder {
        InstructionBuilder(0)
    }

    pub fn set_opcode(&mut self, _op: Opcode) -> &mut InstructionBuilder {
        self.0 = self.0 & 0x0f_ff_ff_ff_ff_ff_ff_ffu64 | match _op {
            Opcode::Nop => 0x00_00_00_00_00_00_00_00u64,
            Opcode::Add => 0x10_00_00_00_00_00_00_00u64,
            Opcode::Mul => 0x20_00_00_00_00_00_00_00u64,
            Opcode::Ld  => 0x30_00_00_00_00_00_00_00u64,
            Opcode::Sav => 0x40_00_00_00_00_00_00_00u64,
            Opcode::Push=> 0x50_00_00_00_00_00_00_00u64,
            Opcode::Pop => 0x60_00_00_00_00_00_00_00u64,
            Opcode::Jz  => 0x71_00_00_00_00_00_00_00u64,
            Opcode::Jgz => 0x72_00_00_00_00_00_00_00u64,
            Opcode::Jlz => 0x73_00_00_00_00_00_00_00u64,
        };
        self
    }

    pub fn set_reg1(&mut self, _reg: Reg) -> &mut InstructionBuilder {
        self.0 = self.0 & 0xf0_ff_ff_ff_ff_ff_ff_ffu64 | match _reg {
            Reg::EAX => 0x00_00_00_00_00_00_00_00u64,
            Reg::EBX => 0x01_00_00_00_00_00_00_00u64,
            Reg::ECX => 0x02_00_00_00_00_00_00_00u64,
            Reg::EDX => 0x03_00_00_00_00_00_00_00u64,
            Reg::ESP => 0x04_00_00_00_00_00_00_00u64,
            Reg::EBP => 0x05_00_00_00_00_00_00_00u64,
            Reg::ISP => 0x06_00_00_00_00_00_00_00u64,
        };
        self
    }

    pub fn set_reg2(&mut self, _reg: Reg) -> &mut InstructionBuilder {
        self.0 = self.0 & 0xff_0f_ff_ff_ff_ff_ff_ffu64 | match _reg {
            Reg::EAX => 0x00_00_00_00_00_00_00_00u64,
            Reg::EBX => 0x00_10_00_00_00_00_00_00u64,
            Reg::ECX => 0x00_20_00_00_00_00_00_00u64,
            Reg::EDX => 0x00_30_00_00_00_00_00_00u64,
            Reg::ESP => 0x00_40_00_00_00_00_00_00u64,
            Reg::EBP => 0x00_50_00_00_00_00_00_00u64,
            Reg::ISP => 0x00_60_00_00_00_00_00_00u64,
        };
        self
    }
    
    pub fn set_addr(&mut self, _addr: u64) -> &mut InstructionBuilder {
        self.0 = self.0 & 0xff_f0_00_00_00_00_00_00u64 | _addr; 
        self
    }

    pub fn finalize(&self) -> Instruction {
        Instruction(self.0)
    }
}
enum_from_primitive!{
#[derive(Clone,Copy,PartialEq, Debug)]
pub enum Opcode {
    Nop = 0x0,
    Add = 0x1,
    Mul = 0x2,
    Ld  = 0x3,
    Sav = 0x4,
    Push= 0x5,
    Pop = 0x6,
    Jz  = 0x7,
    Jgz = 0x8,
    Jlz = 0x9,
}
}





    
enum_from_primitive!{
#[derive(Copy, Clone,PartialEq, Debug)]
pub enum Reg {
    EAX = 0x00,
    EBX = 0x01,
    ECX = 0x02,
    EDX = 0x03,
    ESP = 0x04,
    EBP = 0x05,
    ISP = 0x06,
}
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Reg {
    type Err = ParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        match _s {
            "EAX" => Ok(Reg::EAX),
            "EBX" => Ok(Reg::EBX),
            "ECX" => Ok(Reg::ECX),
            "EDX" => Ok(Reg::EDX),
            "ESP" => Ok(Reg::ESP),
            "EBP" => Ok(Reg::EBP),
            "ISP" => Ok(Reg::ISP),
            s       => panic!("Unrecognized register token: {}", s),
        }
    }

}

#[derive(Debug)]
pub enum CPUBusOp {
    RequestBlock(u64, usize),
    GiveBlock(Vec<(u64, u64)>),
    Sleep,
    WakeUp,
    ExecAt(u64),
    Error(String),
}

#[derive(Debug)]
pub enum MemBusOp {
    RequestBlock(ProcessUniqueId, u64, usize),
    GiveBlock(ProcessUniqueId, Vec<(u64, u64)>),
    Error(String)
}

pub fn get_nth_byte(num:u64, nth:usize) -> u8 {
    let mask =  0x00_00_00_00_00_00_00_ffu64;
    let shifted = num >> (7-nth)*8;
    (mask & shifted) as u8
}


