#![allow(dead_code, unused_variables)]

mod engine;
mod interface;

use engine::Engine;

fn main() {
    println!("Hello, world!");
    let engine = Engine::new();
    interface::run(engine);
}
