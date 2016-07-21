use std::sync::mpsc::{Sender, Receiver};
use utils::*;

//const CACHE_SIZE:usize = 0;
const CORE_NUM:usize = 1;  //number of cores per cpu
const PIPE_SIZE:usize = 8; //size of instruction pipeline


struct Core {
    //OPERATION PIPELINE
    pipe:[(MemAddr, i64); PIPE_SIZE], 

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
    tx:Sender<CPUBusOp>,
    rx:Receiver<CPUBusOp>,
}

impl Core {
    pub fn new(_tx:Sender<CPUBusOp>, _rx:Receiver<CPUBusOp>) -> Core {
        Core{   pipe:[(MemAddr::Nullptr, 0); PIPE_SIZE],
                EAX:0,
                EBX:0,
                ECX:0,
                EDX:0,
                ESP:0,
                EBP:0,
                ISP:0,
                OVERFLOW:false,
                ZERO:false,
                CARRY:false,
                tx:_tx,
                rx:_rx,
        }
    }

    fn load_from_pipe(&self, addr:MemAddr) -> Result<i64, ()> {
        if let MemAddr::Addr(_) = addr {
            if let MemAddr::Addr(offset) = self.pipe[PIPE_SIZE-1].0 - addr {
                if 0 <= offset && offset < PIPE_SIZE as i64 {
                    return Ok(self.pipe[offset as usize].1)
                } else {
                    return Err(())
                }
            }
        }
        Err(())
    }

    pub fn exec_instr(&mut self) {
        let cur_addr = self.ISP;
        let cur_instr = self.load_instr_at(MemAddr::Addr(cur_addr)); 
    }

    fn load_instr_at(&mut self, addr:MemAddr) -> Instruction {
        if let Ok(inst) = self.load_from_pipe(addr) {
            op_to_instr(inst)
        }
        else {
            let mut mem_block = self.load_from_memory(addr, PIPE_SIZE);
            let mut i:usize = 0;
            while let Some(item) = mem_block.pop(){
                self.pipe[i] = item;
                i += 1;
            }
            op_to_instr(self.pipe[0].1)
        }
    }

    fn load_from_memory(&self, start_addr:MemAddr, num:usize) -> Vec<(MemAddr, i64)> {
        self.tx. send(CPUBusOp::RequestBlock(start_addr,num));
        match self.rx.recv().expect("CPUBus has disconnected unexpectedly") {
            CPUBusOp::GiveBlock(res_vec) => res_vec,
            CPUBusOp::RequestBlock(_,_) => panic!("Unexpectedly received RequestBlock in load_from_memory()"),
            CPUBusOp::Error(err) => panic!("CPUBus returned error in load_from_memory(): {}", err),
        }
    }
}

pub struct CPU {
    //cache:[i64, CACHE_SIZE],
    cores:Vec<CPU>,
    //BUS
    tx:Option<Sender<MemBusOp>>,
    rx:Option<Receiver<MemBusOp>>,
}  

impl CPU {
    pub fn new() -> CPU {
        CPU{cores:Vec::new(),
            tx:None,
            rx:None,}
    }

    pub fn exec(&mut self) {
        loop {
            for (id,core) in self.cores.iter().enumerate() {
            
            }
        }
    }
}


