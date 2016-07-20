use std::sync::mpsc::{Sender, Receiver};
use utils::{MemAddr, Instruction, CPUBusOP, Reg};

//static CACHE_SIZE:usize = 0;
static CORE_NUM:usize = 1;  //number of cores per cpu
static PIPE_SIZE:usize = 8; //size of instruction pipeline

struct Pipeline {
    pipe:[(MemAddr, i64); PIPE_SIZE], 
    start_addr:MemAddr,
    end_addr:MemAddr,
}

impl Pipeline {

    fn get(&self, addr:MemAddr) -> Result<i64, _> {
        if let MemAddr::Addr(_) = addr {
            let offset = self.end_addr - addr;       
            if 0 <= offset && offset < PIPE_SIZE {
                Ok(pipe[offset].2)
            } else {
                Err()
            }
        }
        Err()
    }
}

struct Core {
    //OPERATION PIPELINE
    pipe:Pipeline,

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
        Core{   pipe:Pipeline,
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
    

    pub fn exec_instr(&mut self) {
        let cur_instr = self.load_instr_at(MemAddr::Addr(self.ISP)); 
    }

    fn load_instr_at(&mut self, addr:MemAddr) -> i64 {
        if let Ok(addr, inst) = self.pipe.get(addr) {
            op_to_instr(inst)
        }
        else {
            let mem_block = load_from_memory(addr, PIPE_SIZE);
            for i in (0..7) {
                self.pipe[i] = mem_block.pop();
            }
            op_to_instr(self.pipe[0].2);
        }
    }

    fn load_from_memory(&self, start_addr:MemAddr, num:usize) -> Vec(MemAddr, i64) {
        self.tx.send(CPUBusOP::RequestBlock(start_addr,num));
        match self.rx.recv().expect("CPUBus has disconnected unexpectedly") {
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


