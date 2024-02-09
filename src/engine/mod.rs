mod geometry;
mod matrix;
mod piece;

pub mod kick_tables;

use std::{option::Option, time::Duration};

use cgmath::EuclideanSpace;

use self::piece::Piece;

pub use self::{
    matrix::{CellIter, Color, Matrix},
    piece::{Kind as PieceKind, RotateKind, Rotation},
};

pub type Coordinate = cgmath::Point2<usize>;
pub type Offset = cgmath::Vector2<isize>;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    held_cursor: Option<Piece>,
    level: u8,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub(crate) fn new() -> Self {
        Engine {
            matrix: Matrix::new(),
            bag: Vec::new(),
            cursor: None,
            held_cursor: None,
            level: 1,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_matrix(matrix: Matrix) -> Self {
        Engine {
            matrix,
            ..Self::new()
        }
    }

    pub(crate) fn add_cursor(&mut self) {
        if self.bag.is_empty() {
            self.refill_bag();
        }
        let kind = self.bag.pop().expect("Bag is empty");
        self.cursor = Some(Piece::new(kind));
    }

    pub(crate) fn cursor_has_hit_bottom(&self) -> bool {
        self.ticked_down_cursor().is_none()
    }

    pub(crate) fn rotate_cursor(
        &mut self,
        kind: RotateKind,
        kick: Option<Offset>,
    ) -> Result<(), ()> {
        let cursor = self.cursor.as_mut().ok_or(())?;
        let mut new_cursor = cursor.rotated_by(kind);
        if let Some(kick) = kick {
            new_cursor = new_cursor.moved_by(kick);
        }
        match self.matrix.is_clipping(&new_cursor) {
            true => Err(()),
            false => {
                self.cursor = Some(new_cursor);
                Ok(())
            }
        }
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

        //NOTE: Maybe good idea? Involves changing logic in the game module
        // self.place_cursor()
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

    pub(crate) fn line_clear(&mut self, mut animation: impl FnMut(&[usize])) {
        let lines: Vec<usize> = self.matrix.full_lines();
        if lines.is_empty() {
            return;
        }
        animation(lines.as_slice());
        self.matrix.clear_lines(lines.as_slice());
    }

    pub(crate) fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut rng);
    }

    pub(crate) fn hold_cursor(&mut self) {
        let cursor = self.cursor.take().expect("No cursor");
        if self.held_cursor.is_none() {
            self.held_cursor = Some(cursor);
            self.add_cursor();
        } else {
            self.cursor = self.held_cursor;
            self.held_cursor = Some(cursor);
        }
    }

    pub(crate) fn place_cursor(&mut self) {
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

    pub(crate) fn cursor_info(
        &self,
    ) -> Option<(Vec<Coordinate>, Color, PieceKind, Rotation)> {
        let cursor = self.cursor?;
        Some((
            cursor.cells()?,
            cursor.kind.color(),
            cursor.kind,
            cursor.rotation,
        ))
    }

    pub(crate) fn held_cursor_info(&self) -> Option<(Vec<Offset>, Color)> {
        let cursor = self.held_cursor?;
        Some((cursor.default_cells(), cursor.kind.color()))
    }

    fn ticked_down_cursor(&self) -> Option<Piece> {
        let cursor = self.cursor?;
        let new_cursor = cursor.moved_by(Offset::new(0, -1));
        (!self.matrix.is_clipping(&new_cursor)).then_some(new_cursor)
    }
}
