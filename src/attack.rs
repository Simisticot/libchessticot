use crate::bitboard::BitBoard;
use crate::square::Square;

pub const ALL_DIRECTIONS: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
pub const CARDINALS: [usize; 4] = [0, 1, 2, 3];
pub const INTERCARDINALS: [usize; 4] = [4, 5, 6, 7];

pub const ALL_RAYS: [[BitBoard; 64]; 8] = all_rays();

pub const RISING: [bool; 8] = [true, true, false, false, true, false, false, true];

pub const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // up
    (1, 0),   // left
    (0, -1),  // down
    (-1, 0),  // right
    (1, 1),   // up left
    (1, -1),  // down left
    (-1, -1), // down right
    (-1, 1),  // up right
];

pub const ALL_KING_ATTACKS: [BitBoard; 64] = all_circles();

const fn all_circles() -> [BitBoard; 64] {
    let mut circles = [0_u64; 64];
    let mut square = 0;
    let mut dir = 0;
    while square < 64 {
        while dir < 8 {
            let (dx, dy) = DIRECTIONS[dir];
            let (x, y) = (((square % 8) as isize + dx), ((square / 8) as isize + dy));
            if x < 8 && x >= 0 && y < 8 && y >= 0 {
                circles[square] |= 1_u64 << (x + y * 8);
            }
            dir += 1;
        }
        dir = 0;
        square += 1;
    }
    let mut boards = [BitBoard::EMPTY; 64];
    square = 0;
    while square < 64 {
        boards[square] = BitBoard(circles[square]);
        square += 1;
    }
    boards
}

const fn all_rays() -> [[BitBoard; 64]; 8] {
    let mut rays = [[0_u64; 64]; 8];
    let mut dir = 0;
    let mut square = 0;
    while dir < 8 {
        let (dx, dy) = DIRECTIONS[dir];
        while square < 64 {
            let (mut x, mut y) = (((square % 8) as isize + dx), ((square / 8) as isize + dy));
            while x < 8 && x >= 0 && y < 8 && y >= 0 {
                rays[dir][square] |= 1_u64 << (x + y * 8);
                x += dx;
                y += dy;
            }
            square += 1;
        }
        square = 0;
        dir += 1;
    }

    let mut boards = [[BitBoard::EMPTY; 64]; 8];
    dir = 0;
    square = 0;
    while dir < 8 {
        while square < 64 {
            boards[dir][square] = BitBoard(rays[dir][square]);
            square += 1;
        }
        square = 0;
        dir += 1;
    }
    boards
}

pub const KNIGHT_ATTACKS: [BitBoard; 64] = all_knight_moves();

const fn all_knight_moves() -> [BitBoard; 64] {
    let mut moves = [BitBoard::EMPTY; 64];
    let mut square = 0;
    while square < 64 {
        moves[square] = knight_from(Square::new(square as u8));
        square += 1;
    }
    moves
}

const fn knight_from(square: Square) -> BitBoard {
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
