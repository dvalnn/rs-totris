use std::num::TryFromIntError;

use super::{Coordinate, Offset};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation { N, E, S, W }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind { I, O, T, S, Z, J, L }

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;

    pub fn new(kind: Kind) -> Self {
        Piece {
            kind,
            position: Offset::new(0, 0),
            rotation: Rotation::N,
        }
    }

    fn rotator(&self) -> impl Fn(Offset) -> Offset + '_ {
        move |cell| cell * self.rotation
    }

    fn positioner(&self) -> impl Fn(Offset) -> Offset + '_ {
        move |cell| cell + self.position
    }

    pub fn cells(&self) -> Result<Vec<Coordinate>, TryFromIntError> {
        self.kind
            .cells()
            .map(self.rotator())
            .map(self.positioner())
            .map(|cell| {
                Ok::<Coordinate, TryFromIntError>(Coordinate::new(
                    usize::try_from(cell.x)?,
                    usize::try_from(cell.y)?,
                ))
            })
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }
}

impl Kind {
    #[rustfmt::skip]
    pub const ALL: [Kind; 7] = [
        Self::I, Self::O, Self::T, Self::S, Self::Z, Self::J, Self::L,
    ];

    #[rustfmt::skip]
    fn cells(&self) -> [Offset; Piece::CELL_COUNT]{
        match self {
            Kind::O => &[( 0, 0), (1, 0), (0, 1), ( 1, 1)],
            Kind::I => &[(-1, 0), (0, 0), (1, 0), ( 2, 0)],
            Kind::T => &[(-1, 0), (0, 0), (0, 1), ( 0, 1)],
            Kind::L => &[(-1, 0), (0, 0), (0, 1), ( 1, 1)],
            Kind::J => &[(-1, 0), (0, 0), (0, 1), (-1, 1)],
            Kind::S => &[(-1, 0), (0, 0), (1, 0), ( 1, 1)],
            Kind::Z => &[(-1, 1), (0, 1), (0, 0), ( 1, 0)],
        }.map(Offset::from)
    }
}

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
