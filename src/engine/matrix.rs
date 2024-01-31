use std::ops::{Index, IndexMut};

use super::{piece::Piece, Coordinate};

pub struct Matrix([bool; Matrix::SIZE]);

impl Matrix {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        x + y * Self::WIDTH
    }

    pub(super) fn new() -> Self {
        Self([false; Self::SIZE])
    }

    #[rustfmt::skip]
    pub(super) fn is_clipping(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else { return true; };
        cells.iter().any(|coord| !Matrix::on_matrix(*coord) || self[*coord])
    }

    #[rustfmt::skip]
    pub(super) fn is_placeable(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else { return false; };
        cells.iter().all(|coord| Matrix::on_matrix(*coord) && !self[*coord])
    }

    pub(super) fn on_matrix(coord: Coordinate) -> bool {
        Self::valid_coord(coord) && coord.y < Self::HEIGHT
    }

    pub(super) fn valid_coord(coord: Coordinate) -> bool {
        coord.x < Self::WIDTH
    }
}

impl Index<Coordinate> for Matrix {
    type Output = bool;

    fn index(&self, index: Coordinate) -> &Self::Output {
        assert!(Self::on_matrix(index));
        &self.0[Self::indexing(index)]
    }
}

impl IndexMut<Coordinate> for Matrix {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        assert!(Self::on_matrix(index));
        &mut self.0[Self::indexing(index)]
    }
}
