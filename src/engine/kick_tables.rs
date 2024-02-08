use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use super::{piece::Rotation, Offset, PieceKind, RotateKind};

pub trait Kick {
    fn offset(&self) -> Offset;
}

pub enum SrsPlus {
    Kick1,
    Kick2,
    Kick3,
    Kick4,
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum KickType {
    Standard,
    IPiece,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct SrsKickKey(KickType, Rotation, u8);

static KICKS: Lazy<Mutex<HashMap<SrsKickKey, (isize, isize)>>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert(SrsKickKey(KickType::Standard, Rotation::N, 1), (-1, 0));
        Mutex::new(m)
    });

impl SrsPlus {
    fn into_array(
        piece: PieceKind,
        rotation: Rotation,
        rotate_kind: RotateKind,
    ) -> Vec<Self> {
        vec![Self::Kick1, Self::Kick2, Self::Kick3, Self::Kick4]
    }

    fn get_test_index(rotation: Rotation, rotate_kind: RotateKind) -> u8 {
        match (rotation, rotate_kind) {
            // N -> E | 0 -> 1
            (Rotation::N, RotateKind::Clockwise) => todo!(),
            (Rotation::N, RotateKind::CounterClockwise) => todo!(),

            // N -> E | 0 -> 1
            (Rotation::E, RotateKind::Clockwise) => todo!(),
            (Rotation::E, RotateKind::CounterClockwise) => todo!(),

            // N -> E | 0 -> 1
            (Rotation::S, RotateKind::Clockwise) => todo!(),
            (Rotation::S, RotateKind::CounterClockwise) => todo!(),

            // N -> E | 0 -> 1
            (Rotation::W, RotateKind::Clockwise) => todo!(),
            (Rotation::W, RotateKind::CounterClockwise) => todo!(),
        }
    }

    fn kick1(piece: PieceKind, rotation: Rotation, rotate_kind: RotateKind) {}
}

impl Kick for SrsPlus {
    fn offset(&self) -> Offset {
        match self {
            SrsPlus::Kick1 => todo!(),
            SrsPlus::Kick2 => todo!(),
            SrsPlus::Kick3 => todo!(),
            SrsPlus::Kick4 => todo!(),
        }
    }
}
