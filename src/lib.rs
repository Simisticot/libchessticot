mod board_manip;
mod chess_move;
mod coords;
mod engine;
mod piece;
mod player;
mod position;

use core::panic;

pub use crate::board_manip::{move_piece, piece_at, put_piece_at, take_piece_at};
pub use crate::chess_move::{ChessMove, Move};
pub use crate::coords::{all_squares, cards, eight_degrees, inter_cards, Coords, Direction};
pub use crate::engine::{BasicEvaluationPlayer, BetterEvaluationPlayer, FirstMovePlayer};
#[cfg(feature = "rng")]
pub use crate::engine::{RandomCapturePrioPlayer, RandomPlayer};
pub use crate::piece::{Piece, PieceColor, PieceKind};
pub use crate::player::Player;
pub use crate::position::Position;

#[derive(Debug)]
pub struct Game {
    pub current_position: Position,
    pub checkmated: Option<PieceColor>,
    pub stalemate: bool,
}

impl Game {
    pub fn start() -> Game {
        let mut board = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                row.push(Piece::from_initial_position(j, i));
            }
            board.push(row);
        }
        Game {
            current_position: Position::initial(),
            checkmated: None,
            stalemate: false,
        }
    }

    pub fn empty() -> Game {
        Game {
            current_position: Position::empty_board(),
            checkmated: None,
            stalemate: false,
        }
    }
    pub fn make_move(&mut self, chess_move: &ChessMove) {
        if self.current_position.is_move_legal(chess_move) {
            self.current_position = self.current_position.after_move(chess_move);
            if self.current_position.is_checkmate() {
                self.checkmated = Some(self.current_position.to_move.clone());
            }
            self.stalemate = self.current_position.is_stalemate()
        }
    }

    pub fn from_starting_position(starting_position: Position) -> Game {
        let checkmated = starting_position.checkmated();
        let stalemate = starting_position.is_stalemate();
        Game {
            current_position: starting_position,
            checkmated,
            stalemate,
        }
    }
}

#[derive(Debug)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Stalemate,
    TimedOut,
}

pub fn play_engine_game(
    white_player: Box<dyn Player>,
    black_player: Box<dyn Player>,
) -> GameResult {
    let mut game = Game::start();
    let mut turn_counter = 0;

    while game.checkmated.is_none() && !game.current_position.is_stalemate() && turn_counter < 300 {
        let offered_move = match game.current_position.to_move {
            PieceColor::White => white_player.offer_move(&game.current_position),
            PieceColor::Black => black_player.offer_move(&game.current_position),
        };
        if !game.current_position.is_move_legal(&offered_move) {
            panic!("engine offered illegal move");
        } else {
            game.make_move(&offered_move);
            turn_counter += 1;
        }
    }
    if let Some(color) = game.checkmated {
        match color {
            PieceColor::White => GameResult::BlackWin,
            PieceColor::Black => GameResult::WhiteWin,
        }
    } else if game.current_position.is_stalemate() {
        GameResult::Stalemate
    } else {
        GameResult::TimedOut
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{collections::HashSet, hash::RandomState};

    use super::*;

    #[test]
    fn pawn_homerow() {
        let position = Position::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");
        let pawn_location = Coords { y: 6, x: 4 };
        assert_eq!(
            position.legal_moves_from_origin(&pawn_location),
            vec![
                ChessMove::RegularMove(Move {
                    origin: pawn_location,
                    destination: Coords { y: 5, x: 4 }
                }),
                ChessMove::PawnSkip(Move {
                    origin: pawn_location,
                    destination: Coords { y: 4, x: 4 }
                })
            ]
        )
    }

    #[test]
    fn pawn_not_homerow() {
        let position = Position::from_fen("8/8/8/8/8/4P3/8/8 w - - 0 1");
        let pawn_location = Coords { y: 5, x: 4 };
        assert_eq!(
            position.legal_moves_from_origin(&pawn_location),
            vec![ChessMove::RegularMove(Move {
                origin: pawn_location,
                destination: Coords { y: 4, x: 4 }
            })]
        )
    }

    #[test]
    fn pawn_not_homerow_with_capture() {
        let position = Position::from_fen("8/8/8/8/5p2/4P3/8/8 w - - 0 1");
        let pawn_location = Coords { y: 5, x: 4 };
        let opposing_pawn_location = Coords { y: 4, x: 5 };
        assert_eq!(
            position.legal_moves_from_origin(&pawn_location),
            vec![
                ChessMove::RegularMove(Move {
                    origin: pawn_location,
                    destination: Coords { y: 4, x: 4 }
                }),
                ChessMove::RegularMove(Move {
                    origin: pawn_location,
                    destination: opposing_pawn_location
                })
            ]
        )
    }

    #[test]
    fn pawn_not_homerow_blocked() {
        let position = Position::from_fen("8/8/8/8/4p3/4P3/8/8 w - - 0 1");
        let pawn_location = Coords { y: 5, x: 4 };
        assert_eq!(position.legal_moves_from_origin(&pawn_location), vec![])
    }

    #[test]
    fn pawn_edge_of_board_horizontal_blocked() {
        let position = Position::from_fen("8/8/7P/7P/8/8/8/8 w - - 0 1");
        let pawn_location = Coords { y: 3, x: 7 };
        assert_eq!(position.legal_moves_from_origin(&pawn_location), vec![])
    }

    #[test]
    fn pawn_not_homerow_with_capture_blocked() {
        let position = Position::from_fen("8/8/8/8/4pp2/4P3/8/8 w - - 0 1");
        let pawn_location = Coords { y: 5, x: 4 };
        let opposing_pawn_location = Coords { y: 4, x: 5 };
        assert_eq!(
            position.legal_moves_from_origin(&pawn_location),
            vec![ChessMove::RegularMove(Move {
                origin: pawn_location,
                destination: opposing_pawn_location
            })]
        )
    }

    #[test]
    fn pawn_homerow_blocked() {
        let position = Position::from_fen("8/8/8/8/8/4p3/4P3/8 w - - 0 1");
        let pawn_location = Coords { y: 6, x: 4 };
        assert_eq!(position.legal_moves_from_origin(&pawn_location), vec![])
    }

    #[test]
    fn pawn_homerow_second_square_blocked() {
        let position = Position::from_fen("8/8/8/8/4p3/8/4P3/8 w - - 0 1");
        let pawn_location = Coords { y: 6, x: 4 };
        assert_eq!(
            position.legal_moves_from_origin(&pawn_location),
            vec![ChessMove::RegularMove(Move {
                origin: pawn_location,
                destination: Coords { y: 5, x: 4 }
            })]
        )
    }

    #[test]
    fn pawn_homerow_with_capture_blocked() {
        let position = Position::from_fen("8/8/8/8/8/4pp2/4P3/8 w - - 0 1");
        let pawn_location = Coords { y: 6, x: 4 };
        let capture_location = Coords { y: 5, x: 5 };
        assert_eq!(
            position.legal_moves_from_origin(&pawn_location),
            vec![ChessMove::RegularMove(Move {
                origin: pawn_location,
                destination: capture_location
            })]
        )
    }

    #[test]
    fn rook_middle_board() {
        let mut position = Position::empty_board();
        position.board[4][4] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        });
        let rook_location = Coords { y: 4, x: 4 };
        let mut legal_moves = vec![];

        for j in 0..8 {
            if j != 4 {
                legal_moves.push(ChessMove::RegularMove(Move {
                    origin: rook_location,
                    destination: Coords { y: 4, x: j },
                }));
            }
        }
        for i in 0..8 {
            if i != 4 {
                legal_moves.push(ChessMove::RegularMove(Move {
                    origin: rook_location,
                    destination: Coords { x: 4, y: i },
                }));
            }
        }

        let legal_move_set: HashSet<ChessMove, RandomState> =
            HashSet::from_iter(legal_moves.iter().cloned());
        let found_moves: HashSet<ChessMove, RandomState> =
            HashSet::from_iter(position.legal_moves_from_origin(&rook_location));
        let diff: HashSet<&ChessMove, RandomState> =
            legal_move_set.symmetric_difference(&found_moves).collect();

        assert_eq!(diff, HashSet::new())
    }

    #[test]
    fn rook_middle_board_boxed_in_opposite_color() {
        let mut position = Position::empty_board();
        position.board[4][4] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        });
        position.board[5][4] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        });
        position.board[3][4] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        });
        position.board[4][5] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        });
        position.board[4][3] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        });
        let rook_location = Coords { y: 4, x: 4 };
        let up = Coords { y: 5, x: 4 };
        let down = Coords { y: 3, x: 4 };
        let left = Coords { y: 4, x: 3 };
        let right = Coords { y: 4, x: 5 };

        let legal_moves = vec![
            ChessMove::RegularMove(Move {
                origin: rook_location,
                destination: up,
            }),
            ChessMove::RegularMove(Move {
                origin: rook_location,
                destination: down,
            }),
            ChessMove::RegularMove(Move {
                origin: rook_location,
                destination: left,
            }),
            ChessMove::RegularMove(Move {
                origin: rook_location,
                destination: right,
            }),
        ];

        assert_eq!(
            position.legal_moves_from_origin(&rook_location),
            legal_moves
        );
    }

    #[test]
    fn rook_middle_board_boxed_in_own_color() {
        let mut position = Position::empty_board();
        position.board[4][4] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        });
        position.board[5][4] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        });
        position.board[3][4] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        });
        position.board[4][5] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        });
        position.board[4][3] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        });
        let rook_location = Coords { y: 4, x: 4 };

        let legal_moves = vec![];

        assert_eq!(
            position.legal_moves_from_origin(&rook_location),
            legal_moves
        );
    }

    #[test]
    fn knight_middle_board() {
        let mut position = Position::empty_board();
        position.board[3][3] = Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::White,
        });
        let knight_location = Coords { y: 3, x: 3 };

        let legal_moves: HashSet<ChessMove, RandomState> = HashSet::from_iter(
            vec![
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 5, x: 4 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 4, x: 5 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 5, x: 2 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 4, x: 1 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 1, x: 4 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 2, x: 5 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 1, x: 2 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 2, x: 1 },
                }),
            ]
            .iter()
            .cloned(),
        );

        let found_moves: HashSet<ChessMove, RandomState> = HashSet::from_iter(
            position
                .legal_moves_from_origin(&knight_location)
                .iter()
                .cloned(),
        );

        let diff: HashSet<&ChessMove, RandomState> =
            legal_moves.symmetric_difference(&found_moves).collect();

        assert_eq!(diff, HashSet::new())
    }

    #[test]
    fn knight_corner() {
        let mut position = Position::empty_board();
        position.board[0][0] = Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::White,
        });
        let knight_location = Coords { y: 0, x: 0 };

        let legal_moves: HashSet<ChessMove, RandomState> = HashSet::from_iter(
            vec![
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 2, x: 1 },
                }),
                ChessMove::RegularMove(Move {
                    origin: knight_location,
                    destination: Coords { y: 1, x: 2 },
                }),
            ]
            .iter()
            .cloned(),
        );

        let found_moves: HashSet<ChessMove, RandomState> = HashSet::from_iter(
            position
                .legal_moves_from_origin(&knight_location)
                .iter()
                .cloned(),
        );

        let diff: HashSet<&ChessMove, RandomState> =
            legal_moves.symmetric_difference(&found_moves).collect();

        assert_eq!(diff, HashSet::new())
    }

    #[test]
    fn knight_corner_blocked() {
        let mut position = Position::empty_board();
        position.board[0][0] = Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::White,
        });
        position.board[1][2] = Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::White,
        });
        position.board[2][1] = Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::White,
        });
        let knight_location = Coords { y: 0, x: 0 };

        assert_eq!(position.legal_moves_from_origin(&knight_location).len(), 0)
    }

    #[test]
    fn bishob_middle_board() {
        let mut position = Position::empty_board();
        position.board[3][3] = Some(Piece {
            kind: PieceKind::Bishop,
            color: PieceColor::White,
        });
        let bishop_location = Coords { y: 3, x: 3 };
        let mut legal_moves = vec![];

        for j in 0..8 {
            if j != 3 {
                legal_moves.push(ChessMove::RegularMove(Move {
                    origin: bishop_location,
                    destination: Coords { y: j, x: j },
                }));
            }
        }

        for i in 0..7 {
            if i != 3 {
                legal_moves.push(ChessMove::RegularMove(Move {
                    origin: bishop_location,
                    destination: Coords { y: 6 - i, x: i },
                }));
            }
        }

        let legal_move_set: HashSet<ChessMove, RandomState> =
            HashSet::from_iter(legal_moves.iter().cloned());
        let found_move_set: HashSet<ChessMove, RandomState> = HashSet::from_iter(
            position
                .legal_moves_from_origin(&bishop_location)
                .iter()
                .cloned(),
        );
        let difference_set: HashSet<&ChessMove, RandomState> = legal_move_set
            .symmetric_difference(&found_move_set)
            .collect();
        assert_eq!(difference_set, HashSet::new());
    }

    #[test]
    fn king_middle_board() {
        let mut position = Position::empty_board();
        position.board[3][3] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        });
        let king_location = Coords { y: 3, x: 3 };
        let legal_moves = HashSet::from([
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 3, x: 4 },
            }),
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 3, x: 2 },
            }),
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 2, x: 3 },
            }),
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 4, x: 3 },
            }),
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 4, x: 4 },
            }),
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 2, x: 2 },
            }),
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 4, x: 2 },
            }),
            ChessMove::RegularMove(Move {
                origin: king_location.clone(),
                destination: Coords { y: 2, x: 4 },
            }),
        ]);

        let found_moves: HashSet<ChessMove, RandomState> = HashSet::from_iter(
            position
                .legal_moves_from_origin(&king_location)
                .iter()
                .cloned(),
        );

        let diff: HashSet<&ChessMove, RandomState> =
            legal_moves.symmetric_difference(&found_moves).collect();

        assert_eq!(diff, HashSet::new())
    }

    #[test]
    fn cannot_move_out_of_turn() {
        let mut position = Position::empty_board();
        position.board[3][3] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::Black,
        });
        let king_location = Coords { y: 3, x: 3 };
        assert_eq!(position.legal_moves_from_origin(&king_location).len(), 0);
    }

    #[test]
    fn cannot_move_into_check() {
        let mut position = Position::empty_board();

        position.board[0][0] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        });
        position.board[2][2] = Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::Black,
        });
        let king_location = Coords { y: 0, x: 0 };
        assert!(!position.is_move_legal(&ChessMove::RegularMove(Move {
            origin: king_location,
            destination: Coords { y: 0, x: 1 },
        }),));
    }

    #[test]
    fn detects_checkmate() {
        let mut position = Position::empty_board();

        position.board[0][0] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        });
        position.board[1][1] = Some(Piece {
            kind: PieceKind::Queen,
            color: PieceColor::Black,
        });
        position.board[2][2] = Some(Piece {
            kind: PieceKind::Queen,
            color: PieceColor::Black,
        });
        assert!(position.is_checkmate());
    }

    #[test]
    fn can_castle_right() {
        let position = Position::from_fen("8/8/8/8/8/8/8/4K2R w KQ - 0 1");
        assert!(position
            .legal_moves_from_origin(&Coords { y: 7, x: 4 })
            .contains(&ChessMove::CastleRight));
        assert!(position.is_move_legal(&ChessMove::CastleRight,))
    }

    #[test]
    fn castle_right() {
        let position = Position::from_fen("8/8/8/8/8/8/8/4K2R w KQ - 0 1");

        let after_castle_right = position.after_move(&ChessMove::CastleRight);
        assert!(
            piece_at(&after_castle_right.board, &Coords { y: 7, x: 6 }).is_some_and(|piece| piece
                == Piece {
                    kind: PieceKind::King,
                    color: PieceColor::White
                })
        );
        assert!(
            piece_at(&after_castle_right.board, &Coords { y: 7, x: 5 }).is_some_and(|piece| piece
                == Piece {
                    kind: PieceKind::Rook,
                    color: PieceColor::White
                })
        );
    }

    #[test]
    fn make_move() {
        let mut game = Game::empty();

        game.current_position.board[0][0] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        });
        let king_location = Coords { x: 0, y: 0 };
        game.make_move(&ChessMove::RegularMove(Move {
            origin: king_location,
            destination: Coords { x: 0, y: 1 },
        }));
        assert!(piece_at(&game.current_position.board, &king_location).is_none());
        assert_eq!(
            piece_at(&game.current_position.board, &Coords { x: 0, y: 1 })
                .unwrap()
                .kind,
            PieceKind::King
        );
    }

    #[test]
    fn scholars_mate() {
        let mut game = Game::start();

        game.make_move(&ChessMove::PawnSkip(Move {
            origin: Coords { x: 4, y: 6 },
            destination: Coords { x: 4, y: 4 },
        }));
        assert!(game.current_position.to_move == PieceColor::Black);
        game.make_move(&ChessMove::PawnSkip(Move {
            origin: Coords { x: 4, y: 1 },
            destination: Coords { x: 4, y: 3 },
        }));
        game.make_move(&ChessMove::RegularMove(Move {
            origin: Coords { x: 3, y: 7 },
            destination: Coords { x: 7, y: 3 },
        }));
        assert!(game.current_position.to_move == PieceColor::Black);
        game.make_move(&ChessMove::RegularMove(Move {
            origin: Coords { x: 1, y: 0 },
            destination: Coords { x: 2, y: 2 },
        }));
        game.make_move(&ChessMove::RegularMove(Move {
            origin: Coords { x: 5, y: 7 },
            destination: Coords { x: 2, y: 4 },
        }));
        assert!(game.current_position.to_move == PieceColor::Black);
        game.make_move(&ChessMove::RegularMove(Move {
            origin: Coords { x: 6, y: 0 },
            destination: Coords { x: 5, y: 2 },
        }));
        game.make_move(&ChessMove::RegularMove(Move {
            origin: Coords { x: 7, y: 3 },
            destination: Coords { x: 5, y: 1 },
        }));
        assert!(game.current_position.to_move == PieceColor::Black);

        assert!(game.checkmated == Some(PieceColor::Black));
    }

    #[test]
    fn pawn_skip_is_legal() {
        let position = Position::initial();
        assert!(position.is_move_legal(&ChessMove::PawnSkip(Move {
            origin: Coords { x: 4, y: 6 },
            destination: Coords { x: 4, y: 4 }
        }),))
    }

    #[test]
    fn promotion() {
        let position = Position::from_fen("8/P7/8/8/8/8/8/8 w - - 0 1");
        let pawn_location = Coords { y: 1, x: 0 };
        dbg!(position.legal_moves_from_origin(&pawn_location));
        position
            .legal_moves_from_origin(&pawn_location)
            .iter()
            .for_each(|chess_move| match chess_move {
                ChessMove::Promotion(_, _) => (),
                _ => panic!("expected only promotions, found {:?}", chess_move),
            });
    }
}
