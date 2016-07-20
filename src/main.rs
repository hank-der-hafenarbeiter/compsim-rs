use std::sync::mpsc::{Sender, Receiver};

mod CPU;
mod utils;

struct Computer {
    processor:CPU,
    memory:RAM,

}

fn main() {
    println!("Hello, world!");
}
