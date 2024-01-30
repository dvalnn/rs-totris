use super::{Board, Coordinate, Offset};
use anyhow::{Context, Error, Result};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation { N, E, S, W }

impl std::ops::Mul<Rotation> for Offset {
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::N => self,
            Rotation::E => Offset::new(self.y, -self.x),
            Rotation::S => Offset::new(-self.x, -self.y),
            Rotation::W => Offset::new(-self.y, self.x),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind { I, O, T, S, Z, J, L }

impl Kind {
    const KIND_COUNT: usize = 7;

    #[rustfmt::skip]
    pub(super) const ALL: [Kind; Self::KIND_COUNT] = [
        Self::I, Self::O, Self::T, Self::S, Self::Z, Self::J, Self::L,
    ];

    #[rustfmt::skip]
    fn cells(&self) -> [Offset; Piece::CELL_COUNT]{
        match self {
            Kind::O => &[( 0, 0), (1, 0), (0, 1), ( 1, 1)],
            Kind::I => &[(-1, 0), (0, 0), (1, 0), ( 2, 0)],
            Kind::T => &[(-1, 0), (0, 0), (1, 0), ( 0, 1)],
            Kind::L => &[(-1, 0), (0, 0), (1, 0), ( 1, 1)],
            Kind::J => &[(-1, 0), (0, 0), (1, 0), (-1, 1)],
            Kind::S => &[(-1, 0), (0, 0), (0, 1), ( 1, 1)],
            Kind::Z => &[(-1, 1), (0, 1), (0, 0), ( 1, 0)],
        }.map(Offset::from)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

//NOTE: Private functions impl block
impl Piece {
    const CELL_COUNT: usize = 4;

    fn rotator(&self) -> impl Fn(Offset) -> Offset + '_ {
        move |cell| match self.kind {
            Kind::O => cell,
            //TODO: implement I piece specific rotation
            // Kind::I => todo!(),
            _ => cell * self.rotation,
        }
    }

    fn positioner(&self) -> impl Fn(Offset) -> Result<Coordinate> + '_ {
        move |cell| {
            let cell = cell + self.position;
            Ok::<Coordinate, Error>(Coordinate::new(
                usize::try_from(cell.x)?,
                usize::try_from(cell.y)?,
            ))
            .context("Piece is out of bounds (underflow)")
        }
    }

    fn board_bounds_checker(
        &self,
    ) -> impl Fn(Coordinate) -> Result<Coordinate> + '_ {
        move |cell| match Board::in_bounds(cell) {
            true => Ok(cell),
            false => Err(Error::msg("Piece is out of bounds (overflow)")),
        }
    }
}

//NOTE: Public functions impl block
impl Piece {
    pub fn new(kind: Kind) -> Self {
        Piece {
            kind,
            position: Offset::new(0, 0),
            rotation: Rotation::N,
        }
    }

    pub fn cells(&self) -> Result<Vec<Coordinate>> {
        self.kind
            .cells()
            .map(self.rotator())
            .map(self.positioner())
            .into_iter()
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(self.board_bounds_checker())
            .collect::<Result<Vec<_>>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_cells_o() {
        let piece = Piece::new(Kind::O);
        let cells = piece.cells().expect("Should be a valid O piece");
        assert_eq!(
            cells,
            vec![
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(0, 1),
                Coordinate::new(1, 1),
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_cells_i() {
        let piece = Piece::new(Kind::I);
        let cells = piece.cells().unwrap();
    }

    #[rstest]
    #[case(
        Piece{
            kind: Kind::Z,
            position: Offset::new(5, 6),
            rotation: Rotation::W
        },
        vec![
            Coordinate::new(4, 5),
            Coordinate::new(4, 6),
            Coordinate::new(5, 6),
            Coordinate::new(5, 7)
        ]
    )]
    #[case(
        Piece{
            kind: Kind::L,
            position: Offset::new(8, 2),
            rotation: Rotation::S
        },
        vec![
            Coordinate::new(9, 2),
            Coordinate::new(8, 2),
            Coordinate::new(7, 2),
            Coordinate::new(7, 1),
        ]
    )]
    fn test_positioning(
        #[case] piece: Piece,
        #[case] expected: Vec<Coordinate>,
    ) {
        let cells = piece.cells().expect("Should be a valid S piece");
        assert_eq!(cells, expected);
    }
}
