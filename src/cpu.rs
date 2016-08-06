use snowflake::ProcessUniqueId;
use std::sync::mpsc::{Sender, Receiver, channel};
use utils::*;

//const CACHE_SIZE:usize = 0;
const CORE_NUM:usize = 1;  //number of cores per cpu
const PIPE_SIZE:usize = 8; //size of instruction pipeline

#[derive(Debug)]
pub struct Core { //TODO: rewrite tests so that members don't need to be public
    pub ID:ProcessUniqueId,
    //OPERATION PIPELINE
    pub pipe:[(u64, u64); PIPE_SIZE], 

    //REGISTERS
    pub EAX:u64,
    pub EBX:u64,
    pub ECX:u64,
    pub EDX:u64,
    pub ESP:u64,
    pub EBP:u64,
    pub ISP:u64,
    //FLAGS
    pub OVERFLOW:bool,
    pub ZERO:bool,
    pub SIGN:bool,
    pub CARRY:bool,

    //CPU BUS
    pub tx:Sender<CPUBusOp>,
    pub rx:Receiver<CPUBusOp>,
}

impl Core {
    pub fn new(_tx:Sender<CPUBusOp>, _rx:Receiver<CPUBusOp>) -> Core {
        Core{   ID:ProcessUniqueId::new(), 
                pipe:[(0, 0); PIPE_SIZE],
                EAX:0,
                EBX:0,
                ECX:0,
                EDX:0,
                ESP:0,
                EBP:0,
                ISP:0,
                OVERFLOW:false,
                ZERO:false,
                SIGN:false,
                CARRY:false,
                tx:_tx,
                rx:_rx,
        }
    }
    fn read_reg(&self, reg:Reg) -> u64 { 
        match reg {
            Reg::EAX => self.EAX,
            Reg::EBX => self.EBX,
            Reg::ECX => self.ECX,
            Reg::EDX => self.EDX,
            Reg::ESP => self.ESP,
            Reg::EBP => self.EBP,
            Reg::ISP => self.ISP,
        }
    }

    fn write_reg(&mut self, reg:Reg, value:u64) {
        match reg {
            Reg::EAX => self.EAX = value,
            Reg::EBX => self.EBX = value,
            Reg::ECX => self.ECX = value,
            Reg::EDX => self.EDX = value,
            Reg::ESP => self.ESP = value,
            Reg::EBP => self.EBP = value,
            Reg::ISP => self.ISP = value,
        };
    }

    fn read_from_pipe(&self, addr:u64) -> Result<u64, ()> {
        let pipe_start = self.pipe[0].0;
        let pipe_end = pipe_start + PIPE_SIZE as u64;
        if pipe_start <= addr && addr <= pipe_end {
            let offset = addr - pipe_start;
            Ok(self.pipe[offset as usize].1)
        } else {
            Err(())
        }
    }

    fn set_flags(&mut self, res:u64, _of:bool) {

        if _of {
            self.OVERFLOW = true;
            self.CARRY = true;
        } else {
            self.OVERFLOW = false;
            self.CARRY = false;
        }

        if res < 0 {
            self.SIGN = true;
            self.ZERO = false;
        } else if res == 0 {
            self.ZERO = true;
            self.SIGN = false;
        }
    }

    fn reset_flags(&mut self) {
        self.CARRY = false;
        self.OVERFLOW = false;
        self.ZERO = false;
        self.SIGN = false;
    }

    pub fn exec_instr(&mut self) {

        let cur_addr = self.ISP;
        let cur_instr = self.read_instr_at(cur_addr); 
        self.ISP += 1;

        println!("exec_instr: (cur_addr, cur_instr) = ({},{})", cur_addr, cur_instr);   //DEBUG

        match cur_instr.opcode() {
            Opcode::Add => {
                let (mut n,of) = self.read_reg(cur_instr.reg1()).overflowing_add(self.read_reg(cur_instr.reg2()));
                if of {
                    n = n.wrapping_add(self.read_reg(cur_instr.reg1()));
                }
                self.write_reg(cur_instr.reg1(), n);
                self.set_flags(n,of);
            }, 

            Opcode::Mul => {
                let (mut n,of) = self.read_reg(cur_instr.reg1()).overflowing_mul(self.read_reg(cur_instr.reg2()));
                if of {
                    n = n.wrapping_mul(self.read_reg(cur_instr.reg1()));
                }
                self.write_reg(cur_instr.reg1(), n);
                self.set_flags(n,of);
            }, 

            Opcode::Ld  => {
                let n = self.read_from_memory(cur_instr.addr(), 1).pop().expect("Received empty block from read_from_memory()").1;
                self.write_reg(cur_instr.reg1(), n);
                self.set_flags(n,false);
            }, 

            Opcode::Sav  => {
                let n = self.read_reg(cur_instr.reg1());
                self.write_to_memory(vec![(cur_instr.addr(), n)]);
                self.set_flags(n,false);
            },

            Opcode::Push => {
                self.ESP -= 1; 
                let n = self.read_reg(cur_instr.reg1());
                self.write_to_memory(vec![(self.ESP, n)]);
                self.set_flags(n,false);
            },

            Opcode::Pop => {
                self.ESP += 1;
                assert!(self.ESP >= 0);
                let n = self.read_from_memory(self.ESP, 1).pop().expect("Received empty block from read_from_memory()").1;
                self.write_reg(cur_instr.reg1(), n);
                self.set_flags(n,false);
            },
            Opcode::Jz => if self.ZERO {
                self.ISP = cur_instr.addr();
                self.set_flags(0,false);
            },
            Opcode::Jgz => if !self.SIGN {
                self.ISP = cur_instr.addr();
                self.set_flags(0,false);
            },
            Opcode::Jlz => if self.SIGN {
                self.ISP = cur_instr.addr();
                self.set_flags(0,false);
            }, 
            Opcode::Nop => self.set_flags(0,false), 
        }
    }

    fn read_instr_at(&mut self, addr:u64) -> Instruction {
        if let Ok(opcode) = self.read_from_pipe(addr) {
            Instruction(opcode)
        }
        else {
            let mut mem_block = self.read_from_memory(addr, PIPE_SIZE);
            let mut i:usize = PIPE_SIZE-1;
            while let Some(item) = mem_block.pop(){
                self.pipe[i] = item;
                i -= 1;
            }
            Instruction(self.pipe[0].1)
        }
    }

    fn read_from_memory(&self, start_addr:u64, num:usize) -> Vec<(u64, u64)> {
        self.tx. send(CPUBusOp::RequestBlock(start_addr,num));
        match self.rx.recv().expect("CPUBus has disconnected unexpectedly") {
            CPUBusOp::GiveBlock(res_vec) => res_vec,
            CPUBusOp::RequestBlock(_,_) => panic!("Unexpectedly received RequestBlock in read_from_memory()"),
            CPUBusOp::Error(err) => panic!("CPUBus returned error in read_from_memory(): {}", err),
            op => panic!("Unimplemented CPUBusOp:{:?}", op),
        }
    }

    fn write_to_memory(&self, values:Vec<(u64, u64)>) {
        self.tx.send(CPUBusOp::GiveBlock(values));
    }
}

pub struct CPU {
    //cache:[u64, CACHE_SIZE],
    cores:Vec<(Core, Sender<CPUBusOp>, Receiver<CPUBusOp>)>,
    //BUS
    tx:Sender<MemBusOp>,
    rx:Receiver<MemBusOp>,
}  

impl CPU {
    pub fn new(_tx:Sender<MemBusOp>, _rx:Receiver<MemBusOp>) -> CPU {
        let mut _cores:Vec<(Core, Sender<CPUBusOp>, Receiver<CPUBusOp>)> = Vec::new();
        for i in 0..CORE_NUM {
            let (tx_cpu, rx_core) = channel();
            let (tx_core, rx_cpu) = channel();
            _cores.push((Core::new(tx_core, rx_core), tx_cpu, rx_cpu));
        }
        CPU{cores:_cores,
            tx:_tx,
            rx:_rx,}
    }

    pub fn exec(&mut self) {
        
        loop {
            while let Ok(op) = self.rx.try_recv() {
                if let MemBusOp::GiveBlock(id, block) = op {
                    if let Err(_) = self.cores.iter().find(|&&(ref c, _, _)| id == c.ID).expect("Unexpected ProcessorID in cpu::exec()!").1.send(CPUBusOp::GiveBlock(block)) {
                        panic!("Channel from CPU to Core has closed unexpectedly in CPU::exec()");
                    }
                } else {
                    panic!("Unexpected MemBusOp in cpu::exec()");
                }
            }
            for &(ref core, ref tx, ref rx) in self.cores.iter() {
                while let Ok(op) = rx.try_recv() {
                    if let CPUBusOp::RequestBlock(addr, size) = op {
                        if let Err(_) = self.tx.send(MemBusOp::RequestBlock(core.ID, addr, size)) {
                            panic!("Channel from CPU to Motherboard has closed unexpectedly in CPU::exec()");
                        }
                    } else {
                        panic!("Unexpected MemBusOp while processing memory requests of cores in cpu::exec()");
                    }
                }
            }
        }
    }
}


