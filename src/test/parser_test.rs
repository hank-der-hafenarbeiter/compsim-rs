extern crate rand;

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
    use utils::*;
    use parser::*;

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
    use utils::*;
    use self::rand::Rng;
    use super::rand_instr;

    let mut inst_vec:Vec<Instruction> = Vec::new();
    let mut inst_string = String::new();
    let mut f_handle = File::create("test.asm").unwrap();

    for _ in 0..100 {
        let (s,i,_) = rand_instr();
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
