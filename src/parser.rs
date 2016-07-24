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
        let mut handle = stdin().lock();

        let mut program:Vec<Instruction> = Vec::new();

        while let Ok(s) = handle.read_line(&mut buffer) {
            let iter = buffer.trim().chars();

            let mnemonic:String = iter.take_while(|c| *c != ' ').collect();
            let op1:String = iter.skip(1).take_while(|c| *c != ' ').collect(); 
            let op2:String = iter.collect();

            if mnemonic == "EOF" {
                break;
            }

            program.push(match mnemonic.as_ref() {
                "ADD"   => Instruction::Add(string_to_reg(op1), string_to_reg(op2)),
                "MUL"   => Instruction::Mul(string_to_reg(op1), string_to_reg(op2)),
                "LDD"   => Instruction::Ld(string_to_reg(op1), string_to_address(op2)),
                "SAV"   => Instruction::Sav(string_to_address(op1), string_to_reg(op2)),
                "PUSH"  => Instruction::Push(string_to_reg(op1)),
                "POP"   => Instruction::Pop(string_to_reg(op1)),
                "JZ"    => Instruction::Jz(string_to_address(op1)),
                "JGZ"   => Instruction::Jgz(string_to_address(op1)),
                "JLZ"   => Instruction::Jlz(string_to_address(op1)),
                "NOP"   => Instruction::Nop,
                &_       => panic!("Unexpected mnemonic in Parser::read_from_input()"),
            });
        }
    }
}

fn string_to_address(_s:String) -> MemAddr {
    MemAddr::Addr(i64::from_str_radix(&_s, 16).expect("Invalid Memory Address:"))
}

fn string_to_reg(_s:String) -> Reg {
    match _s.as_ref() {
        "EAX" => Reg::EAX,
        "EBX" => Reg::EBX,
        "ECX" => Reg::ECX,
        "EDX" => Reg::EDX,
        "ESP" => Reg::ESP,
        "EBP" => Reg::EBP,
        "ISP" => Reg::ISP,
        &_    => panic!("Invalid register in Parser::string_to_reg()"),
    }
}
