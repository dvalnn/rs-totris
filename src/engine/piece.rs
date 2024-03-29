use super::{matrix::Color, Coordinate, Matrix, Offset};
use cgmath::{EuclideanSpace, Zero};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RotateKind { Clockwise, CounterClockwise }

impl std::ops::Add<RotateKind> for Rotation {
    type Output = Self;

    fn add(self, kind: RotateKind) -> Self::Output {
        let index = Rotation::iter()
            .position(|dir| dir == self)
            .expect("invalid rotation");

        Rotation::iter()
            .cycle()
            .nth(match kind {
                RotateKind::Clockwise => index + 1,
                RotateKind::CounterClockwise => index + 3,
            })
            .expect("invalid rotation")
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum Rotation { N, E, S, W }

impl Rotation {
    fn i_offset(&self) -> Offset {
        match self {
            Rotation::N => Offset::zero(),
            Rotation::E => Offset::new(1, 0),
            Rotation::S => Offset::new(1, -1),
            Rotation::W => Offset::new(0, -1),
        }
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

    pub fn color(&self) -> Color {
        match self {
            Kind::I => Color::Cyan,
            Kind::O => Color::Yellow,
            Kind::T => Color::Purple,
            Kind::S => Color::Green,
            Kind::Z => Color::Red,
            Kind::J => Color::Blue,
            Kind::L => Color::Orange,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;
    const SPAWN_POSITION: Offset =
        Offset::new((Matrix::WIDTH / 2) as isize, Matrix::HEIGHT as isize);

    pub fn new(kind: Kind) -> Self {
        //TODO: look into integrating this with the Kind cells array
        let spawn_offset = match kind {
            Kind::I | Kind::O => Offset::new(-1, 0),
            _ => Offset::zero(),
        };

        Piece {
            kind,
            position: Self::SPAWN_POSITION + spawn_offset,
            rotation: Rotation::N,
        }
    }

    pub(super) fn moved_by(&self, offset: Offset) -> Self {
        Self {
            position: self.position + offset,
            ..*self
        }
    }

    pub(super) fn rotated_by(&self, kind: RotateKind) -> Self {
        let rotation = self.rotation + kind;
        Self { rotation, ..*self }
    }

    pub(super) fn reset(self) -> Self {
        Self::new(self.kind)
    }

    /// Returns the cells of this [`Piece`].
    /// If the piece is out of bounds, `None` is returned.
    pub(super) fn cells(&self) -> Option<Vec<Coordinate>> {
        self.kind
            .cells()
            .map(self.rotator())
            .map(self.positioner())
            .into_iter()
            .collect::<Option<Vec<_>>>()
    }

    pub(super) fn default_cells(&self) -> Vec<Offset> {
        self.kind.cells().into_iter().collect()
    }

    fn rotator(&self) -> impl Fn(Offset) -> Offset + '_ {
        move |offset| match self.kind {
            Kind::O => offset,
            Kind::I => offset * self.rotation + self.rotation.i_offset(),
            _ => offset * self.rotation,
        }
    }

    fn positioner(&self) -> impl Fn(Offset) -> Option<Coordinate> + '_ {
        move |offset| {
            let cell = offset + self.position;
            let positive_offset = cell.cast::<usize>()?;
            let coord = Coordinate::from_vec(positive_offset);
            Matrix::valid_coord(coord).then_some(coord)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_cells_o() {
        let piece = Piece {
            kind: Kind::O,
            position: Offset::zero(),
            rotation: Rotation::N,
        };

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
        let piece = Piece {
            kind: Kind::I,
            position: Offset::zero(),
            rotation: Rotation::N,
        };
        let _ = piece.cells().unwrap();
    }

    #[rstest]
    //NOTE: case 1
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
    //NOTE: case 2
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
    //NOTE: case 3
    #[case(
        Piece{
            kind: Kind::I,
            position: Offset::new(5,5),
            rotation: Rotation::N
        },
        vec![
            Coordinate::new(4,5),
            Coordinate::new(5,5),
            Coordinate::new(6,5),
            Coordinate::new(7,5),
        ]
    )]
    //NOTE: case 4
    #[case(
        Piece{
            kind: Kind::I,
            position: Offset::new(5,5),
            rotation: Rotation::E
        },
        vec![
            Coordinate::new(6,6),
            Coordinate::new(6,5),
            Coordinate::new(6,4),
            Coordinate::new(6,3),
        ]
    )]
    //NOTE: case 5
    #[case(
        Piece{
            kind: Kind::I,
            position: Offset::new(5,5),
            rotation: Rotation::S
        },
        vec![
            Coordinate::new(7,4),
            Coordinate::new(6,4),
            Coordinate::new(5,4),
            Coordinate::new(4,4),
        ]
    )]
    //NOTE: case 6
    #[case(
        Piece{
            kind: Kind::I,
            position: Offset::new(5,5),
            rotation: Rotation::W
        },
        vec![
            Coordinate::new(5,3),
            Coordinate::new(5,4),
            Coordinate::new(5,5),
            Coordinate::new(5,6),
        ]
    )]
    fn test_positioning(
        #[case] piece: Piece,
        #[case] expected: Vec<Coordinate>,
    ) {
        let cells = piece.cells().expect("Should be a valid S piece");
        assert_eq!(cells, expected);
    }

    #[rstest]
    #[case (
        Piece {
            kind: Kind::I,
            position: Offset::zero(),
            rotation: Rotation::N,
        },
        super::RotateKind::Clockwise,
        Piece {
            kind: Kind::I,
            position: Offset::zero(),
            rotation: Rotation::E,
        }
    )]
    #[case (
        Piece {
            kind: Kind::I,
            position: Offset::zero(),
            rotation: Rotation::N,
        },
        super::RotateKind::CounterClockwise,
        Piece {
            kind: Kind::I,
            position: Offset::zero(),
            rotation: Rotation::W,
        }
    )]
    #[case (
        Piece {
            kind: Kind::I,
            position: Offset::zero(),
            rotation: Rotation::S,
        },
        super::RotateKind::CounterClockwise,
        Piece {
            kind: Kind::I,
            position: Offset::zero(),
            rotation: Rotation::E,
        }
    )]
    fn test_rotated_by(
        #[case] piece: Piece,
        #[case] kind: super::RotateKind,
        #[case] expected: Piece,
    ) {
        let rotated = piece.rotated_by(kind);
        assert_eq!(rotated, expected);
    }
}
