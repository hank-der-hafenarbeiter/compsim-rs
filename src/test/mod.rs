extern crate rand;
extern crate num;
extern crate enum_primitive;
    
use self::rand::Rng;
use utils::*;
use enum_primitive::FromPrimitive;

mod parser_test;
mod cpu_test;
mod utils_test;


pub fn rand_reg() -> (&'static str, Reg, u64) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0,7) as usize {
        0 => ("EAX",Reg::EAX, 0x00_00_00_00_00_00_00_00u64),
        1 => ("EBX",Reg::EBX, 0x01_00_00_00_00_00_00_00u64),
        2 => ("ECX",Reg::ECX, 0x02_00_00_00_00_00_00_00u64),
        3 => ("EDX",Reg::EDX, 0x03_00_00_00_00_00_00_00u64),
        4 => ("ESP",Reg::ESP, 0x04_00_00_00_00_00_00_00u64),
        5 => ("EBP",Reg::EBP, 0x05_00_00_00_00_00_00_00u64),
        6 => ("ISP",Reg::ISP, 0x06_00_00_00_00_00_00_00u64),
        _ => panic!("Rand's fucked!"),
    }
}

pub fn rand_addr() -> u64 {

    let mut rng = rand::thread_rng();
    rng.gen_range(0,0x00_0f_ff_ff_ff_ff_ff_ff)
}

pub fn rand_instr() -> (String, Instruction) {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    let mut i = Instruction(0);
    let mut opt = 0;

    let (s_reg1, enum_reg1, opt_reg1) = rand_reg();
    let (s_reg2, enum_reg2, opt_reg) = rand_reg();
    let opt_reg2 = opt_reg >> 4; //second reg is always to the left of the first reg
    let addr = rand_addr();
    match rng.gen_range(0,10) as usize {
        0 => {
            s = s + "ADD"     + " " + s_reg1    + " " + s_reg2;
            opt = 0x10_00_00_00_00_00_00_00u64 | opt_reg1 | opt_reg2; 
            i = Instruction(opt);
        },
        1 => {
            s = s + "MUL"     + " " + s_reg1    + " " + s_reg2;
            opt = 0x20_00_00_00_00_00_00_00u64 | opt_reg1 | opt_reg2;
            i = Instruction(opt);
        },
        2 => {
            s = s + "LD"      + " " + s_reg1    + " " + &(addr.to_string());
            opt = 0x30_00_00_00_00_00_00_00u64 | opt_reg1 | addr;
            i = Instruction(opt);
        },
        3 => {
            s = s + "SAV"     + " " + &(addr.to_string())    + " " + s_reg1;
            opt = 0x40_00_00_00_00_00_00_00u64 | opt_reg1 | addr;
            i = Instruction(opt);
        },
        4 => {
            s = s + "PUSH"    + " " + s_reg1;
            opt = 0x50_00_00_00_00_00_00_00u64 | opt_reg1;
            i = Instruction(opt);
        },
        5 => {
            s = s + "POP"     + " " + s_reg1;
            opt = 0x60_00_00_00_00_00_00_00u64 | opt_reg1;
            i = Instruction(opt);
        },
        6 => {
            s = s + "JZ"      + " " + &(addr.to_string());
            opt = 0x71_00_00_00_00_00_00_00u64 | addr;
            i = Instruction(opt);
        },
        7 => {
            s = s + "JGZ"     + " " + &(addr.to_string());
            opt = 0x72_00_00_00_00_00_00_00u64 | addr;
            i = Instruction(opt);
        },
        8 => {
            s = s + "JLZ"     + " " + &(addr.to_string());
            opt = 0x73_00_00_00_00_00_00_00u64 | addr;
            i = Instruction(opt);
        },
        9 => {
            s = s + "NOP";
            opt = 0x00_00_00_00_00_00_00_00u64;
            i = Instruction(opt);
        },
        _ => panic!("Rand's fucked!"),
    };
    (s,i)
}

