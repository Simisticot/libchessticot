use crate::Coords;
use crate::PieceKind;

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub enum ChessMove {
    RegularMove(Move),
    PawnSkip(Move),
    CastleLeft,
    CastleRight,
    EnPassant(Move, Coords),
    Promotion(Move, PieceKind),
}

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub struct Move {
    pub origin: Coords,
    pub destination: Coords,
}

impl Move {
    pub fn x_distance(&self) -> isize {
        self.origin.x - self.destination.x
    }

    pub fn y_abs_distance(&self) -> usize {
        self.origin.y.abs_diff(self.destination.y)
    }
    pub fn x_abs_distance(&self) -> usize {
        self.origin.x.abs_diff(self.destination.x)
    }
}
