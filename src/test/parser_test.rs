extern crate rand;

#[test]
fn converting_strings_to_regs() {
    use utils::*;
    use parser::*;

    let vec = vec!["EAX", "EBX", "ECX", "EDX", "ESP", "EBP", "ISP"];
    let mut iter = vec.iter();
    assert!(Ok(Reg::EAX) == iter.next().unwrap().parse());
    assert!(Ok(Reg::EBX) == iter.next().unwrap().parse());
    assert!(Ok(Reg::ECX) == iter.next().unwrap().parse());
    assert!(Ok(Reg::EDX) == iter.next().unwrap().parse());
    assert!(Ok(Reg::ESP) == iter.next().unwrap().parse());
    assert!(Ok(Reg::EBP) == iter.next().unwrap().parse());
    assert!(Ok(Reg::ISP) == iter.next().unwrap().parse());
}


#[test]
fn converting_strings_to_instructions() {
    use utils::*;
    use parser::*;
    
    let v_regs = vec!["EAX", "EBX", "ECX", "EDX", "ESP", "EBP", "ISP"];
    let s = "ADD EBX EBX".to_string();
    assert!(Ok(InstructionBuilder::new().set_opcode(Opcode::Add).set_reg1(Reg::EBX).set_reg2(Reg::EBX).finalize()) == s.parse());
}

#[test]
fn converting_file_to_programm() {
    use std::io::Read;
    use std::io::Write;
    use std::fs::File;
    use std::fs::remove_file;
    use parser::Parser;
    use utils::*;
    use self::rand::Rng;
    use super::rand_instr;

    let mut inst_vec:Vec<Instruction> = Vec::new();
    let mut inst_string = String::new();
    let mut f_handle = File::create("test.asm").unwrap();

    for _ in 0..100 {
        let (s,i) = rand_instr();
        println!("{}\n{:#X}\n{:#X}\n", s, s.parse::<Instruction>().unwrap().0, i.0);
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
    remove_file("test.asm");
}
