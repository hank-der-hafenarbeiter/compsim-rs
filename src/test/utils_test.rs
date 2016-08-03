#[test]
fn test_op_to_instr() {
    use utils::*;
    use super::rand_instr;

    for _ in 0..100 {
        let (_,instr,op) = rand_instr();
        //let op = 0x11_10_00_00_00_00_00_00i64;
        //let instr = Instruction::Add(Reg::EAX, Reg::EAX);

        let res = op_to_instr(op);
        println!("{:?}, {:#x}, {:#?}", instr, op, res);
        assert_eq!(instr, res.unwrap());
    }
    
}

#[test]
fn nth_byte() {
    use utils::get_nth_byte;

    let number = 0x01_02_03_04_05_06_07_08i64;
    println!("numbers: {:#x}", number);
    for i in 0..8 {
        println!("i = {}, get_nth_byte() = {}", i, get_nth_byte(number,i as usize));
        assert_eq!(get_nth_byte(number, i as usize), (i+1) as i8);
    }
    
}
