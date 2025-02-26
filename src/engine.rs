#[cfg(feature = "rng")]
use rand::prelude::IndexedRandom;
use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;

use crate::all_squares;
use crate::piece_at;
use crate::player::Player;
use crate::ChessMove;
use crate::Piece;
use crate::PieceColor;
use crate::PieceKind;
use crate::Position;

pub struct FirstMovePlayer;

impl Display for FirstMovePlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "First available move")
    }
}
impl Player for FirstMovePlayer {
    fn offer_move(&self, position: &Position) -> ChessMove {
        position.all_legal_moves().first().unwrap().clone()
    }
    fn evalutate(&self, _position: &Position) -> isize {
        0
    }
}

#[cfg(feature = "rng")]
pub struct RandomPlayer;

#[cfg(feature = "rng")]
impl Display for RandomPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Random")
    }
}

#[cfg(feature = "rng")]
impl Player for RandomPlayer {
    fn offer_move(&self, position: &Position) -> ChessMove {
        pick_random_move(position)
    }
    fn evalutate(&self, _position: &Position) -> isize {
        0
    }
}

#[cfg(feature = "rng")]
fn pick_random_move(position: &Position) -> ChessMove {
    position
        .all_legal_moves()
        .choose(&mut rand::rng())
        .unwrap()
        .clone()
}

#[cfg(feature = "rng")]
pub struct RandomCapturePrioPlayer;

#[cfg(feature = "rng")]
impl Display for RandomCapturePrioPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Prioritize Capture")
    }
}

#[cfg(feature = "rng")]
impl Player for RandomCapturePrioPlayer {
    fn offer_move(&self, position: &Position) -> ChessMove {
        let moves_with_capture: Vec<ChessMove> = position
            .all_legal_moves()
            .into_iter()
            .filter(|chess_move| {
                position
                    .after_move(chess_move)
                    .piece_count(position.to_move.opposite())
                    < position.piece_count(position.to_move.opposite())
            })
            .collect();
        if moves_with_capture.len() > 0 {
            moves_with_capture
                .choose(&mut rand::rng())
                .unwrap()
                .clone()
                .clone()
        } else {
            pick_random_move(position)
        }
    }
    fn evalutate(&self, _position: &Position) -> isize {
        0
    }
}

fn basic_evaluation(position: &Position) -> isize {
    fn piece_value(kind: &PieceKind) -> isize {
        match kind {
            PieceKind::King => 0,
            PieceKind::Pawn => 10,
            PieceKind::Rook => 50,
            PieceKind::Bishop => 30,
            PieceKind::Knight => 20,
            PieceKind::Queen => 100,
        }
    }
    fn evaluate_piece(piece: &Piece, is_attacked: bool, to_move: &PieceColor) -> isize {
        piece_value(&piece.kind)
            * (if to_move == &piece.color { 1 } else { -1 } * {
                if !is_attacked {
                    2
                } else {
                    1
                }
            })
    }
    all_squares()
        .iter()
        .map(|square| match piece_at(&position.board, square) {
            None => 0_isize,
            Some(piece) => evaluate_piece(
                &piece,
                position.is_attacked_by(&position.to_move.opposite(), square),
                &position.to_move,
            ),
        })
        .reduce(|acc, e| acc + e)
        .expect("all squares is never 0 length")
}
pub struct BasicEvaluationPlayer;

impl Display for BasicEvaluationPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Basic Evaluation")
    }
}

fn moves_with_evaluation(
    position: &Position,
    evaluation: fn(&Position) -> isize,
) -> HashMap<isize, Vec<ChessMove>> {
    let all_moves = position.all_legal_moves();
    let mut moves_by_evaluation = HashMap::new();
    all_moves.iter().for_each(|chess_move| {
        moves_by_evaluation
            .entry(evaluation(&position.after_move(chess_move)))
            .or_insert(Vec::new())
            .push(chess_move.clone())
    });
    moves_by_evaluation
}

fn first_move_with_max_evaluation(
    moves_by_evaluation: HashMap<isize, Vec<ChessMove>>,
) -> ChessMove {
    moves_by_evaluation
        .get(moves_by_evaluation.keys().max().unwrap())
        .unwrap()
        .first()
        .unwrap()
        .clone()
}

fn first_move_with_min_evaluation(
    moves_by_evaluation: HashMap<isize, Vec<ChessMove>>,
) -> ChessMove {
    moves_by_evaluation
        .get(moves_by_evaluation.keys().min().unwrap())
        .unwrap()
        .first()
        .unwrap()
        .clone()
}

impl Player for BasicEvaluationPlayer {
    fn offer_move(&self, position: &Position) -> ChessMove {
        first_move_with_max_evaluation(moves_with_evaluation(position, basic_evaluation))
    }
    fn evalutate(&self, position: &Position) -> isize {
        basic_evaluation(position)
    }
}

pub struct BetterEvaluationPlayer {}

impl Player for BetterEvaluationPlayer {
    fn offer_move(&self, position: &Position) -> ChessMove {
        first_move_with_min_evaluation(moves_with_evaluation(position, better_evaluation))
    }
    fn evalutate(&self, position: &Position) -> isize {
        -better_evaluation(position)
    }
}

impl Display for BetterEvaluationPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Better evaluation")
    }
}

fn better_evaluation(position: &Position) -> isize {
    fn piece_value(kind: &PieceKind) -> isize {
        match kind {
            PieceKind::King => 10000,
            PieceKind::Pawn => 100,
            PieceKind::Rook => 500,
            PieceKind::Bishop => 300,
            PieceKind::Knight => 200,
            PieceKind::Queen => 5000,
        }
    }
    fn evaluate_piece(
        piece: &Piece,
        is_attacked: bool,
        to_move: &PieceColor,
        controlled_squares: isize,
    ) -> isize {
        let value = piece_value(&piece.kind);
        let control_value = 2;
        let own_color_factor = if &piece.color == to_move { 1 } else { -1 };
        let attacked_factor = if is_attacked {
            if &piece.color == to_move {
                -5
            } else {
                -piece_value(&piece.kind)
            }
        } else {
            0
        };
        ((value + (controlled_squares * control_value)) + attacked_factor) * own_color_factor
    }
    let score_from_all_squares = all_squares()
        .iter()
        .map(|square| match piece_at(&position.board, square) {
            None => 0_isize,
            Some(piece) => evaluate_piece(
                &piece,
                position.is_attacked_by(&piece.color.opposite(), square),
                &position.to_move,
                position
                    .color_to_move(piece.color)
                    .legal_moves_from_origin(square)
                    .len()
                    .try_into()
                    .unwrap(),
            ),
        })
        .reduce(|acc, e| acc + e)
        .expect("all squares is never 0 length");
    let score_from_checkmate = if position.is_checkmate() { 10000000 } else { 0 };
    score_from_all_squares + score_from_checkmate
}

fn alpha_beta_negamax(
    position: &Position,
    depth: isize,
    evaluate: fn(position: &Position) -> isize,
    mut alpha: isize,
    beta: isize,
) -> isize {
    if depth == 0 || position.is_checkmate() || position.is_stalemate() {
        return evaluate(position);
    }
    let mut best = isize::MIN;
    for chess_move in position.all_legal_moves() {
        let eval = -alpha_beta_negamax(
            &position.after_move(&chess_move),
            depth - 1,
            evaluate,
            -beta,
            -alpha,
        );
        if eval > best {
            best = eval;
            if eval > alpha {
                alpha = eval;
            }
            if eval >= beta {
                return best;
            }
        }
    }
    best
}

fn negamax(position: &Position, depth: isize, evaluate: fn(&Position) -> isize) -> isize {
    if depth == 0 || position.is_checkmate() || position.is_stalemate() {
        return evaluate(position);
    }
    let mut best = isize::MIN;
    for chess_move in position.all_legal_moves() {
        best = cmp::max(
            best,
            -negamax(&position.after_move(&chess_move), depth - 1, evaluate),
        );
    }
    best
}

fn minimax(
    position: &Position,
    depth: isize,
    maximize: bool,
    evaluate: fn(position: &Position) -> isize,
) -> isize {
    if depth == 0 || position.is_checkmate() || position.is_stalemate() {
        return evaluate(position);
    }
    if maximize {
        let mut best = isize::MIN;
        for chess_move in position.all_legal_moves() {
            best = cmp::max(
                best,
                minimax(
                    &position.after_move(&chess_move),
                    depth - 1,
                    false,
                    evaluate,
                ),
            );
        }
        best
    } else {
        let mut worst = isize::MAX;
        for chess_move in position.all_legal_moves() {
            worst = cmp::min(
                worst,
                minimax(&position.after_move(&chess_move), depth - 1, true, evaluate),
            );
        }
        worst
    }
}

fn planner_evaluation(position: &Position) -> isize {
    -alpha_beta_negamax(
        position,
        2,
        better_evaluation,
        isize::MIN + 1,
        isize::MAX - 1,
    )
}
pub struct Planner;

impl Player for Planner {
    fn evalutate(&self, position: &Position) -> isize {
        planner_evaluation(position)
    }
    fn offer_move(&self, position: &Position) -> ChessMove {
        first_move_with_max_evaluation(moves_with_evaluation(position, planner_evaluation))
    }
}

impl Display for Planner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Planner")
    }
}

#[cfg(test)]
mod tests {
    use crate::Coords;
    use crate::Move;

    use super::*;
    #[test]
    fn better_evaluation_finds_king_rook_fork() {
        let position =
            Position::from_fen("rnb1kbnr/pppppppp/8/1N6/8/8/PPPPPPPP/R1BQKBNR w KQkq - 0 1");
        assert_eq!(
            BetterEvaluationPlayer {}.offer_move(&position),
            ChessMove::RegularMove(Move {
                origin: Coords { x: 1, y: 3 },
                destination: Coords { x: 2, y: 1 }
            })
        );
    }

    #[test]
    fn better_evaluation_doesnt_sac_knight_after_fork() {
        let position =
            Position::from_fen("Nnbk1bnr/pp1p1ppp/8/4p3/8/8/PPPPPPPP/R1BQKBNR w KQka - 0 1");
        assert_ne!(
            BetterEvaluationPlayer {}.offer_move(&position),
            ChessMove::RegularMove(Move {
                origin: Coords { x: 0, y: 0 },
                destination: Coords { x: 2, y: 1 }
            })
        );
    }

    #[test]
    fn planner_doesnt_sac_knight_after_fork() {
        let position =
            Position::from_fen("Nnbk1bnr/pp1p1ppp/8/4p3/8/8/PPPPPPPP/R1BQKBNR w KQka - 0 1");
        assert_ne!(
            Planner {}.offer_move(&position),
            ChessMove::RegularMove(Move {
                origin: Coords { x: 0, y: 0 },
                destination: Coords { x: 2, y: 1 }
            })
        );
    }

    #[test]
    fn planner_finds_king_rook_fork() {
        let position =
            Position::from_fen("rnb1kbnr/pppppppp/8/1N6/8/8/PPPPPPPP/R1BQKBNR w KQkq - 0 1");
        assert_eq!(
            Planner {}.offer_move(&position),
            ChessMove::RegularMove(Move {
                origin: Coords { x: 1, y: 3 },
                destination: Coords { x: 2, y: 1 }
            })
        );
    }
}
