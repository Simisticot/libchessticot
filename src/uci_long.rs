use crate::{
    board_manip::{king_at, pawn_at},
    ChessMove, Coords, Direction, Move, PieceColor, PieceKind, Position,
};

impl ChessMove {
    pub fn to_uci_long(&self, current_position: &Position) -> String {
        let origin: Coords = match self {
            ChessMove::RegularMove(movement)
            | ChessMove::PawnSkip(movement)
            | ChessMove::Promotion(movement, _)
            | ChessMove::EnPassant(movement, _) => movement.origin,
            ChessMove::CastleLeft | ChessMove::CastleRight => match current_position.to_move {
                PieceColor::Black => Coords::from_algebraic("e8"),
                PieceColor::White => Coords::from_algebraic("e1"),
            },
        };
        let destination: Coords = match self {
            ChessMove::RegularMove(movement)
            | ChessMove::PawnSkip(movement)
            | ChessMove::Promotion(movement, _)
            | ChessMove::EnPassant(movement, _) => movement.destination,
            ChessMove::CastleLeft => match current_position.to_move {
                PieceColor::Black => Coords::from_algebraic("c8"),
                PieceColor::White => Coords::from_algebraic("c1"),
            },
            ChessMove::CastleRight => match current_position.to_move {
                PieceColor::Black => Coords::from_algebraic("g8"),
                PieceColor::White => Coords::from_algebraic("g1"),
            },
        };
        origin.to_algebraic() + &destination.to_algebraic()
    }
    pub fn from_uci_long(uci_long: &str, current_position: &Position) -> ChessMove {
        assert!(uci_long.len() >= 4);
        assert!(uci_long.len() <= 5);
        let promotion_target: Option<PieceKind> = if uci_long.len() == 5 {
            match uci_long.chars().last().unwrap() {
                'q' => Some(PieceKind::Queen),
                'k' => Some(PieceKind::King),
                'r' => Some(PieceKind::Rook),
                'n' => Some(PieceKind::Knight),
                'b' => Some(PieceKind::Bishop),
                _ => panic!("unsuported promotion target"),
            }
        } else {
            None
        };

        let origin: Coords = Coords::from_algebraic(&uci_long[..2]);
        let destination: Coords = Coords::from_algebraic(&uci_long[2..4]);
        let movement = Move {
            origin,
            destination,
        };
        if let Some(target) = promotion_target {
            ChessMove::Promotion(movement, target)
        } else if pawn_at(&current_position.board, &movement.origin)
            && movement.y_abs_distance() > 1
        {
            ChessMove::PawnSkip(movement)
        } else if pawn_at(&current_position.board, &movement.origin)
            && current_position
                .en_passant_on
                .is_some_and(|square| square == movement.destination)
        {
            ChessMove::EnPassant(
                movement.clone(),
                movement.destination
                    + Direction {
                        dx: 0,
                        dy: current_position.to_move.pawn_orientation(),
                    },
            )
        } else if king_at(&current_position.board, &movement.origin) && movement.x_distance() == -2
        {
            ChessMove::CastleLeft
        } else if king_at(&current_position.board, &movement.origin) && movement.x_distance() == 2 {
            ChessMove::CastleRight
        } else {
            ChessMove::RegularMove(movement)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Coords, Move};

    use super::*;

    #[test]
    fn deserializes_e2_e4() {
        assert_eq!(
            ChessMove::from_uci_long("e2e4", &Position::initial()),
            ChessMove::PawnSkip(Move {
                origin: Coords { x: 4, y: 6 },
                destination: Coords { x: 4, y: 4 }
            })
        )
    }

    #[test]
    fn deserializes_promotion() {
        assert_eq!(
            ChessMove::from_uci_long("h7h8q", &Position::from_fen("8/7P/8/8/8/8/8/8 w - - 0 1")),
            ChessMove::Promotion(
                Move {
                    origin: Coords { x: 7, y: 1 },
                    destination: Coords { x: 7, y: 0 }
                },
                PieceKind::Queen
            )
        )
    }

    #[test]
    fn deserializes_en_passant() {
        assert_eq!(
            ChessMove::from_uci_long("e4d3", &Position::from_fen("8/8/8/8/3Pp3/8/8/8 w - d3 0 1")),
            ChessMove::EnPassant(
                Move {
                    origin: Coords::from_algebraic("e4"),
                    destination: Coords::from_algebraic("d3")
                },
                Coords::from_algebraic("d4")
            )
        )
    }

    #[test]
    fn deserializes_castle_right() {
        assert_eq!(
            ChessMove::from_uci_long(
                "e1c1",
                &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1")
            ),
            ChessMove::CastleRight
        )
    }

    #[test]
    fn deserializes_castle_left() {
        assert_eq!(
            ChessMove::from_uci_long(
                "e1g1",
                &Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1")
            ),
            ChessMove::CastleLeft
        )
    }

    #[test]
    fn deserializes_knight_to_c3() {
        assert_eq!(
            ChessMove::from_uci_long("b1c3", &Position::initial()),
            ChessMove::RegularMove(Move {
                origin: Coords::from_algebraic("b1"),
                destination: Coords::from_algebraic("c3")
            })
        )
    }

    #[test]
    fn serializes_e4() {
        assert_eq!(
            ChessMove::PawnSkip(Move {
                origin: Coords::from_algebraic("e2"),
                destination: Coords::from_algebraic("e4")
            })
            .to_uci_long(&Position::initial()),
            "e2e4"
        )
    }

    #[test]
    fn serializes_e5() {
        assert_eq!(
            ChessMove::PawnSkip(Move {
                origin: Coords::from_algebraic("e7"),
                destination: Coords::from_algebraic("e5")
            })
            .to_uci_long(&Position::initial()),
            "e7e5"
        )
    }
}
