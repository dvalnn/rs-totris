#![allow(dead_code, unused_variables)]

mod engine;
mod interface;

use engine::Engine;
use interface::Interface;

fn main() {
    println!("Hello, world!");
    let engine = Engine::new(); 
    Interface::run(engine);
}
