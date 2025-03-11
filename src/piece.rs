use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

impl Piece {
    pub fn from_initial_position(square_number: usize) -> Option<Piece> {
        match square_number {
            0 | 7 => Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::Black,
            }),
            1 | 6 => Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::Black,
            }),
            2 | 5 => Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::Black,
            }),
            3 => Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::Black,
            }),
            4 => Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::Black,
            }),
            8..=15 => Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::Black,
            }),
            48..=55 => Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            }),
            56 | 63 => Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::White,
            }),
            57 | 62 => Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::White,
            }),
            58 | 61 => Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::White,
            }),
            59 => Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::White,
            }),
            60 => Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::White,
            }),
            _ => None,
        }
    }

    pub fn to_fen_char(&self) -> char {
        match self.kind {
            PieceKind::Pawn => match self.color {
                PieceColor::White => 'P',
                PieceColor::Black => 'p',
            },
            PieceKind::Rook => match self.color {
                PieceColor::White => 'R',
                PieceColor::Black => 'r',
            },
            PieceKind::Knight => match self.color {
                PieceColor::White => 'N',
                PieceColor::Black => 'n',
            },
            PieceKind::Bishop => match self.color {
                PieceColor::White => 'B',
                PieceColor::Black => 'b',
            },
            PieceKind::Queen => match self.color {
                PieceColor::White => 'Q',
                PieceColor::Black => 'q',
            },
            PieceKind::King => match self.color {
                PieceColor::White => 'K',
                PieceColor::Black => 'k',
            },
        }
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceKind {
    pub fn promoteable() -> std::slice::Iter<'static, PieceKind> {
        [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
        ]
        .iter()
    }
}

#[derive(Eq, Hash, Copy, Clone, PartialEq, Debug)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn opposite(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
    pub fn homerow(&self) -> isize {
        match self {
            PieceColor::White => 7,
            PieceColor::Black => 0,
        }
    }
    pub fn pawn_orientation(&self) -> isize {
        match self {
            PieceColor::White => -1,
            PieceColor::Black => 1,
        }
    }

    pub fn both() -> std::array::IntoIter<PieceColor, 2> {
        [PieceColor::White, PieceColor::Black].into_iter()
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceColor::Black => write!(f, "Black"),
            PieceColor::White => write!(f, "White"),
        }
    }
}
