use crate::Coords;
use crate::Piece;
use crate::PieceColor;
use crate::PieceKind;

#[derive(Clone, PartialEq)]
pub struct Board {
    black_rooks: u64,
    black_knights: u64,
    black_bishops: u64,
    black_queens: u64,
    black_kings: u64,
    black_pawns: u64,
    white_rooks: u64,
    white_knights: u64,
    white_bishops: u64,
    white_queens: u64,
    white_kings: u64,
    white_pawns: u64,
}

fn bit_at_nth(number: u64, n: usize) -> bool {
    ((1 << n) & number) > 0
}

fn clear_nth(number: &mut u64, n: usize) {
    *number &= !(1 << n)
}

impl Board {
    pub fn move_piece(&mut self, origin: Coords, dest: Coords) {
        if let Some(origin_piece) = self.take_piece_at(origin) {
            self.put_piece_at(origin_piece, dest);
        }
    }
    pub fn piece_at(&self, loc: &Coords) -> Option<Piece> {
        let square_number = loc.to_square_number();
        if self.black_rook_at(square_number) {
            Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::Black,
            })
        } else if self.black_knight_at(square_number) {
            Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::Black,
            })
        } else if self.black_bishop_at(square_number) {
            Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::Black,
            })
        } else if self.black_queen_at(square_number) {
            Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::Black,
            })
        } else if self.black_king_at(square_number) {
            Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::Black,
            })
        } else if self.black_pawn_at(square_number) {
            Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::Black,
            })
        } else if self.white_rook_at(square_number) {
            Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::White,
            })
        } else if self.white_knight_at(square_number) {
            Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::White,
            })
        } else if self.white_bishop_at(square_number) {
            Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::White,
            })
        } else if self.white_queen_at(square_number) {
            Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::White,
            })
        } else if self.white_king_at(square_number) {
            Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::White,
            })
        } else if self.white_pawn_at(square_number) {
            Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            })
        } else {
            None
        }
    }

    pub fn pawn_at(&self, loc: &Coords) -> bool {
        let square_number = loc.to_square_number();
        self.black_pawn_at(square_number) || self.white_pawn_at(square_number)
    }

    pub fn king_at(&self, loc: &Coords) -> bool {
        let square_number = loc.to_square_number();
        self.white_king_at(square_number) || self.black_king_at(square_number)
    }

    pub fn take_piece_at(&mut self, loc: Coords) -> Option<Piece> {
        let piece = self.piece_at(&loc);
        self.clear_square(&loc);
        piece
    }

    fn clear_square(&mut self, loc: &Coords) {
        let square_number = loc.to_square_number();

        clear_nth(&mut self.black_rooks, square_number);
        clear_nth(&mut self.black_knights, square_number);
        clear_nth(&mut self.black_bishops, square_number);
        clear_nth(&mut self.black_queens, square_number);
        clear_nth(&mut self.black_kings, square_number);
        clear_nth(&mut self.black_pawns, square_number);

        clear_nth(&mut self.white_rooks, square_number);
        clear_nth(&mut self.white_knights, square_number);
        clear_nth(&mut self.white_bishops, square_number);
        clear_nth(&mut self.white_queens, square_number);
        clear_nth(&mut self.white_kings, square_number);
        clear_nth(&mut self.white_pawns, square_number);
    }

    pub fn put_piece_at(&mut self, piece: Piece, loc: Coords) {
        let square_number = loc.to_square_number() as u32;
        self.clear_square(&loc);
        match piece.color {
            PieceColor::Black => match piece.kind {
                PieceKind::Rook => self.black_rooks |= 2_u64.pow(square_number),
                PieceKind::Knight => self.black_knights |= 2_u64.pow(square_number),
                PieceKind::Bishop => self.black_bishops |= 2_u64.pow(square_number),
                PieceKind::Queen => self.black_queens |= 2_u64.pow(square_number),
                PieceKind::King => self.black_kings |= 2_u64.pow(square_number),
                PieceKind::Pawn => self.black_pawns |= 2_u64.pow(square_number),
            },
            PieceColor::White => match piece.kind {
                PieceKind::Rook => self.white_rooks |= 2_u64.pow(square_number),
                PieceKind::Knight => self.white_knights |= 2_u64.pow(square_number),
                PieceKind::Bishop => self.white_bishops |= 2_u64.pow(square_number),
                PieceKind::Queen => self.white_queens |= 2_u64.pow(square_number),
                PieceKind::King => self.white_kings |= 2_u64.pow(square_number),
                PieceKind::Pawn => self.white_pawns |= 2_u64.pow(square_number),
            },
        }
    }

    fn black_rook_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.black_rooks, square_number)
    }
    fn black_knight_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.black_knights, square_number)
    }
    fn black_bishop_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.black_bishops, square_number)
    }
    fn black_queen_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.black_queens, square_number)
    }
    fn black_king_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.black_kings, square_number)
    }
    fn black_pawn_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.black_pawns, square_number)
    }

    fn white_rook_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.white_rooks, square_number)
    }
    fn white_knight_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.white_knights, square_number)
    }
    fn white_bishop_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.white_bishops, square_number)
    }
    fn white_queen_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.white_queens, square_number)
    }
    fn white_king_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.white_kings, square_number)
    }
    fn white_pawn_at(&self, square_number: usize) -> bool {
        bit_at_nth(self.white_pawns, square_number)
    }

    pub fn initial() -> Board {
        Board {
            black_rooks: (2_u64.pow(0)) + (2_u64.pow(7)),
            black_knights: (2_u64.pow(1)) + (2_u64.pow(6)),
            black_bishops: (2_u64.pow(2)) + (2_u64.pow(5)),
            black_queens: 2_u64.pow(3),
            black_kings: 2_u64.pow(4),
            black_pawns: { (8..=15).map(|number| 2_u64.pow(number)).sum() },
            white_rooks: (2_u64.pow(56)) + (2_u64.pow(63)),
            white_knights: (2_u64.pow(57)) + (2_u64.pow(62)),
            white_bishops: (2_u64.pow(58)) + (2_u64.pow(61)),
            white_queens: (2_u64.pow(59)),
            white_kings: (2_u64.pow(60)),
            white_pawns: { (48..=55).map(|number| 2_u64.pow(number)).sum() },
        }
    }

    pub fn empty() -> Board {
        Board {
            black_rooks: 0,
            black_knights: 0,
            black_bishops: 0,
            black_queens: 0,
            black_kings: 0,
            black_pawns: 0,
            white_rooks: 0,
            white_knights: 0,
            white_bishops: 0,
            white_queens: 0,
            white_kings: 0,
            white_pawns: 0,
        }
    }

    pub fn from_fen(fen_board: &str) -> Board {
        let mut board = Board::empty();
        let mut i = 0_usize;
        fen_board.chars().for_each(|character| match character {
            '1'..='8' => {
                i += character
                    .to_digit(10)
                    .expect("should be integer 1 through 8") as usize;
            }
            '/' => (),
            'r' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Rook,
                        color: PieceColor::Black,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'n' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Knight,
                        color: PieceColor::Black,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'b' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Bishop,
                        color: PieceColor::Black,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'q' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Queen,
                        color: PieceColor::Black,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'k' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::King,
                        color: PieceColor::Black,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'p' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Pawn,
                        color: PieceColor::Black,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'R' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Rook,
                        color: PieceColor::White,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'N' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Knight,
                        color: PieceColor::White,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'B' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Bishop,
                        color: PieceColor::White,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'Q' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Queen,
                        color: PieceColor::White,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'K' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::King,
                        color: PieceColor::White,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            'P' => {
                board.put_piece_at(
                    Piece {
                        kind: PieceKind::Pawn,
                        color: PieceColor::White,
                    },
                    Coords::from_square_number(i),
                );
                i += 1
            }
            _ => panic!("{} is not a valid board character in FEN", character),
        });
        assert_eq!(i, 64);

        board
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for i in 0..64 {
            match self.piece_at(&Coords::from_square_number(i)) {
                None => match fen.chars().last() {
                    None => fen.push('1'),
                    Some(character) => match character {
                        '/' | 'p' | 'P' | 'n' | 'N' | 'r' | 'R' | 'b' | 'B' | 'q' | 'Q' | 'k'
                        | 'K' => fen.push('1'),
                        '1'..='7' => {
                            fen.pop();
                            fen.push_str(
                                &(character.to_digit(10).expect("matched digits 1 through 7") + 1)
                                    .to_string(),
                            );
                        }
                        _ => panic!("more than 8 empty squares in rank!"),
                    },
                },
                Some(piece) => fen.push(piece.to_fen_char()),
            };
            if (i + 1) % 8 == 0 && i > 0 {
                fen.push('/');
            }
        }
        fen.pop();
        fen
    }
}

#[cfg(test)]
mod tests {
    use crate::all_squares;

    use super::*;

    #[test]
    fn empty_board_is_empty() {
        let board = Board::empty();
        all_squares()
            .iter()
            .for_each(|square| assert!(board.piece_at(square).is_none()));
    }

    #[test]
    fn piece_is_where_i_put_it() {
        let mut board = Board::empty();
        board.put_piece_at(
            Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            },
            Coords::from_algebraic("e4"),
        );
        assert!(
            board
                .piece_at(&Coords::from_algebraic("e4"))
                .is_some_and(
                    |piece| piece.kind == PieceKind::Pawn && piece.color == PieceColor::White
                )
        );
    }

    #[test]
    fn taking_a_piece_removes_the_piece() {
        let mut board = Board::empty();

        board.put_piece_at(
            Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            },
            Coords::from_algebraic("e4"),
        );

        board.take_piece_at(Coords::from_algebraic("e4"));

        assert!(board.piece_at(&Coords::from_algebraic("e4")).is_none());
    }

    #[test]
    fn taking_a_piece_returns_the_piece() {
        let mut board = Board::empty();

        board.put_piece_at(
            Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            },
            Coords::from_algebraic("e4"),
        );

        assert!(
            board
                .take_piece_at(Coords::from_algebraic("e4"))
                .is_some_and(
                    |piece| piece.kind == PieceKind::Pawn && piece.color == PieceColor::White
                )
        );
    }

    #[test]
    fn piece_is_where_i_moved_it_not_where_i_moved_it_from() {
        let mut board = Board::empty();
        board.put_piece_at(
            Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            },
            Coords::from_algebraic("e4"),
        );
        board.move_piece(Coords::from_algebraic("e4"), Coords::from_algebraic("a8"));
        assert!(board.piece_at(&Coords::from_algebraic("e4")).is_none());
        assert!(
            board
                .piece_at(&Coords::from_algebraic("a8"))
                .is_some_and(
                    |piece| piece.kind == PieceKind::Pawn && piece.color == PieceColor::White
                )
        );
    }

    #[test]
    fn first_bit_in_1_is_set() {
        assert!(bit_at_nth(1, 0))
    }

    #[test]
    fn fourth_bit_in_1_is_not_set() {
        assert!(!bit_at_nth(1, 3))
    }

    #[test]
    fn second_bit_in_3_is_set() {
        assert!(bit_at_nth(3, 1))
    }

    #[test]
    fn third_bit_in_3_is_not_set() {
        assert!(!bit_at_nth(3, 2))
    }

    #[test]
    fn fourth_bit_in_31_is_set() {
        assert!(bit_at_nth(31, 3))
    }

    #[test]
    fn fourth_bit_in_32_is_not_set() {
        assert!(!bit_at_nth(32, 3))
    }

    #[test]
    fn clear_sixth_32_is_0() {
        let mut number = 32;
        clear_nth(&mut number, 5);
        assert_eq!(number, 0);
    }

    #[test]
    fn clear_first_1_is_0() {
        let mut number = 1;
        clear_nth(&mut number, 0);
        assert_eq!(number, 0);
    }

    #[test]
    fn clear_seventh_0_is_0() {
        let mut number = 0;
        clear_nth(&mut number, 6);
        assert_eq!(number, 0);
    }

    #[test]
    fn black_rook_in_a8_in_initial_position() {
        assert!(Board::initial().black_rook_at(Coords::from_algebraic("a8").to_square_number()))
    }

    #[test]
    fn white_king_in_e1_after_putting_white_king_in_e1() {
        let mut board = Board::empty();
        board.put_piece_at(
            Piece {
                kind: PieceKind::King,
                color: PieceColor::White,
            },
            Coords::from_algebraic("e1"),
        );
        assert!(board.white_king_at(Coords::from_algebraic("e1").to_square_number()));
    }
}
