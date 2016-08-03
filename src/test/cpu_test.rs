
#[test]
#[ignore]
fn basic_reg_instr() {
    use cpu::*;
    use utils::*;
    use snowflake;
    use std::sync::mpsc::channel;

    let (fake_tx, fake_rx) = channel();
    let mut c = Core{   
        ID:snowflake::ProcessUniqueId::new(), 
        pipe:[
            (MemAddr::Addr(0), instruction_to_opcode(Instruction::Add(Reg::EAX,Reg::EAX)).unwrap()),
            (MemAddr::Addr(1), instruction_to_opcode(Instruction::Nop).unwrap()),
            (MemAddr::Addr(2), instruction_to_opcode(Instruction::Nop).unwrap()),
            (MemAddr::Addr(3), instruction_to_opcode(Instruction::Nop).unwrap()),
            (MemAddr::Addr(4), instruction_to_opcode(Instruction::Nop).unwrap()),
            (MemAddr::Addr(5), instruction_to_opcode(Instruction::Nop).unwrap()),
            (MemAddr::Addr(6), instruction_to_opcode(Instruction::Mul(Reg::EAX, Reg::EBP)).unwrap()),
            (MemAddr::Addr(7), instruction_to_opcode(Instruction::Jz(MemAddr::Addr(0))).unwrap()),
        ],
        EAX:1,
        EBX:0,
        ECX:3,
        EDX:4,
        ESP:5,
        EBP:6,
        ISP:0,
        OVERFLOW:false,
        ZERO:false,
        SIGN:false,
        CARRY:false,
        tx:fake_tx,
        rx:fake_rx,
    };

    c.exec_instr();
    panic!();
}
