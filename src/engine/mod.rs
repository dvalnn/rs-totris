#![allow(dead_code, unused_variables)]

mod piece;

use piece::Kind as PieceKind;

pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::new(),
            bag: Vec::new(),
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
