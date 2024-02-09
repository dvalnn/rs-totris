use std::{
    ops::{Index, IndexMut},
    slice::ArrayChunks,
};

use super::{geometry::GridIncrement, piece::Piece, Coordinate};

#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color { Yellow, Cyan, Purple, Orange, Blue, Green, Red, }

type Cell = Option<Color>;
pub struct Matrix(pub(super) [Cell; Matrix::SIZE]);

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
        cells.iter().any(|coord| {
            !Self::valid_coord(*coord)
                || (Self::on_matrix(*coord) && self[*coord].is_some())
        })
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

    pub(super) fn full_lines(&self) -> Vec<usize> {
        self.lines()
            .enumerate()
            .filter_map(|(i, line)| {
                line.iter().all(Option::is_some).then_some(i)
            })
            .collect()
    }

    pub(super) fn clear_lines(&mut self, indices: &[usize]) {
        debug_assert!(indices.is_sorted());
        for &line in indices.iter().rev() {
            // override the line to clear with the remainder of the matrix
            let start_of_remainder = Self::WIDTH * (line + 1);
            self.0.copy_within(start_of_remainder.., line * Self::WIDTH);
            self.0[Self::SIZE - Self::WIDTH..].fill(None);
        }
    }

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        x + y * Self::WIDTH
    }

    fn lines(&self) -> ArrayChunks<'_, Cell, { Self::WIDTH }> {
        self.0.array_chunks()
    }
}

impl Index<Coordinate> for Matrix {
    type Output = Cell;

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
    pub(super) cells: std::slice::Iter<'matrix, Cell>,
}

impl<'matrix> Iterator for CellIter<'matrix> {
    type Item = (Coordinate, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        let &cell = self.cells.next()?;
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
                (Coordinate::new(0, 0), None),
                (Coordinate::new(1, 0), None),
                (Coordinate::new(2, 0), Some(Color::Yellow)),
                (Coordinate::new(3, 0), None),
                (Coordinate::new(4, 0), None),
            ]
        );

        let other_item = iter.by_ref().nth(8);
        assert_eq!(
            other_item,
            Some((Coordinate::new(3, 1), Some(Color::Cyan))),
        );

        assert!(iter.all(|(_, color)| color.is_none()));
    }
}
