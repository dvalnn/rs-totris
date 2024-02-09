// #![allow(dead_code)]
#![feature(array_chunks, is_sorted)]

mod engine;
mod game;
mod interface;

use crate::{engine::Engine, game::Game};

fn main() {
    println!("Hello, world!");
    interface::run(Game::new(Engine::new()));
}
