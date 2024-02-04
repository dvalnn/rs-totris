#![allow(dead_code)]
#![feature(array_chunks, is_sorted)]

mod engine;
mod interface;

use engine::{Color, Engine, Matrix};

fn main() {
    println!("Hello, world!");
    //TODO: Remove this
    let mut matrix = Matrix::new();
    for col in 0..=6 {
        matrix[(col, 0).into()] = Some(Color::Green);
    }

    let mut engine = Engine::from_matrix(matrix);
    engine.debug_add_cursor();
    interface::run(engine);
}
