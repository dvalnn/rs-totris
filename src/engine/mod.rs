mod geometry;
mod matrix;
mod piece;

use std::{option::Option, time::Duration};

use cgmath::EuclideanSpace;

use piece::{Kind as PieceKind, Piece};

pub use matrix::{CellIter, Color, Matrix};
pub use piece::RotateKind;

pub type Coordinate = cgmath::Point2<usize>;
pub type Offset = cgmath::Vector2<isize>;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum MoveKind { Left, Right }

impl MoveKind {
    fn offset(&self) -> Offset {
        match self {
            Self::Left => Offset::new(-1, 0),
            Self::Right => Offset::new(1, 0),
        }
    }
}

pub struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    cursor: Option<Piece>,
    level: u8,
    drop_timer: Duration,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::new(),
            bag: Vec::new(),
            cursor: None,
            level: 1,
            drop_timer: Duration::default(),
        }
    }

    pub fn from_matrix(matrix: Matrix) -> Self {
        Engine {
            matrix,
            ..Self::new()
        }
    }

    pub fn add_cursor(&mut self) {
        if self.bag.is_empty() {
            self.refill_bag();
        }
        let kind = self.bag.pop().expect("Bag is empty");
        self.cursor = Some(Piece::new(kind));
    }

    pub fn cursor_has_hit_bottom(&self) -> bool {
        self.ticked_down_cursor().is_none()
    }

    pub fn rotate_cursor(&mut self, kind: RotateKind) -> Result<(), ()> {
        let cursor = self.cursor.as_mut().ok_or(())?;
        let new_cursor = cursor.rotated_by(kind);
        match self.matrix.is_clipping(&new_cursor) {
            true => Err(()),
            false => {
                self.cursor = Some(new_cursor);
                Ok(())
            }
        }
    }

    pub fn move_cursor(&mut self, kind: MoveKind) -> Result<(), ()> {
        let Some(cursor) = self.cursor.as_mut() else {
            return Ok(());
        };

        let new_cursor = cursor.moved_by(kind.offset());
        if new_cursor.cells().is_none() {
            return Err(());
        }

        if self.matrix.is_clipping(&new_cursor) {
            return Err(());
        }

        self.cursor = Some(new_cursor);
        Ok(())
    }

    pub fn tick_down(&mut self) {
        self.cursor = Some(
            self.ticked_down_cursor()
                .expect("tried to tick down to invalid position"),
        );
    }

    pub fn hard_drop(&mut self) {
        while let Some(new_cursor) = self.ticked_down_cursor() {
            self.cursor = Some(new_cursor);
        }
        self.place_cursor()
    }

    pub fn drop_time(&self) -> Duration {
        let level = self.level - 1;

        Duration::from_secs_f32(
            (0.8 - ((level) as f32 * 0.007)).powi(level as _),
        )
    }

    pub fn cells(&self) -> CellIter<'_> {
        CellIter {
            position: Coordinate::origin(),
            cells: self.matrix.0.iter(),
        }
    }

    pub fn cursor_info(&self) -> Option<(Vec<Coordinate>, Color)> {
        let cursor = self.cursor?;
        Some((cursor.cells()?, cursor.kind.color()))
    }

    pub fn line_clear(&mut self, mut animation: impl FnMut(&[usize])) {
        let lines: Vec<usize> = self.matrix.full_lines();
        if lines.is_empty() {
            return;
        }
        animation(lines.as_slice());
        self.matrix.clear_lines(lines.as_slice());
    }

    pub fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut rng);
    }

    pub fn place_cursor(&mut self) {
        let cursor = self
            .cursor
            .take()
            .expect("Place cursor called without cursor");

        debug_assert!(
            self.matrix.is_placeable(&cursor),
            "Tried to place cursor in unplaceable location: {:?}",
            cursor
        );

        let color = cursor.kind.color();
        for coords in cursor.cells().expect("cursor out of bounds !??!?!") {
            self.matrix[coords] = Some(color);
        }
    }

    fn ticked_down_cursor(&self) -> Option<Piece> {
        let cursor = self.cursor?;
        let new_cursor = cursor.moved_by(Offset::new(0, -1));
        (!self.matrix.is_clipping(&new_cursor)).then_some(new_cursor)
    }
}
