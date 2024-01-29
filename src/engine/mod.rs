#![allow(dead_code, unused_variables)]

struct Engine {
    board: Board,
}

impl Engine {
    fn new() -> Self {
        Engine {
            board: Board::new()
        }
    }
}

struct Board([bool; Board::SIZE]);

impl Board{
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn new() -> Self{
        Self([false; Self::SIZE])
    }
}
