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
pub enum ParseError {
    EOF,
    UnkownInstruction(String),
    UnkownReg(String),
    InvalidMemAddress(String),
}

pub fn string_to_instruction(_s:&str) -> Result<Instruction, ParseError> {
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


pub fn string_to_address(_s:&str) -> Result<MemAddr, ParseError> {
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

pub fn string_to_reg(_s:&str) -> Result<Reg, ParseError> {
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


