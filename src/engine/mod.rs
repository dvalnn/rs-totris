#![allow(dead_code, unused_variables)]

mod piece;

use piece::{Kind as PieceKind, Piece};

type Coordinate = cgmath::Vector2<usize>;
type Offset = cgmath::Vector2<isize>;

pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::new(),
            bag: Vec::new(),
            cursor: None,
        }
    }

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut rng);
    }

    fn place_cursor(&mut self) {
        // debug assert that the cursor does not overlap with the board
        todo!();
    }
}

struct Board([bool; Board::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn new() -> Self {
        Self([false; Self::SIZE])
    }
}
