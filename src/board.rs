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
    white_bishop: u64,
    white_queens: u64,
    white_kings: u64,
    white_pawns: u64,
}

fn bit_at_nth(number: u64, n: usize) -> bool {
    ((1 << n) & number) > 0
}

impl Board {
    pub fn move_piece(&mut self, origin: Coords, dest: Coords) {
        if let Some(origin_piece) = self.take_piece_at(origin) {
            self.put_piece_at(origin_piece, dest);
        }
    }
    pub fn piece_at(&self, loc: &Coords) -> Option<Piece> {
        None
    }

    pub fn pawn_at(&self, loc: &Coords) -> bool {
        self.piece_at(loc)
            .is_some_and(|piece| piece.kind == PieceKind::Pawn)
    }

    pub fn king_at(&self, loc: &Coords) -> bool {
        self.piece_at(loc)
            .is_some_and(|piece| piece.kind == PieceKind::King)
    }

    pub fn take_piece_at(&mut self, loc: Coords) -> Option<Piece> {
        None
    }
    pub fn put_piece_at(&mut self, piece: Piece, loc: Coords) {
        ()
    }

    fn black_rook_at(&self, square: Coords) -> bool {
        bit_at_nth(self.black_rooks, square.to_square_number())
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
            white_bishop: (2_u64.pow(58)) + (2_u64.pow(61)),
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
            white_bishop: 0,
            white_queens: 0,
            white_kings: 0,
            white_pawns: 0,
        }
    }

    pub fn from_fen(fen_board: &str) -> Board {
        let mut content = vec![];
        fen_board.chars().for_each(|character| match character {
            '1'..='8' => {
                for _ in 0..character.to_digit(10).expect("matched digits 1 through 8") {
                    content.push(None);
                }
            }
            '/' => (),
            'r' => content.push(Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::Black,
            })),
            'n' => content.push(Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::Black,
            })),
            'b' => content.push(Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::Black,
            })),
            'q' => content.push(Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::Black,
            })),
            'k' => content.push(Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::Black,
            })),
            'p' => content.push(Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::Black,
            })),
            'R' => content.push(Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::White,
            })),
            'N' => content.push(Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::White,
            })),
            'B' => content.push(Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::White,
            })),
            'Q' => content.push(Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::White,
            })),
            'K' => content.push(Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::White,
            })),
            'P' => content.push(Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            })),
            _ => panic!("{} is not a valid board character in FEN", character),
        });

        assert_eq!(content.len(), 64);
        Board::empty()
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        /* self.content.chunks(8).for_each(|rank| {
            rank.iter()
                .for_each(|square_contents| match square_contents {
                    None => match fen.chars().last() {
                        None => fen.push('1'),
                        Some(character) => match character {
                            '/' | 'p' | 'P' | 'n' | 'N' | 'r' | 'R' | 'b' | 'B' | 'q' | 'Q'
                            | 'k' | 'K' => fen.push('1'),
                            '1'..='7' => {
                                fen.pop();
                                fen.push_str(
                                    &(character.to_digit(10).expect("matched digits 1 through 7")
                                        + 1)
                                    .to_string(),
                                );
                            }
                            _ => panic!("more than 8 empty squares in rank! {:?}", rank),
                        },
                    },
                    Some(piece) => fen.push(piece.to_fen_char()),
                });
            fen.push('/');
        }); */
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
    fn black_rook_in_a8_in_initial_position() {
        assert!(Board::initial().black_rook_at(Coords::from_algebraic("a8")))
    }
}
