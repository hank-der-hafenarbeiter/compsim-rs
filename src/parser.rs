use std::io::{stdin, BufRead};
use std::i64;
use utils::{Instruction, MemAddr, Reg};

struct Parser {
    buffer:Vec<i64>,
}

impl Parser {
    
    pub fn new() -> Parser {
        Parser{buffer:Vec::new()}
    }

    pub fn read_from_input() {
        let mut buffer = String::new();
        let stdin =  stdin();
        let mut handle = stdin.lock();

        let mut program:Vec<Instruction> = Vec::new();

        while let Ok(s) = handle.read_line(&mut buffer) {
            let mut iter = buffer.split_whitespace();

            let mnemonic:String = iter.next().expect("Read empty Instruction").to_string();
            if let Some(op1) = iter.next() {
                if let Some(op2) = iter.next() {
                    program.push(match mnemonic.as_ref() {
                        "ADD"   => Instruction::Add(string_to_reg(&op1.to_string()), string_to_reg(&op2.to_string())),
                        "MUL"   => Instruction::Mul(string_to_reg(&op1.to_string()), string_to_reg(&op2.to_string())),
                        "LDD"   => Instruction::Ld(string_to_reg(&op1.to_string()), string_to_address(&op2.to_string())),
                        "SAV"   => Instruction::Sav(string_to_address(&op1.to_string()), string_to_reg(&op2.to_string())),
                        &_      => panic!("Unkown two operant instruction"),
                    })
                } else {
                    program.push(match mnemonic.as_ref() {
                        "PUSH"  => Instruction::Push(string_to_reg(&op1.to_string())),
                        "POP"   => Instruction::Pop(string_to_reg(&op1.to_string())),
                        "JZ"    => Instruction::Jz(string_to_address(&op1.to_string())),
                        "JGZ"   => Instruction::Jgz(string_to_address(&op1.to_string())),
                        "JLZ"   => Instruction::Jlz(string_to_address(&op1.to_string())),
                        &_      => panic!("Unkown one operant instruction"),
                    })
                }
            } else {
                program.push(match mnemonic.as_ref() {
                    "NOP"   => Instruction::Nop,
                    "EOF"   => break,
                    &_       => panic!("Unknown no operant instruction"),
                });
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum ParseError {
    EOF,
    UnkownInstruction,
}

fn string_to_instruction(_s:&String) -> Result<Instruction, ParseError> {
    let mut iter = _s.split_whitespace();

        let mnemonic:String = iter.next().expect("Read empty Instruction").to_string();
        if let Some(op1) = iter.next() {
            if let Some(op2) = iter.next() {
                println!("string to instruction: {:?}", op2);
                match mnemonic.as_ref() {
                    "ADD"   => Ok(Instruction::Add(string_to_reg(&op1.to_string()), string_to_reg(&op2.to_string()))),
                    "MUL"   => Ok(Instruction::Mul(string_to_reg(&op1.to_string()), string_to_reg(&op2.to_string()))),
                    "LDD"   => Ok(Instruction::Ld(string_to_reg(&op1.to_string()), string_to_address(&op2.to_string()))),
                    "SAV"   => Ok(Instruction::Sav(string_to_address(&op1.to_string()), string_to_reg(&op2.to_string()))),
                    &_      => Err(ParseError::UnkownInstruction),
                }
            } else {
                match mnemonic.as_ref() {
                    "PUSH"  => Ok(Instruction::Push(string_to_reg(&op1.to_string()))),
                    "POP"   => Ok(Instruction::Pop(string_to_reg(&op1.to_string()))),
                    "JZ"    => Ok(Instruction::Jz(string_to_address(&op1.to_string()))),
                    "JGZ"   => Ok(Instruction::Jgz(string_to_address(&op1.to_string()))),
                    "JLZ"   => Ok(Instruction::Jlz(string_to_address(&op1.to_string()))),
                    &_      => Err(ParseError::UnkownInstruction),
                }
            }
        } else {
            match mnemonic.as_ref() {
                "NOP"   => Ok(Instruction::Nop),
                "EOF"   => Err(ParseError::EOF),
                &_       => Err(ParseError::UnkownInstruction),
            }
        }
}


fn string_to_address(_s:&String) -> MemAddr {
    MemAddr::Addr(i64::from_str_radix(&_s, 16).expect("Invalid Memory Address:"))
}

fn string_to_reg(_s:&String) -> Reg {
    let inst = match _s.as_ref() {
        "EAX" => Reg::EAX,
        "EBX" => Reg::EBX,
        "ECX" => Reg::ECX,
        "EDX" => Reg::EDX,
        "ESP" => Reg::ESP,
        "EBP" => Reg::EBP,
        "ISP" => Reg::ISP,
        &_    => panic!("Invalid register in Parser::string_to_reg()"),
    };
    println!("string to reg({:?}) -> {:?}", _s, inst);
    inst
}



#[test]
fn converting_strings_to_instructions() {
    use utils::*;
    use parser::*;
    use std::fmt::Debug;
    let s = "ADD EBX EBX".to_string();
    let inst = string_to_instruction(&s);
    println!("{:?}", inst);
    assert!(Ok(Instruction::Add(Reg::EAX, Reg::EBX)) == string_to_instruction(&s));
}

