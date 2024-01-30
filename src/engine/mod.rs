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
        let cursor = self
            .cursor
            .take()
            .expect("Place cursor called without cursor");

        for coords in cursor.cells().expect("cursor out of bounds") {
            let cell = self
                .board
                .get_mut(coords)
                .expect("cursor out of bounds ?!?!?!");

            // NOTE: The calling code should ensure this never happens
            debug_assert!(!(*cell));
            *cell = true;
        }
    }
}

pub struct Board([bool; Board::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn new() -> Self {
        Self([false; Self::SIZE])
    }

    const fn index(Coordinate { x, y }: Coordinate) -> usize {
        x + y * Self::WIDTH
    }

    pub fn in_bounds(Coordinate { x, y }: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    pub fn get_mut(&mut self, coordinate: Coordinate) -> Option<&mut bool> {
        Self::in_bounds(coordinate)
            .then(|| &mut self.0[Self::index(coordinate)])
    }
}
