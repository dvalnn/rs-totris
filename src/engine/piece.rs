use cgmath::Vector2;

type Coordinate = Vector2<usize>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Kind {
    pub const ALL: [Kind; 7] = [
        Kind::I,
        Kind::O,
        Kind::T,
        Kind::S,
        Kind::Z,
        Kind::J,
        Kind::L,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: Kind,
    pub position: Coordinate,
    pub rotation: Rotation,
}

impl Piece {
    pub fn new(kind: Kind) -> Self {
        Piece {
            kind,
            position: Coordinate::new(0, 0),
            rotation: Rotation::N,
        }
    }
}
