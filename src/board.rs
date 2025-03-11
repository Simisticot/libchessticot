use crate::Coords;
use crate::Piece;
use crate::PieceColor;
use crate::PieceKind;

#[derive(Clone, PartialEq)]
pub struct Board {
    content: [Option<Piece>; 64],
}

impl Board {
    pub fn move_piece(&mut self, origin: Coords, dest: Coords) {
        if let Some(origin_piece) = self.take_piece_at(origin) {
            self.put_piece_at(origin_piece, dest);
        }
    }
    pub fn piece_at(&self, loc: &Coords) -> Option<Piece> {
        self.content[loc.to_square_number()]
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
        self.content[loc.to_square_number()].take()
    }
    pub fn put_piece_at(&mut self, piece: Piece, loc: Coords) {
        self.content[loc.to_square_number()] = Some(piece);
    }

    pub fn initial() -> Board {
        let mut content: [Option<Piece>; 64] = [None; 64];
        for (i, square) in content.iter_mut().enumerate() {
            *square = Piece::from_initial_position(i);
        }
        Board { content }
    }

    pub fn empty() -> Board {
        Board {
            content: [None; 64],
        }
    }

    pub fn from_fen(fen_board: &str) -> Board {
        let mut content = [None; 64];
        let mut index = 0;
        fen_board.chars().for_each(|character| match character {
            '1'..='8' => {
                for _ in 0..character.to_digit(10).expect("matched digits 1 through 8") {
                    index += 1;
                }
            }
            '/' => (),
            'r' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Rook,
                    color: PieceColor::Black,
                });
                index += 1;
            }
            'n' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Knight,
                    color: PieceColor::Black,
                });
                index += 1;
            }
            'b' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Bishop,
                    color: PieceColor::Black,
                });
                index += 1;
            }
            'q' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Queen,
                    color: PieceColor::Black,
                });
                index += 1;
            }
            'k' => {
                content[index] = Some(Piece {
                    kind: PieceKind::King,
                    color: PieceColor::Black,
                });
                index += 1;
            }
            'p' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Pawn,
                    color: PieceColor::Black,
                });
                index += 1;
            }
            'R' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Rook,
                    color: PieceColor::White,
                });
                index += 1;
            }
            'N' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Knight,
                    color: PieceColor::White,
                });
                index += 1;
            }
            'B' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Bishop,
                    color: PieceColor::White,
                });
                index += 1;
            }
            'Q' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Queen,
                    color: PieceColor::White,
                });
                index += 1;
            }
            'K' => {
                content[index] = Some(Piece {
                    kind: PieceKind::King,
                    color: PieceColor::White,
                });
                index += 1;
            }
            'P' => {
                content[index] = Some(Piece {
                    kind: PieceKind::Pawn,
                    color: PieceColor::White,
                });
                index += 1;
            }
            _ => panic!("{} is not a valid board character in FEN", character),
        });

        assert_eq!(content.len(), 64);
        Board { content }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        self.content.chunks(8).for_each(|rank| {
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
        });
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
}
