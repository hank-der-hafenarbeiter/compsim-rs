extern crate rand;
    
use self::rand::Rng;
use utils::*;

mod parser_test;
mod cpu_test;
mod utils_test;


pub fn rand_reg() -> (&'static str, Reg, i64) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0,7) as usize {
        0 => ("EAX",Reg::EAX, 0x01_00_00_00_00_00_00_00i64),
        1 => ("EBX",Reg::EBX, 0x02_00_00_00_00_00_00_00i64),
        2 => ("ECX",Reg::ECX, 0x03_00_00_00_00_00_00_00i64),
        3 => ("EDX",Reg::EDX, 0x04_00_00_00_00_00_00_00i64),
        4 => ("ESP",Reg::ESP, 0x05_00_00_00_00_00_00_00i64),
        5 => ("EBP",Reg::EBP, 0x06_00_00_00_00_00_00_00i64),
        6 => ("ISP",Reg::ISP, 0x07_00_00_00_00_00_00_00i64),
        _ => panic!("Rand's fucked!"),
    }
}

pub fn rand_addr() -> (i64, MemAddr) {

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0,0x00_0f_ff_ff_ff_ff_ff_ff);
    (num, MemAddr::Addr(num))
}

pub fn rand_instr() -> (String, Instruction, i64) {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    let mut i = Instruction::Nop;
    let mut opt = 0;

    let (s_reg1, enum_reg1, opt_reg1) = rand_reg();
    let (s_reg2, enum_reg2, opt_reg) = rand_reg();
    let opt_reg2 = opt_reg >> 4; //second reg is always to the left of the first reg
    let (opt_addr,t2) = rand_addr();
    let (s_addr, enum_addr) = (&opt_addr.to_string(),t2);

    match rng.gen_range(0,10) as usize {
        0 => {
            s = s + "ADD"     + " " + s_reg1    + " " + s_reg2;
            i = Instruction::Add(enum_reg1, enum_reg2);
            opt = 0x10_00_00_00_00_00_00_00i64 | opt_reg1 | opt_reg2; 
        },
        1 => {
            s = s + "MUL"     + " " + s_reg1    + " " + s_reg2;
            i = Instruction::Mul(enum_reg1, enum_reg2);
            opt = 0x20_00_00_00_00_00_00_00i64 | opt_reg1 | opt_reg2;
        },
        2 => {
            s = s + "LD"      + " " + s_reg1    + " " + s_addr;
            i = Instruction::Ld(enum_reg1, enum_addr);
            opt = 0x30_00_00_00_00_00_00_00i64 | opt_reg1 | opt_addr;
        },
        3 => {
            s = s + "SAV"     + " " + s_addr    + " " + s_reg1;
            i = Instruction::Sav(enum_addr, enum_reg1);
            opt = 0x40_00_00_00_00_00_00_00i64 | opt_reg1 | opt_addr;
        },
        4 => {
            s = s + "PUSH"    + " " + s_reg1;
            i = Instruction::Push(enum_reg1);
            opt = 0x50_00_00_00_00_00_00_00i64 | opt_reg1;
        },
        5 => {
            s = s + "POP"     + " " + s_reg1;
            i = Instruction::Pop(enum_reg1);
            opt = 0x60_00_00_00_00_00_00_00i64 | opt_reg1;
        },
        6 => {
            s = s + "JZ"      + " " + s_addr;
            i = Instruction::Jz(enum_addr);
            opt = 0x71_00_00_00_00_00_00_00i64 | opt_addr;
        },
        7 => {
            s = s + "JGZ"     + " " + s_addr;
            i = Instruction::Jgz(enum_addr);
            opt = 0x72_00_00_00_00_00_00_00i64 | opt_addr;
        },
        8 => {
            s = s + "JLZ"     + " " + s_addr;
            i = Instruction::Jlz(enum_addr);
            opt = 0x73_00_00_00_00_00_00_00i64 | opt_addr;
        },
        9 => {
            s = s + "NOP";
            i = Instruction::Nop;
            opt = 0x00_00_00_00_00_00_00_00i64;
        },
        _ => panic!("Rand's fucked!"),
    };
    (s,i,opt)
}

