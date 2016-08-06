#![feature(step_by)]
extern crate snowflake;
#[macro_use]extern crate enum_primitive;
extern crate num;

mod cpu;
mod utils;
mod parser;
mod test;

use std::sync::mpsc::{Sender,Receiver, channel};
use utils::MemBusOp;

const RAM_SIZE:usize = 1_000_000;

struct Ram {
    memory:[i64; RAM_SIZE],
    tx:Sender<MemBusOp>,
    rx:Receiver<MemBusOp>,
}

impl Ram {

    pub fn new(_tx:Sender<MemBusOp>, _rx:Receiver<MemBusOp>) -> Ram {
       Ram { memory:[0;RAM_SIZE], tx:_tx, rx:_rx}
    }
}

struct Motherboard {
    processor:cpu::CPU,
    processor_bus:(Sender<MemBusOp>, Receiver<MemBusOp>),
    memory_bus:(Sender<MemBusOp>, Receiver<MemBusOp>),
    memory:Ram,
}

impl Motherboard {
    
    pub fn new() -> Motherboard {
        let (m_cpu_tx, cpu_m_rx) = channel();
        let (cpu_m_tx, m_cpu_rx) = channel();

        let (m_mem_tx, mem_m_rx) = channel();
        let (mem_m_tx, m_mem_rx) = channel();
        Motherboard {   processor:cpu::CPU::new(cpu_m_tx, cpu_m_rx),
                        processor_bus:(m_cpu_tx, m_cpu_rx),
                        memory:Ram::new(mem_m_tx, mem_m_rx),
                        memory_bus:(m_mem_tx, m_mem_rx),
        }
    }
}

fn main() {


}
