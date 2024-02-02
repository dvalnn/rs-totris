#![allow(dead_code)]

mod engine;
mod interface;

use engine::{Color, Coordinate, Engine, Matrix};

fn main() {
    println!("Hello, world!");
    //TODO: Remove this
    let mut matrix = Matrix::new();
    matrix[Coordinate::new(2, 2)] = Some(Color::Red);
    matrix[Coordinate::new(3, 2)] = Some(Color::Red);
    matrix[Coordinate::new(4, 2)] = Some(Color::Red);
    matrix[Coordinate::new(5, 2)] = Some(Color::Red);

    let mut engine = Engine::from_matrix(matrix);
    engine.debug_add_cursor();
    interface::run(engine);
}
