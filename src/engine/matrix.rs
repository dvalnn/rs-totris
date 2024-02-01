use std::ops::{Index, IndexMut};

use super::{geometry::GridIncrement, piece::Piece, Coordinate};

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color { Yellow, Cyan, Purple, Orange, Blue, Green, Red, }

type Cell = Option<Color>;
pub struct Matrix(pub(super) [Option<Color>; Matrix::SIZE]);

impl Matrix {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;
    pub const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    pub(crate) fn new() -> Self {
        Self([None; Self::SIZE])
    }

    pub(super) fn is_clipping(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else {
            return true;
        };
        cells
            .iter()
            .any(|coord| !Matrix::on_matrix(*coord) || self[*coord].is_some())
    }

    pub(super) fn is_placeable(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else {
            return false;
        };
        cells
            .iter()
            .all(|coord| Matrix::on_matrix(*coord) && self[*coord].is_none())
    }

    pub(super) fn on_matrix(coord: Coordinate) -> bool {
        Self::valid_coord(coord) && coord.y < Self::HEIGHT
    }

    pub(super) fn valid_coord(coord: Coordinate) -> bool {
        coord.x < Self::WIDTH
    }

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        x + y * Self::WIDTH
    }
}

impl Index<Coordinate> for Matrix {
    type Output = Option<Color>;

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

pub struct CellIter<'matrix> {
    pub(super) position: Coordinate,
    pub(super) cells: std::slice::Iter<'matrix, Option<Color>>,
}

impl<'matrix> Iterator for CellIter<'matrix> {
    type Item = (Coordinate, &'matrix Option<Color>);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(cell) = self.cells.next() else {
            return None;
        };

        let coord = self.position;
        self.position.grid_inc();

        Some((coord, cell))
    }
}

#[cfg(test)]
mod test {

    use cgmath::EuclideanSpace;

    use super::*;

    #[test]
    fn cell_iter() {
        let mut matrix = Matrix::new();
        matrix[Coordinate::new(2, 0)] = Some(Color::Yellow);
        matrix[Coordinate::new(3, 1)] = Some(Color::Cyan);

        let mut iter = CellIter {
            position: Coordinate::origin(),
            cells: matrix.0.iter(),
        };

        let first_five = iter.by_ref().take(5).collect::<Vec<_>>();
        assert_eq!(
            first_five,
            [
                (Coordinate::new(0, 0), &None),
                (Coordinate::new(1, 0), &None),
                (Coordinate::new(2, 0), &Some(Color::Yellow)),
                (Coordinate::new(3, 0), &None),
                (Coordinate::new(4, 0), &None),
            ]
        );

        let other_item = iter.by_ref().skip(8).next();
        assert_eq!(
            other_item,
            Some((Coordinate::new(3, 1), &Some(Color::Cyan))),
        );

        assert!(iter.all(|(_, color)| color.is_none()));
    }
}
