use crate::bitboard::BitBoard;
use crate::square::Square;
pub const KNIGHT_ATTACKS: [BitBoard; 64] = all_knight_moves();
pub const fn all_knight_moves() -> [BitBoard; 64] {
    let mut moves = [BitBoard::EMPTY; 64];
    let mut square = 0;
    while square < 64 {
        moves[square] = knight_from(Square::new(square as u8));
        square += 1;
    }
    moves
}

pub const fn knight_from(square: Square) -> BitBoard {
    BitBoard(
        ((1 << square.index()) << 17)
            & if square.rank() < 6 && square.file() < 7 {
                !0
            } else {
                0
            }
            | (1 << square.index()) << 15
                & if square.rank() < 6 && square.file() > 0 {
                    !0
                } else {
                    0
                }
            | (1 << square.index()) << 10
                & if square.rank() < 7 && square.file() < 6 {
                    !0
                } else {
                    0
                }
            | (1 << square.index()) << 6
                & if square.rank() < 7 && square.file() > 1 {
                    !0
                } else {
                    0
                }
            | (1 << square.index()) >> 17
                & if square.rank() > 1 && square.file() > 0 {
                    !0
                } else {
                    0
                }
            | (1 << square.index()) >> 15
                & if square.rank() > 1 && square.file() < 7 {
                    !0
                } else {
                    0
                }
            | (1 << square.index()) >> 10
                & if square.rank() > 0 && square.file() > 1 {
                    !0
                } else {
                    0
                }
            | (1 << square.index()) >> 6
                & if square.rank() > 0 && square.file() < 6 {
                    !0
                } else {
                    0
                },
    )
}
