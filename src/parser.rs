#[cfg(test)]
extern crate rand;
use std::io::{stdin, BufRead};
use std::io::Read;
use std::fs::File;
use std::vec::IntoIter;
use std::i64;
use std::error::*;
use std::num::*;
use utils::{Instruction, Reg};

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
            self.program.push(input_buffer.parse().expect("Parsing line failed: "));
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
                .push(line.parse().expect("Parsing line failed: "));
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

