mod geometry;
mod matrix;
mod piece;

use std::{option::Option, time::Duration};

use cgmath::EuclideanSpace;

use piece::{Kind as PieceKind, Piece};

pub(crate) use matrix::{CellIter, Color, Matrix};

pub(crate) type Coordinate = cgmath::Point2<usize>;
pub(crate) type Offset = cgmath::Vector2<isize>;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub(crate) enum RotateKind { Clockwise, CounterClockwise }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub(crate) enum MoveKind { Left, Right }

impl MoveKind {
    fn offset(&self) -> Offset {
        match self {
            Self::Left => Offset::new(-1, 0),
            Self::Right => Offset::new(1, 0),
        }
    }
}

pub(crate) struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    cursor: Option<Piece>,
    level: u8,
}

impl Engine {
    pub(crate) fn new() -> Self {
        Engine {
            matrix: Matrix::new(),
            bag: Vec::new(),
            cursor: None,
            level: 1,
        }
    }

    pub(crate) fn from_matrix(matrix: Matrix) -> Self {
        Engine {
            matrix,
            ..Self::new()
        }
    }

    pub(crate) fn debug_add_cursor(&mut self) {
        self.cursor = Some(Piece::new(PieceKind::J));
    }

    pub(crate) fn cursor_has_hit_bottom(&self) -> bool {
        self.ticked_down_cursor().is_none()
    }

    pub(crate) fn rotate_cursor(&mut self, kind: RotateKind) -> Result<(), ()> {
        todo!("rotate cursor {:?}", kind);
    }

    pub(crate) fn move_cursor(&mut self, kind: MoveKind) -> Result<(), ()> {
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

    pub(crate) fn tick_down(&mut self) {
        self.cursor = Some(
            self.ticked_down_cursor()
                .expect("tried to tick down to invalid position"),
        );
    }

    pub(crate) fn hard_drop(&mut self) {
        while let Some(new_cursor) = self.ticked_down_cursor() {
            self.cursor = Some(new_cursor);
        }
        self.place_cursor()
    }

    pub(crate) fn drop_time(&self) -> Duration {
        let level = self.level - 1;

        Duration::from_secs_f32(
            (0.8 - ((level) as f32 * 0.007)).powi(level as _),
        )
    }

    pub(crate) fn cells(&self) -> CellIter<'_> {
        CellIter {
            position: Coordinate::origin(),
            cells: self.matrix.0.iter(),
        }
    }

    pub(crate) fn cursor_info(&self) -> Option<(Vec<Coordinate>, Color)> {
        let cursor = self.cursor?;
        Some((cursor.cells()?, cursor.kind.color()))
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
        let Some(cursor) = self.cursor else {
            return None;
        };
        let new_cursor = cursor.moved_by(Offset::new(0, -1));
        (!self.matrix.is_clipping(&new_cursor)).then_some(new_cursor)
    }
}
