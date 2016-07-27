#[cfg(test)]
extern crate rand;
use std::io::{stdin, BufRead};
use std::io::Read;
use std::fs::File;
use std::vec::IntoIter;
use std::i64;
use std::error::*;
use std::num::*;
use utils::{Instruction, MemAddr, Reg};

#[derive(Debug, Clone)]
pub struct Parser {
    buffer:Vec<i64>,
    program:Vec<Instruction>,
}

impl Parser {
    
    pub fn new() -> Parser {
        Parser{
            buffer:Vec::new(),
            program:Vec::new(),
        }
    }

    pub fn read_from_input(&mut self) {
        let mut input_buffer = String::new();
        let stdin =  stdin();
        let mut handle = stdin.lock();


        while let Ok(s) = handle.read_line(&mut input_buffer) {
            self.program.push(string_to_instruction(&input_buffer).expect("ParseError in read_from_input()"));
        }
    }

    pub fn into_instructions(self) -> IntoIter<Instruction> {
        self.program.into_iter()
    }

    pub fn read_from_file(&mut self, _f:&mut File) {
        let mut file_as_string = String::new();
        _f.read_to_string(&mut file_as_string);
        println!("{:?}", file_as_string);
        for line in file_as_string.lines() {
            self.program
                .push(string_to_instruction(line).expect("converting_strings_to_instructions() failed in read_from_file():"));
        }
    }
}


#[derive(PartialEq, Debug, Clone)]
enum ParseError {
    EOF,
    UnkownInstruction(String),
    UnkownReg(String),
    InvalidMemAddress(String),
}

fn string_to_instruction(_s:&str) -> Result<Instruction, ParseError> {
    let mut iter = _s.split_whitespace();

        let mnemonic:String = iter.next().expect("Read empty Instruction").to_string();
        if let Some(op1) = iter.next() {
            if let Some(op2) = iter.next() {
                match mnemonic.as_ref() {
                    "ADD"   => Ok(Instruction::Add(string_to_reg(op1).unwrap(), string_to_reg(op2).unwrap())),
                    "MUL"   => Ok(Instruction::Mul(string_to_reg(op1).unwrap(), string_to_reg(op2).unwrap())),
                    "LD"   => Ok(Instruction::Ld(string_to_reg(op1).unwrap(), string_to_address(op2).unwrap())),
                    "SAV"   => Ok(Instruction::Sav(string_to_address(op1).unwrap(), string_to_reg(op2).unwrap())),
                    s      => Err(ParseError::UnkownInstruction(s.to_string())),
                }
            } else {
                match mnemonic.as_ref() {
                    "PUSH"  => Ok(Instruction::Push(string_to_reg(op1).unwrap())),
                    "POP"   => Ok(Instruction::Pop(string_to_reg(op1).unwrap())),
                    "JZ"    => Ok(Instruction::Jz(string_to_address(op1).unwrap())),
                    "JGZ"   => Ok(Instruction::Jgz(string_to_address(op1).unwrap())),
                    "JLZ"   => Ok(Instruction::Jlz(string_to_address(op1).unwrap())),
                    s      => Err(ParseError::UnkownInstruction(s.to_string())),
                }
            }
        } else {
            match mnemonic.as_ref() {
                "NOP"   => Ok(Instruction::Nop),
                "EOF"   => Err(ParseError::EOF),
                s       => Err(ParseError::UnkownInstruction(s.to_string())),
            }
        }
}


fn string_to_address(_s:&str) -> Result<MemAddr, ParseError> {
    i64::from_str_radix(&_s, 10).map_err(|s| ParseError::InvalidMemAddress(s.to_string())).and_then(|x| { 
        if x >= 0 && x <= 0x00_0f_ff_ff_ff_ff_ff_ff {
            Ok(MemAddr::Addr(x))
        } else if x < 0 {
            Err(ParseError::InvalidMemAddress("Negative Address".to_string()))
        } else {
            Err(ParseError::InvalidMemAddress("Address too big: ".to_string() + &x.to_string()))
        }
    })
}

fn string_to_reg(_s:&str) -> Result<Reg, ParseError> {
    match _s.as_ref() {
        "EAX" => Ok(Reg::EAX),
        "EBX" => Ok(Reg::EBX),
        "ECX" => Ok(Reg::ECX),
        "EDX" => Ok(Reg::EDX),
        "ESP" => Ok(Reg::ESP),
        "EBP" => Ok(Reg::EBP),
        "ISP" => Ok(Reg::ISP),
        s     => Err(ParseError::UnkownReg(s.to_string())),
    }
}

#[test]
fn converting_strings_to_regs() {
    use utils::*;
    use parser::*;

    let vec = vec!["EAX", "EBX", "ECX", "EDX", "ESP", "EBP", "ISP"];
    let mut iter = vec.iter();
    assert!(Ok(Reg::EAX) == string_to_reg(iter.next().unwrap()));
    assert!(Ok(Reg::EBX) == string_to_reg(iter.next().unwrap()));
    assert!(Ok(Reg::ECX) == string_to_reg(iter.next().unwrap()));
    assert!(Ok(Reg::EDX) == string_to_reg(iter.next().unwrap()));
    assert!(Ok(Reg::ESP) == string_to_reg(iter.next().unwrap()));
    assert!(Ok(Reg::EBP) == string_to_reg(iter.next().unwrap()));
    assert!(Ok(Reg::ISP) == string_to_reg(iter.next().unwrap()));
}


#[test]
fn converting_strings_to_instructions() {
    use utils::*;
    use parser::*;
    
    let v_regs = vec!["EAX", "EBX", "ECX", "EDX", "ESP", "EBP", "ISP"];
    let s = "ADD EBX EBX".to_string();
    let inst = string_to_instruction(&s);
    assert!(Ok(Instruction::Add(Reg::EBX, Reg::EBX)) == string_to_instruction(&s));
}

#[test]
fn converting_strings_to_addresses() {
    for num in 0..0x00_00_00_00_00_00_ff_ffi64 {
        if let Err(mem) = string_to_address(&num.to_string()) {
            println!("{:x} => {:?}", num, string_to_address(&num.to_string()));
            panic!("valid address not accepted");
        }
    }

    if let Err(mem) = string_to_address(&0x00_00_00_00_0f_ff_ff_ff.to_string()) {
        panic!("Heighest addres not accepted!");
    }
    
    for num in -10..-1 as i64 {
        if let Ok(mem) = string_to_address(&num.to_string()) {
            panic!("too low address accepted");
        }
    }
      
    for num in (0x00_00_00_00_f0_00_00_00i64..0x00_00_00_00_1f_ff_ff_ffi64).step_by(100000) {
        if let Ok(mem) = string_to_address(&num.to_string()) {
            panic!("too high address accepted");
        }
    }
}

#[test]
fn converting_file_to_programm() {
    use std::io::Read;
    use std::io::Write;
    use std::fs::File;
    use parser::Parser;
    use self::rand::Rng;


    let mut rand_reg = || -> (&str, Reg) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0,7) as usize {
            0 => ("EAX",Reg::EAX),
            1 => ("EBX",Reg::EBX),
            2 => ("ECX",Reg::ECX),
            3 => ("EDX",Reg::EDX),
            4 => ("EBP",Reg::EBP),
            5 => ("ESP",Reg::ESP),
            6 => ("ISP",Reg::ISP),
            _ => panic!("Rand's fucked!"),
        }
    };

    let mut rand_addr = || -> (i64, MemAddr) {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0,0x00_0f_ff_ff_ff_ff_ff_ff);
        (num, MemAddr::Addr(num))
    };

    let mut rand_instr = || -> (String, Instruction) {
        let mut rng = rand::thread_rng();
        let mut s = String::new();
        let mut i = Instruction::Nop;

        let (s_reg1, enum_reg1) = rand_reg();
        let (s_reg2, enum_reg2) = rand_reg();
        let (t1,t2) = rand_addr();
        let (s_addr, enum_addr) = (&t1.to_string(),t2);

        match rng.gen_range(0,10) as usize {
            0 => {
                s = s + "ADD"     + " " + s_reg1    + " " + s_reg2;
                i = Instruction::Add(enum_reg1, enum_reg2);
            },
            1 => {
                s = s + "MUL"     + " " + s_reg1    + " " + s_reg2;
                i = Instruction::Mul(enum_reg1, enum_reg2);
            },
            2 => {
                s = s + "LD"      + " " + s_reg1    + " " + s_addr;
                i = Instruction::Ld(enum_reg1, enum_addr);
            },
            3 => {
                s = s + "SAV"     + " " + s_addr    + " " + s_reg1;
                i = Instruction::Sav(enum_addr, enum_reg1);
            },
            4 => {
                s = s + "PUSH"    + " " + s_reg1;
                i = Instruction::Push(enum_reg1);
            },
            5 => {
                s = s + "POP"     + " " + s_reg1;
                i = Instruction::Pop(enum_reg1);
            },
            6 => {
                s = s + "JZ"      + " " + s_addr;
                i = Instruction::Jz(enum_addr);
            },
            7 => {
                s = s + "JGZ"     + " " + s_addr;
                i = Instruction::Jgz(enum_addr);
            },
            8 => {
                s = s + "JLZ"     + " " + s_addr;
                i = Instruction::Jlz(enum_addr);
            },
            9 => {
                s = s + "NOP";
                i = Instruction::Nop;
            },
            _ => panic!("Rand's fucked!"),
        };
        (s,i)
                            
    };

    let mut inst_vec:Vec<Instruction> = Vec::new();
    let mut inst_string = String::new();
    let mut f_handle = File::create("test.asm").unwrap();

    for _ in 0..100 {
        let (s,i) = rand_instr();
        inst_string = inst_string + &s;
        inst_string.push('\n');

        inst_vec.push(i);
    }

    f_handle.write_all(inst_string.as_bytes());
    f_handle.flush(); 

    let mut f_handle = File::open("test.asm").unwrap();
    let mut p = Parser::new();
    p.read_from_file(&mut f_handle);
    println!("{:#?}",p);
    let mut iter = p.into_instructions().zip(inst_vec.into_iter());
   
    for (i1,i2) in iter {
        println!("{:?} | {:?}",i1, i2);
        assert_eq!(i1,i2);
    }
}
