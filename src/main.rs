#![allow(dead_code)]

mod engine;
mod interface;

use engine::Engine;

fn main() {
    println!("Hello, world!");
    let engine = Engine::new();
    interface::run(engine);
}
