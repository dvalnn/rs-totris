#![allow(dead_code)]

mod engine;
mod interface;

use engine::{Color, Coordinate, Engine, Matrix};

fn main() {
    println!("Hello, world!");
    let mut matrix = Matrix::new();
    matrix[Coordinate::new(2, 2)] = Some(Color::Red);
    matrix[Coordinate::new(3, 2)] = Some(Color::Red);
    matrix[Coordinate::new(4, 2)] = Some(Color::Red);
    matrix[Coordinate::new(5, 2)] = Some(Color::Red);
    let engine = Engine::from_matrix(matrix);
    interface::run(engine);
}
