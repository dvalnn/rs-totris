use std::{collections::HashMap, sync::Mutex};

use cgmath::Zero;
use once_cell::sync::Lazy;

use super::{piece::Rotation, Offset, PieceKind, RotateKind};

pub trait Kick {
    fn offset(&self) -> Offset;
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct StdMapKey(Rotation, u8);

//NOTE: SRS+ Clockwise Kick Table for J, L, S, T, Z Pieces
//      Counter-clockwise kick are the same, but with the opposite sign
static SRS_STD_KICKS: Lazy<Mutex<HashMap<StdMapKey, Offset>>> =
    Lazy::new(|| {
        use Rotation::*;

        let mut m = HashMap::new();
        //N -> E | 0 -> 1 = -(E -> N | 1 -> 0)
        m.insert(StdMapKey(N, 0), Offset::new(-1, 0));
        m.insert(StdMapKey(N, 1), Offset::new(-1, 1));
        m.insert(StdMapKey(N, 2), Offset::new(0, -2));
        m.insert(StdMapKey(N, 3), Offset::new(-1, -2));

        //E -> S | 1 -> 2
        m.insert(StdMapKey(E, 0), Offset::new(1, 0));
        m.insert(StdMapKey(E, 1), Offset::new(1, -1));
        m.insert(StdMapKey(E, 2), Offset::new(0, 2));
        m.insert(StdMapKey(E, 3), Offset::new(1, 2));

        //S -> W | 2 -> 3
        m.insert(StdMapKey(S, 0), Offset::new(1, 0));
        m.insert(StdMapKey(S, 1), Offset::new(1, 1));
        m.insert(StdMapKey(S, 2), Offset::new(0, -2));
        m.insert(StdMapKey(S, 3), Offset::new(1, -2));

        //W -> N | 3 -> 0
        m.insert(StdMapKey(W, 0), Offset::new(-1, 0));
        m.insert(StdMapKey(W, 1), Offset::new(-1, -1));
        m.insert(StdMapKey(W, 2), Offset::new(0, 2));
        m.insert(StdMapKey(W, 3), Offset::new(-1, 2));

        Mutex::new(m)
    });

#[derive(Clone, Hash, PartialEq, Eq)]
struct IMapKey(Rotation, RotateKind, u8);

//NOTE: SRS+ Kick Table for I piece with simetric I piece rotation
static SRS_I_KICKS: Lazy<Mutex<HashMap<IMapKey, Offset>>> = Lazy::new(|| {
    use RotateKind::*;
    use Rotation::*;

    let mut m = HashMap::new();
    //N -> E | 0 -> 1
    m.insert(IMapKey(N, Clockwise, 0), Offset::new(-2, 0));
    m.insert(IMapKey(N, Clockwise, 1), Offset::new(1, 0));
    m.insert(IMapKey(N, Clockwise, 2), Offset::new(1, 2));
    m.insert(IMapKey(N, Clockwise, 3), Offset::new(-2, -1));

    //N -> W | 0 -> 3
    m.insert(IMapKey(N, CounterClockwise, 0), Offset::new(2, 0));
    m.insert(IMapKey(N, CounterClockwise, 1), Offset::new(-1, 0));
    m.insert(IMapKey(N, CounterClockwise, 2), Offset::new(-1, 2));
    m.insert(IMapKey(N, CounterClockwise, 3), Offset::new(2, -1));

    //S -> W | 2 -> 3
    m.insert(IMapKey(S, Clockwise, 0), Offset::new(2, 0));
    m.insert(IMapKey(S, Clockwise, 1), Offset::new(-1, 0));
    m.insert(IMapKey(S, Clockwise, 2), Offset::new(2, 1));
    m.insert(IMapKey(S, Clockwise, 3), Offset::new(-1, -1));

    //S -> E | 2 -> 1
    m.insert(IMapKey(S, CounterClockwise, 0), Offset::new(-2, 0));
    m.insert(IMapKey(S, CounterClockwise, 1), Offset::new(1, 0));
    m.insert(IMapKey(S, CounterClockwise, 2), Offset::new(-2, 1));
    m.insert(IMapKey(S, CounterClockwise, 3), Offset::new(1, -1));

    //E -> S | 1 -> S
    m.insert(IMapKey(E, Clockwise, 0), Offset::new(-1, 0));
    m.insert(IMapKey(E, Clockwise, 1), Offset::new(2, 0));
    m.insert(IMapKey(E, Clockwise, 2), Offset::new(-1, 2));
    m.insert(IMapKey(E, Clockwise, 3), Offset::new(2, -1));

    //E -> N | 1 -> 0
    m.insert(IMapKey(E, CounterClockwise, 0), Offset::new(2, 0));
    m.insert(IMapKey(E, CounterClockwise, 1), Offset::new(-1, 0));
    m.insert(IMapKey(E, CounterClockwise, 2), Offset::new(2, 1));
    m.insert(IMapKey(E, CounterClockwise, 3), Offset::new(-1, -2));

    //W -> N | 3 -> 0
    m.insert(IMapKey(W, Clockwise, 0), Offset::new(-2, 0));
    m.insert(IMapKey(W, Clockwise, 1), Offset::new(1, 0));
    m.insert(IMapKey(W, Clockwise, 2), Offset::new(-2, 1));
    m.insert(IMapKey(W, Clockwise, 3), Offset::new(1, -2));

    //W -> S | 3 -> 2
    m.insert(IMapKey(W, CounterClockwise, 0), Offset::new(1, 0));
    m.insert(IMapKey(W, CounterClockwise, 1), Offset::new(-2, 0));
    m.insert(IMapKey(W, CounterClockwise, 2), Offset::new(1, 2));
    m.insert(IMapKey(W, CounterClockwise, 3), Offset::new(-2, -1));

    Mutex::new(m)
});

pub struct SrsPlus {
    piece_kind: PieceKind,
    rotation: Rotation,
    rotate_kind: RotateKind,
}

impl SrsPlus {
    const KICK_COUNT: u8 = 4;

    pub fn new(
        piece_kind: PieceKind,
        rotation: Rotation,
        rotate_kind: RotateKind,
    ) -> Self {
        Self {
            piece_kind,
            rotation,
            rotate_kind,
        }
    }

    pub fn get_kicks(&self) -> Vec<Offset> {
        match self.piece_kind {
            PieceKind::O => vec![Offset::zero()],
            PieceKind::I => self.get_i_kicks(),
            _ => self.get_std_kicks(),
        }
    }

    fn get_std_kicks(&self) -> Vec<Offset> {
        let mut modifier = 1;
        let mut rotation = self.rotation;

        if let RotateKind::CounterClockwise = self.rotate_kind {
            modifier = -1;
            rotation = rotation + self.rotate_kind;
        }

        let mut kicks = Vec::with_capacity(Self::KICK_COUNT as usize);
        let map = SRS_STD_KICKS.lock().unwrap();
        for i in 0..Self::KICK_COUNT {
            let key = StdMapKey(rotation, i);
            let kick = map.get(&key).unwrap();
            kicks.push(*kick * modifier);
        }
        kicks
    }

    fn get_i_kicks(&self) -> Vec<Offset> {
        let mut kicks = Vec::with_capacity(Self::KICK_COUNT as usize);
        let map = SRS_I_KICKS.lock().unwrap();
        for i in 0..Self::KICK_COUNT {
            let key = IMapKey(self.rotation, self.rotate_kind, i);
            let kick = map.get(&key).unwrap();
            kicks.push(*kick);
        }
        kicks
    }
}

#[cfg(test)]
mod test {
    use super::{PieceKind::*, RotateKind::*, Rotation::*, *};
    use rstest::rstest;

    #[rstest]
    #[case(SrsPlus::new(J, N, Clockwise),
            vec![
                Offset::new(-1, 0),
                Offset::new(-1, 1),
                Offset::new(0, -2),
                Offset::new(-1, -2),
            ])]
    #[case(SrsPlus::new(J, E, CounterClockwise),
            vec![
                Offset::new(1, 0),
                Offset::new(1, -1),
                Offset::new(0, 2),
                Offset::new(1, 2),
            ])]
    fn test_std_kicks(
        #[case] srs_plus: SrsPlus,
        #[case] expected: Vec<Offset>,
    ) {
        assert_eq!(srs_plus.get_kicks(), expected);
    }
}
