#![allow(dead_code)]
#![feature(array_chunks, is_sorted)]

mod engine;
mod interface;

use engine::Engine;
fn main() {
    println!("Hello, world!");
    interface::run(Engine::new());
}
