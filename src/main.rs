use std::sync::mpsc::{Sender, Receiver};

mod CPU;
mod utils;

struct RAM {
    placeholder:i64
}

struct Computer {
    processor:CPU::CPU,
    memory:RAM,

}

fn main() {
    println!("Hello, world!");
}
