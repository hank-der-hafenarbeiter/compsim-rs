use std::sync::mpsc::{Sender, Receiver};

//static CACHE_SIZE:usize = 0;
static CORE_NUM:usize = 1;  //number of cores per cpu
static PIPE_SIZE:usize = 8; //size of instruction pipeline
struct Pipeline {
    pipe:[(MemAddr, Instruction), PIPE_SIZE]; 
    start_addr:MemAddr,
    end_addr:MemAddr,
}

struct Core {
    //OPERATION PIPELINE
    pipe:[(MemAddr, Instruction), PIPE_SIZE]

    //REGISTERS
    EAX:i64,
    EBX:i64,
    ECX:i64,
    EDX:i64,
    ESP:i64,
    EBP:i64,
    ISP:i64,
    
    //FLAGS
    OVERFLOW:bool,
    ZERO:bool,
    CARRY:bool,

    //CPU BUS
    tx:Option<Sender>,
    rx:Option<Receiver>,
}

impl Core {
    pub fn new() -> Core {
        Core{   pipe:[(MemAddr::Nullptr,Instruction::Nop), PIPE_SIZE],
                EAX:0,
                EBX:0,
                ECX:0,
                EDX:0,
                ESP:0,
                EBP:0,
                ISP:0,
                OVERFLOW:0,
                ZERO:0,
                CARRY:0,
                tx:None,
                rx:None,
        }
    }
    
    fn decode_op(inst_code:i64) -> Instruction {
        unimplemented!();
    }

    pub fn exec_instr(&mut self) {
        let cur_instr = load_instr_at(MemAddr(self.ISP)); 
    }

    fn load_instr_at(&mut self, addr:MemAddr) -> Instruction {
        if let Result(op) = self.pipe.get(addr) {
            op
        }
        else {
            0
    }

    fn load_from_memory(&self, start_addr:MemAddr, num:usize) -> Vec(MemAddr, i64) {
        tx.send(CPUBusOP::RequestBlock(start_addr,num));
        match ans = rx.recv().expect("CPUBus has disconnected unexpectedly") {
            CPUBusOP::GiveBlock(res_vec) => res_vec,
            CPUBusOP::Error(err) => panic!("CPUBus returned error in load_from_memory(): {}", err),
        }
    }
}

struct CPU {
    //cache:[i64, CACHE_SIZE],
    cores:Vec<CPU>,
    //BUS
    tx:Option<Sender>,
    rx:Option<Receiver>,
}  

impl CPU {
    pub fn new() -> CPU {
        CPU{cores:Vector::new<Core>()
            tx:None,
            rx:None,}
    }

    pub fn exec() {
        loop {
            for (id,core) in cores.iter().enumerate() {
            
}


