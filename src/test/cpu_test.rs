
#[test]
fn basic_reg_instr() {
    use cpu::*;
    use utils::*;
    use snowflake;
    use std::sync::mpsc::channel;

    let (fake_tx, fake_rx) = channel();
    let mut c = Core{   
        ID:snowflake::ProcessUniqueId::new(), 
        pipe:[
            (0, InstructionBuilder::new().set_opcode(Opcode::Add).set_reg1(Reg::EAX).set_reg2(Reg::ECX).finalize().0),
            (1, InstructionBuilder::new().finalize().0),
            (2, InstructionBuilder::new().set_opcode(Opcode::Jz).set_addr(0).finalize().0),
            (3, InstructionBuilder::new().finalize().0),
            (4, InstructionBuilder::new().finalize().0),
            (5, InstructionBuilder::new().finalize().0),
            (6, InstructionBuilder::new().finalize().0),
            (7, InstructionBuilder::new().finalize().0),
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
    for _ in 0..5 {
        c.exec_instr();
    }
    panic!();
}
