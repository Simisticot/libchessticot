use std::str;

use crate::all_squares;
use crate::cards;
use crate::eight_degrees;
use crate::inter_cards;
use crate::move_piece;
use crate::piece_at;
use crate::put_piece_at;
use crate::take_piece_at;
use crate::ChessMove;
use crate::Coords;
use crate::Direction;
use crate::Move;
use crate::Piece;
use crate::PieceColor;
use crate::PieceKind;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub board: Vec<Vec<Option<Piece>>>,
    pub to_move: PieceColor,
    white_can_castle_queen_side: bool,
    white_can_castle_king_side: bool,
    black_can_castle_queen_side: bool,
    black_can_castle_king_side: bool,
    en_passant_on: Option<Coords>,
}

impl Position {
    pub fn initial() -> Position {
        let mut board = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                row.push(Piece::from_initial_position(j, i));
            }
            board.push(row);
        }
        Position {
            board,
            to_move: PieceColor::White,
            white_can_castle_king_side: true,
            white_can_castle_queen_side: true,
            black_can_castle_king_side: true,
            black_can_castle_queen_side: true,
            en_passant_on: None,
        }
    }
    pub fn empty_board() -> Position {
        let mut board = Vec::new();
        for _ in 0..8 {
            let mut row = Vec::new();
            for _ in 0..8 {
                row.push(None);
            }
            board.push(row);
        }
        Position {
            board,
            to_move: PieceColor::White,
            white_can_castle_king_side: true,
            white_can_castle_queen_side: true,
            black_can_castle_king_side: true,
            black_can_castle_queen_side: true,
            en_passant_on: None,
        }
    }
    pub fn from_fen(fen_record: &str) -> Position {
        let fields: Vec<&str> = fen_record.split(" ").collect();

        assert!(fields.len() == 6);

        let mut board = vec![vec![]; 8];
        let mut rank = 0;
        fields[0].chars().for_each(|character| match character {
            '1'..='8' => {
                for _ in 0..character.to_digit(10).expect("matched digits 1 through 8") {
                    board[rank].push(None);
                }
            }
            '/' => {
                rank += 1;
            }
            'r' => board[rank].push(Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::Black,
            })),
            'n' => board[rank].push(Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::Black,
            })),
            'b' => board[rank].push(Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::Black,
            })),
            'q' => board[rank].push(Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::Black,
            })),
            'k' => board[rank].push(Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::Black,
            })),
            'p' => board[rank].push(Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::Black,
            })),
            'R' => board[rank].push(Some(Piece {
                kind: PieceKind::Rook,
                color: PieceColor::White,
            })),
            'N' => board[rank].push(Some(Piece {
                kind: PieceKind::Knight,
                color: PieceColor::White,
            })),
            'B' => board[rank].push(Some(Piece {
                kind: PieceKind::Bishop,
                color: PieceColor::White,
            })),
            'Q' => board[rank].push(Some(Piece {
                kind: PieceKind::Queen,
                color: PieceColor::White,
            })),
            'K' => board[rank].push(Some(Piece {
                kind: PieceKind::King,
                color: PieceColor::White,
            })),
            'P' => board[rank].push(Some(Piece {
                kind: PieceKind::Pawn,
                color: PieceColor::White,
            })),
            _ => panic!("{} is not a valid board character in FEN", character),
        });

        assert_eq!(board.len(), 8);
        for rank in &board {
            assert_eq!(rank.len(), 8);
        }

        assert!(fields[1].len() == 1);

        let to_move = match fields[1]
            .chars()
            .nth(0)
            .expect("Second FEN field should be 'w' or 'b'")
        {
            'w' => PieceColor::White,
            'b' => PieceColor::Black,
            _ => panic!("Second FEN field should be 'w' or 'b'"),
        };

        let white_can_castle_left = fields[2].contains("Q");
        let white_can_castle_right = fields[2].contains("K");
        let black_can_castle_left = fields[2].contains("q");
        let black_can_castle_right = fields[2].contains("k");

        let en_passant_on = if fields[3] == "-" {
            None
        } else {
            Some(Coords::from_algebraic(fields[3]))
        };

        Position {
            board,
            to_move,
            en_passant_on,
            white_can_castle_queen_side: white_can_castle_left,
            white_can_castle_king_side: white_can_castle_right,
            black_can_castle_queen_side: black_can_castle_left,
            black_can_castle_king_side: black_can_castle_right,
        }
    }
    pub fn opposite_color_to_move(&self) -> Position {
        let mut new_position = self.clone();
        new_position.to_move = new_position.to_move.opposite();
        new_position
    }

    pub fn color_to_move(&self, color: PieceColor) -> Position {
        Position {
            to_move: color,
            ..self.clone()
        }
    }

    pub fn after_move(&self, chess_move: &ChessMove) -> Position {
        let mut new_board = self.board.clone();
        let mut en_passant_on = None;
        match chess_move {
            ChessMove::RegularMove(coordinates) => {
                move_piece(&mut new_board, coordinates.origin, coordinates.destination);
            }
            ChessMove::PawnSkip(movement) => {
                move_piece(&mut new_board, movement.origin, movement.destination);
                en_passant_on = Some(Coords {
                    x: movement.origin.x,
                    y: (movement.origin.y + movement.destination.y) / 2 as isize,
                });
            }
            ChessMove::CastleLeft => {
                let row = self.to_move.homerow();
                move_piece(
                    &mut new_board,
                    Coords { x: 4, y: row },
                    Coords { x: 2, y: row },
                );
                move_piece(
                    &mut new_board,
                    Coords { x: 0, y: row },
                    Coords { x: 3, y: row },
                );
            }
            ChessMove::CastleRight => {
                let row = self.to_move.homerow();
                move_piece(
                    &mut new_board,
                    Coords { x: 4, y: row },
                    Coords { x: 6, y: row },
                );
                move_piece(
                    &mut new_board,
                    Coords { x: 7, y: row },
                    Coords { x: 5, y: row },
                );
            }
            ChessMove::EnPassant(movement, pawn_taken) => {
                move_piece(&mut new_board, movement.origin, movement.destination);
                take_piece_at(&mut new_board, *pawn_taken);
            }
            ChessMove::Promotion(movement, promoted_to) => {
                take_piece_at(&mut new_board, movement.origin);
                put_piece_at(
                    &mut new_board,
                    Piece {
                        kind: *promoted_to,
                        color: self.to_move.clone(),
                    },
                    movement.destination,
                );
            }
        }

        let black_can_castle_king_side = match chess_move {
            ChessMove::CastleLeft => {
                self.to_move == PieceColor::White && self.black_can_castle_king_side
            }
            ChessMove::CastleRight => {
                self.to_move == PieceColor::White && self.black_can_castle_king_side
            }
            ChessMove::RegularMove(movement) => {
                ((movement.origin != Coords { y: 7, x: 4 }
                    && movement.origin != Coords { y: 7, x: 7 })
                    || self.to_move == PieceColor::White)
                    && self.black_can_castle_king_side
            }
            _ => self.black_can_castle_king_side,
        };

        let black_can_castle_queen_side = match chess_move {
            ChessMove::CastleLeft => {
                self.to_move == PieceColor::White && self.black_can_castle_queen_side
            }
            ChessMove::CastleRight => {
                self.to_move == PieceColor::White && self.black_can_castle_queen_side
            }
            ChessMove::RegularMove(movement) => {
                ((movement.origin != Coords { y: 7, x: 4 }
                    && movement.origin != Coords { y: 7, x: 0 })
                    || self.to_move == PieceColor::White)
                    && self.black_can_castle_queen_side
            }
            _ => self.black_can_castle_queen_side,
        };
        let white_can_castle_king_side = match chess_move {
            ChessMove::CastleLeft => {
                self.to_move == PieceColor::Black && self.white_can_castle_king_side
            }
            ChessMove::CastleRight => {
                self.to_move == PieceColor::Black && self.white_can_castle_king_side
            }
            ChessMove::RegularMove(movement) => {
                ((movement.origin != Coords { y: 7, x: 4 }
                    && movement.origin != Coords { y: 7, x: 7 })
                    || self.to_move == PieceColor::Black)
                    && self.white_can_castle_king_side
            }
            _ => self.white_can_castle_king_side,
        };

        let white_can_castle_queen_side = match chess_move {
            ChessMove::CastleLeft => {
                self.to_move == PieceColor::Black && self.white_can_castle_queen_side
            }
            ChessMove::CastleRight => {
                self.to_move == PieceColor::Black && self.white_can_castle_queen_side
            }
            ChessMove::RegularMove(movement) => {
                ((movement.origin != Coords { y: 7, x: 4 }
                    && movement.origin != Coords { y: 7, x: 0 })
                    || self.to_move == PieceColor::Black)
                    && self.white_can_castle_queen_side
            }
            _ => self.white_can_castle_queen_side,
        };

        Position {
            board: new_board,
            to_move: self.to_move.opposite(),
            en_passant_on,
            white_can_castle_queen_side,
            white_can_castle_king_side,
            black_can_castle_queen_side,
            black_can_castle_king_side,
            ..self.clone()
        }
    }
    pub fn is_checkmate(&self) -> bool {
        return self.is_in_check(&self.to_move) && self.all_legal_moves().len() == 0;
    }
    pub fn checkmated(&self) -> Option<PieceColor> {
        if self.is_checkmate() {
            Some(self.to_move)
        } else {
            None
        }
    }
    pub fn all_legal_moves(&self) -> Vec<ChessMove> {
        all_squares()
            .iter()
            .map(|square| self.legal_moves_from_origin(square))
            .flatten()
            .collect()
    }

    fn all_possible_moves(&self) -> Vec<ChessMove> {
        all_squares()
            .iter()
            .map(|square| self.possible_moves_from_origin(square))
            .flatten()
            .collect()
    }
    pub fn legal_moves_from_origin(&self, origin: &Coords) -> Vec<ChessMove> {
        self.possible_moves_from_origin(origin)
            .iter()
            .cloned()
            .filter(|chess_move| !self.opens_own_king(chess_move))
            .collect()
    }

    fn possible_moves_from_origin(&self, origin: &Coords) -> Vec<ChessMove> {
        match piece_at(&self.board, origin) {
            None => Vec::new(),
            Some(piece) => {
                if piece.color == self.to_move {
                    self.movement_from_origin(origin, piece)
                } else {
                    Vec::new()
                }
            }
        }
    }
    pub fn is_move_legal(&self, chess_move: &ChessMove) -> bool {
        let origin = match chess_move {
            ChessMove::RegularMove(movement) => movement.origin,
            ChessMove::PawnSkip(movement) => movement.origin,
            ChessMove::EnPassant(movement, _) => movement.origin,
            ChessMove::CastleRight | ChessMove::CastleLeft => {
                let row = self.to_move.homerow();
                Coords { y: row, x: 4 }
            }
            ChessMove::Promotion(movement, _) => movement.origin,
        };

        self.legal_moves_from_origin(&origin).contains(chess_move)
    }
    pub fn is_attacked_by(&self, by: &PieceColor, square: &Coords) -> bool {
        let attacked_by_king: bool =
            self.projected_movement(square, eight_degrees(), &by.opposite(), Some(1))
                .iter()
                .any(|chess_move| match chess_move {
                    ChessMove::RegularMove(movement) => {
                        piece_at(&self.board, &movement.destination).is_some_and(|piece| {
                            piece.kind == PieceKind::King && &piece.color == by
                        })
                    }
                    _ => false,
                });
        let attacked_by_rook_or_queen: bool =
            self.rook_from(square, &by.opposite())
                .iter()
                .any(|chess_move| match chess_move {
                    ChessMove::RegularMove(movement) => {
                        piece_at(&self.board, &movement.destination).is_some_and(|piece| {
                            (piece.kind == PieceKind::Rook || piece.kind == PieceKind::Queen)
                                && &piece.color == by
                        })
                    }
                    _ => false,
                });

        let attacked_by_bishop_or_queen: bool = self
            .bishop_from(square, &by.opposite())
            .iter()
            .any(|chess_move| match chess_move {
                ChessMove::RegularMove(movement) => piece_at(&self.board, &movement.destination)
                    .is_some_and(|piece| {
                        (piece.kind == PieceKind::Bishop || piece.kind == PieceKind::Queen)
                            && &piece.color == by
                    }),
                _ => false,
            });
        let attacked_by_knight: bool =
            self.knight_from(square, &by.opposite())
                .iter()
                .any(|chess_move| match chess_move {
                    ChessMove::RegularMove(movement) => {
                        piece_at(&self.board, &movement.destination).is_some_and(|piece| {
                            piece.kind == PieceKind::Knight && &piece.color == by
                        })
                    }
                    _ => false,
                });

        let attacked_by_pawn: bool = self.attacked_by_pawn(square, by);
        let attacked_en_passant: bool = piece_at(&self.board, square)
            .is_some_and(|piece| piece.color == by.opposite() && piece.kind == PieceKind::Pawn)
            && self.en_passant_on.is_some_and(|en_passant_on| {
                en_passant_on
                    == *square
                        + Direction {
                            dx: 0,
                            dy: by.pawn_orientation(),
                        }
                    && self.attacked_by_pawn(&en_passant_on, by)
            });

        attacked_by_knight
            || attacked_by_pawn
            || attacked_en_passant
            || attacked_by_bishop_or_queen
            || attacked_by_rook_or_queen
            || attacked_by_king
    }

    fn attacked_by_pawn(&self, square: &Coords, attacking_color: &PieceColor) -> bool {
        Position::pawn_attacked_squares(square, &attacking_color.opposite())
            .iter()
            .any(|attacking_square| {
                attacking_square.is_in_bounds()
                    && piece_at(&self.board, &attacking_square).is_some_and(|piece| {
                        piece.kind == PieceKind::Pawn && &piece.color == attacking_color
                    })
            })
    }

    fn is_in_check(&self, color: &PieceColor) -> bool {
        match self.king_location(color) {
            None => false,
            Some(loc) => self.is_attacked_by(&color.opposite(), &loc),
        }
    }
    fn opens_own_king(&self, chess_move: &ChessMove) -> bool {
        let potential_position = self.after_move(chess_move);
        potential_position.is_in_check(&self.to_move)
    }

    pub fn can_castle_queen_side(&self, color: &PieceColor) -> bool {
        match color {
            PieceColor::White => self.white_can_castle_queen_side,
            PieceColor::Black => self.black_can_castle_queen_side,
        }
    }
    pub fn can_castle_king_side(&self, color: &PieceColor) -> bool {
        match color {
            PieceColor::White => self.white_can_castle_king_side,
            PieceColor::Black => self.black_can_castle_king_side,
        }
    }
    fn movement_from_origin(&self, origin: &Coords, piece: Piece) -> Vec<ChessMove> {
        match piece.kind {
            PieceKind::Pawn => self.pawn_from(origin, &piece.color),
            PieceKind::Rook => self.rook_from(origin, &piece.color),
            PieceKind::Knight => self.knight_from(origin, &piece.color),
            PieceKind::Bishop => self.bishop_from(origin, &piece.color),
            PieceKind::Queen => self.queen_movement(origin, &piece.color),
            PieceKind::King => self.king_movement(origin, &piece.color),
        }
    }
    fn king_movement(&self, origin: &Coords, origin_color: &PieceColor) -> Vec<ChessMove> {
        let mut moves = self.projected_movement(origin, eight_degrees(), origin_color, Some(1));
        let row = origin_color.homerow();
        if piece_at(&self.board, &Coords { y: row, x: 5 }).is_none()
            && piece_at(&self.board, &Coords { y: row, x: 6 }).is_none()
            && piece_at(&self.board, &Coords { y: row, x: 4 }).is_some_and(|piece| {
                piece
                    == Piece {
                        kind: PieceKind::King,
                        color: origin_color.clone(),
                    }
            })
            && piece_at(&self.board, &Coords { y: row, x: 7 }).is_some_and(|piece| {
                piece
                    == Piece {
                        kind: PieceKind::Rook,
                        color: origin_color.clone(),
                    }
            })
            && self.can_castle_king_side(origin_color)
            && !self.is_in_check(origin_color)
        {
            moves.push(ChessMove::CastleRight);
        }
        if piece_at(&self.board, &Coords { y: row, x: 3 }).is_none()
            && piece_at(&self.board, &Coords { y: row, x: 2 }).is_none()
            && piece_at(&self.board, &Coords { y: row, x: 1 }).is_none()
            && piece_at(&self.board, &Coords { y: row, x: 4 }).is_some_and(|piece| {
                piece
                    == Piece {
                        kind: PieceKind::King,
                        color: origin_color.clone(),
                    }
            })
            && piece_at(&self.board, &Coords { y: row, x: 0 }).is_some_and(|piece| {
                piece
                    == Piece {
                        kind: PieceKind::Rook,
                        color: origin_color.clone(),
                    }
            })
            && self.can_castle_queen_side(origin_color)
            && !self.is_in_check(origin_color)
        {
            moves.push(ChessMove::CastleLeft);
        }

        moves
    }
    fn queen_movement(&self, origin: &Coords, color: &PieceColor) -> Vec<ChessMove> {
        self.projected_movement(origin, eight_degrees(), color, None)
    }
    fn bishop_from(&self, origin: &Coords, color: &PieceColor) -> Vec<ChessMove> {
        self.projected_movement(origin, inter_cards(), color, None)
    }
    fn knight_from(&self, origin: &Coords, color: &PieceColor) -> Vec<ChessMove> {
        let directions: Vec<Direction> = vec![
            Direction { dy: 2, dx: 1 },
            Direction { dy: 2, dx: -1 },
            Direction { dy: 1, dx: 2 },
            Direction { dy: 1, dx: -2 },
            Direction { dy: -2, dx: 1 },
            Direction { dy: -2, dx: -1 },
            Direction { dy: -1, dx: -2 },
            Direction { dy: -1, dx: 2 },
        ];
        let potential_moves = directions.iter().map(|direction| {
            ChessMove::RegularMove(Move {
                origin: origin.clone(),
                destination: *origin + *direction,
            })
        });
        potential_moves
            .into_iter()
            .filter(|chess_move| match chess_move {
                ChessMove::RegularMove(coordinates) => {
                    coordinates.destination.is_in_bounds()
                        && piece_at(&self.board, &coordinates.destination)
                            .is_none_or(|piece| &piece.color != color)
                }
                _ => false,
            })
            .collect()
    }
    fn rook_from(&self, origin: &Coords, color: &PieceColor) -> Vec<ChessMove> {
        self.projected_movement(origin, cards(), color, None)
    }

    fn pawn_attacked_squares(origin: &Coords, color: &PieceColor) -> Vec<Coords> {
        vec![
            Coords {
                x: origin.x + 1,
                y: origin.y + color.pawn_orientation(),
            },
            Coords {
                x: origin.x - 1,
                y: origin.y + color.pawn_orientation(),
            },
        ]
    }

    fn pawn_from(&self, origin: &Coords, color: &PieceColor) -> Vec<ChessMove> {
        let mut legal_moves = vec![];
        let forward = Direction {
            dx: 0,
            dy: color.pawn_orientation(),
        };
        let ahead_one = *origin + forward;
        let ahead_two = ahead_one + forward;

        if !ahead_one.is_in_bounds() {
            return legal_moves;
        }

        if piece_at(&self.board, &ahead_one).is_none() {
            legal_moves.push(ChessMove::RegularMove(Move {
                origin: origin.clone(),
                destination: ahead_one,
            }));
            if ahead_two.is_in_bounds()
                && (origin.y == 1 || origin.y == 6)
                && piece_at(&self.board, &ahead_two).is_none()
            {
                legal_moves.push(ChessMove::PawnSkip(Move {
                    origin: origin.clone(),
                    destination: ahead_two,
                }));
            }
        }

        Position::pawn_attacked_squares(origin, color)
            .iter()
            .for_each(|diagonal| {
                if diagonal.is_in_bounds() {
                    match piece_at(&self.board, &diagonal) {
                        None => {}
                        Some(piece) => {
                            if piece.color == color.opposite() {
                                legal_moves.push(ChessMove::RegularMove(Move {
                                    origin: origin.clone(),
                                    destination: *diagonal,
                                }));
                            }
                        }
                    }
                }
            });
        if let Some(en_passant) = self.en_passant_from(origin, color) {
            legal_moves.push(en_passant);
        }
        legal_moves
            .iter()
            .map(|pawn_move| match pawn_move {
                ChessMove::RegularMove(movement) => {
                    if movement.destination.y == color.opposite().homerow() {
                        PieceKind::promoteable()
                            .map(|promotable_kind| {
                                ChessMove::Promotion(movement.clone(), promotable_kind.clone())
                            })
                            .collect()
                    } else {
                        vec![pawn_move.clone()]
                    }
                }
                ChessMove::PawnSkip(_) => vec![pawn_move.clone()],
                ChessMove::EnPassant(_, _) => vec![pawn_move.clone()],
                _ => panic!("Pawn moves should only be regular, skip or en passant"),
            })
            .flatten()
            .collect()
    }
    fn en_passant_from(&self, origin: &Coords, color: &PieceColor) -> Option<ChessMove> {
        match self.en_passant_on {
            None => None,
            Some(coordinates) => {
                for candidate in vec![
                    coordinates
                        + Direction {
                            dx: 1,
                            dy: color.opposite().pawn_orientation(),
                        },
                    coordinates
                        + Direction {
                            dx: -1,
                            dy: color.opposite().pawn_orientation(),
                        },
                ] {
                    if candidate.is_in_bounds() && candidate == *origin {
                        return Some(ChessMove::EnPassant(
                            Move {
                                origin: origin.clone(),
                                destination: coordinates.clone(),
                            },
                            coordinates
                                + Direction {
                                    dx: 0,
                                    dy: color.opposite().pawn_orientation(),
                                },
                        ));
                    }
                }
                None
            }
        }
    }
    fn king_location(&self, color: &PieceColor) -> Option<Coords> {
        for i in 0..8 {
            for j in 0..8 {
                let loc = Coords { y: i, x: j };
                if piece_at(&self.board, &loc)
                    .is_some_and(|piece| piece.kind == PieceKind::King && piece.color == *color)
                {
                    return Some(loc);
                }
            }
        }
        None
    }
    fn projected_movement(
        &self,
        origin: &Coords,
        directions: Vec<Direction>,
        origin_color: &PieceColor,
        limit: Option<isize>,
    ) -> Vec<ChessMove> {
        directions
            .iter()
            .map(|dir| self.raycast(origin, dir, origin_color, limit))
            .flatten()
            .map(|destination| {
                ChessMove::RegularMove(Move {
                    origin: origin.clone(),
                    destination,
                })
            })
            .collect()
    }
    pub fn raycast(
        &self,
        origin: &Coords,
        direction: &Direction,
        origin_color: &PieceColor,
        limit: Option<isize>,
    ) -> Vec<Coords> {
        let limit = limit.unwrap_or(7) + 1;
        let mut squares = vec![];
        // for instead of loop to avoid potential infinite loop
        for i in 1..limit {
            let next_square = *origin + (*direction * i);
            if !next_square.is_in_bounds() {
                break;
            }
            if let Some(piece) = piece_at(&self.board, &next_square) {
                if piece.color == origin_color.opposite() {
                    squares.push(next_square);
                }
                break;
            }
            squares.push(next_square);
        }
        squares
    }
    pub fn piece_count(&self, color: PieceColor) -> usize {
        all_squares()
            .iter()
            .filter(|square| {
                piece_at(&self.board, square).is_some_and(|piece| piece.color == color)
            })
            .count()
    }

    pub fn is_stalemate(&self) -> bool {
        self.all_legal_moves().len() == 0 && !self.is_in_check(&self.to_move)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn initial_position_from_fen() {
        assert_eq!(
            Position::initial(),
            Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        );
    }

    #[test]
    fn finds_complex_checkmate() {
        assert!(Position::from_fen(
            "r1bqkbnr/2pp1Qpp/ppn5/4p3/2BPP3/8/PPP2PPP/RNB1K1NR b KQkq - 0 1"
        )
        .is_checkmate());
    }

    #[test]
    fn execute_move_into_check() {
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
        let king_destination = Coords { y: 0, x: 1 };

        let new_position = position.after_move(&ChessMove::RegularMove(Move {
            origin: king_location,
            destination: king_destination,
        }));
        assert!(new_position.king_location(&PieceColor::White) == Some(king_destination.clone()));
        assert!(new_position.is_attacked_by(&PieceColor::Black, &king_destination,));
        assert!(new_position.is_in_check(&PieceColor::White));
    }

    #[test]
    fn detects_check() {
        let mut position = Position::empty_board();

        position.board[0][1] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        });
        position.board[2][2] = Some(Piece {
            kind: PieceKind::Knight,
            color: PieceColor::Black,
        });
        assert!(position.is_in_check(&PieceColor::White));
    }

    #[test]
    fn detects_move_into_check() {
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
        assert!(position.opens_own_king(&ChessMove::RegularMove(Move {
            origin: king_location,
            destination: Coords { y: 0, x: 1 },
        }),));
    }

    #[test]
    fn detects_knight_attack() {
        let position = Position::from_fen("8/8/8/8/8/1n6/8/K7 b - - 0 1");
        let king_location = Coords { y: 7, x: 0 };
        assert!(position.is_attacked_by(&PieceColor::Black, &king_location));
    }

    #[test]
    fn detects_pawn_attack() {
        let position = Position::from_fen("8/8/8/4p3/3K4/8/8/8 w - - 0 1");
        let king_location = Coords { y: 4, x: 3 };
        assert!(position.is_attacked_by(&PieceColor::Black, &king_location));
    }

    #[test]
    fn en_passant_attack_requires_pawn_present() {
        let position = Position::from_fen("8/8/8/8/4P3/8/8/8 b - e3 0 1");
        let pawn_location = Coords { y: 4, x: 4 };
        assert!(!position.is_attacked_by(&PieceColor::Black, &pawn_location));
    }

    #[test]
    fn detects_en_passant_attack() {
        let position = Position::from_fen("8/8/8/8/4Pp2/8/8/8 b - e3 0 1");
        let pawn_location = Coords { y: 4, x: 4 };
        assert!(position.is_attacked_by(&PieceColor::Black, &pawn_location));
    }

    #[test]
    fn detects_bishop_attack() {
        let position = Position::from_fen("8/7b/8/8/8/8/2K5/8 w - - 0 1");
        let king_location = Coords { y: 6, x: 2 };
        assert!(position.is_attacked_by(&PieceColor::Black, &king_location));
    }

    #[test]
    fn detects_diagonal_queen_attack() {
        let position = Position::from_fen("8/7q/8/8/8/8/2K5/8 w - - 0 1");
        let king_location = Coords { y: 6, x: 2 };
        assert!(position.is_attacked_by(&PieceColor::Black, &king_location));
    }

    #[test]
    fn detects_rook_attack() {
        let position = Position::from_fen("2r5/8/8/8/8/8/2K5/8 w - - 0 1");
        let king_location = Coords { y: 6, x: 2 };
        assert!(position.is_attacked_by(&PieceColor::Black, &king_location));
    }

    #[test]
    fn detects_cardinal_queen_attack() {
        let position = Position::from_fen("2q5/8/8/8/8/8/2K5/8 w - - 0 1");
        let king_location = Coords { y: 6, x: 2 };
        assert!(position.is_attacked_by(&PieceColor::Black, &king_location));
    }

    #[test]
    fn detects_cardinal_king_attack() {
        let position = Position::from_fen("8/8/8/8/8/2k5/2K5/8 w - - 0 1");
        let white_king_location = Coords { y: 6, x: 2 };
        assert!(position.is_attacked_by(&PieceColor::Black, &white_king_location));
    }

    #[test]
    fn detects_diagonal_king_attack() {
        let position = Position::from_fen("8/8/8/8/8/3k4/2K5/8 w - - 0 1");
        let white_king_location = Coords { y: 6, x: 2 };
        assert!(position.is_attacked_by(&PieceColor::Black, &white_king_location));
    }

    #[test]
    fn promotion_is_an_attack() {
        let position = Position::from_fen("8/8/8/8/8/8/1p6/K7 w - - 0 1");
        let king_location = Coords { y: 7, x: 0 };
        position
            .color_to_move(PieceColor::Black)
            .all_possible_moves()
            .iter()
            .for_each(|chess_move| match chess_move {
                ChessMove::Promotion(_, _) => (),
                _ => panic!("only promotions in this position, found {:?}", chess_move),
            });
        assert!(position.is_attacked_by(&PieceColor::Black, &king_location,));
    }

    #[test]
    fn no_en_passant_from_accross_the_board() {
        let mut position = Position::empty_board();
        position.board[1][4] = Some(Piece {
            kind: PieceKind::Pawn,
            color: PieceColor::White,
        });
        position.board[7][2] = Some(Piece {
            kind: PieceKind::Pawn,
            color: PieceColor::Black,
        });
        let after_skip = position.after_move(&ChessMove::PawnSkip(Move {
            origin: Coords { y: 1, x: 4 },
            destination: Coords { y: 3, x: 4 },
        }));

        assert!(after_skip.en_passant_on == Some(Coords { y: 2, x: 4 }));
        assert!(!after_skip.is_move_legal(&ChessMove::EnPassant(
            Move {
                origin: Coords { y: 7, x: 2 },
                destination: Coords { y: 2, x: 4 }
            },
            Coords { y: 3, x: 4 }
        ),))
    }

    #[test]
    fn en_passant_left() {
        let position = Position::from_fen("8/8/8/8/2p5/8/1P6/8 w - - 0 1");
        let after_skip = position.after_move(&ChessMove::PawnSkip(Move {
            origin: Coords { y: 6, x: 1 },
            destination: Coords { y: 4, x: 1 },
        }));
        let black_pawn_location = Coords { y: 4, x: 2 };
        let ep = ChessMove::EnPassant(
            Move {
                origin: black_pawn_location,
                destination: Coords { y: 5, x: 1 },
            },
            Coords { y: 4, x: 1 },
        );
        assert_eq!(after_skip.en_passant_on, Some(Coords { y: 5, x: 1 }));
        assert!(after_skip
            .legal_moves_from_origin(&black_pawn_location)
            .contains(&ep));
        assert!(after_skip.is_move_legal(&ep))
    }

    #[test]
    fn en_passant_right() {
        let position = Position::from_fen("8/8/8/8/p7/8/1P6/8 w - - 0 1");
        let after_skip = position.after_move(&ChessMove::PawnSkip(Move {
            origin: Coords { y: 6, x: 1 },
            destination: Coords { y: 4, x: 1 },
        }));
        let black_pawn_location = Coords { y: 4, x: 0 };
        let ep = ChessMove::EnPassant(
            Move {
                origin: black_pawn_location,
                destination: Coords { y: 5, x: 1 },
            },
            Coords { y: 4, x: 1 },
        );
        assert!(after_skip.en_passant_on == Some(Coords { y: 5, x: 1 }));
        assert!(after_skip
            .legal_moves_from_origin(&black_pawn_location)
            .contains(&ep));
        assert!(after_skip.is_move_legal(&ep))
    }

    #[test]
    fn finds_king() {
        let mut position = Position::empty_board();

        position.board[0][0] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        });
        assert_eq!(
            position.king_location(&PieceColor::White).unwrap(),
            Coords { x: 0, y: 0 }
        )
    }

    #[test]
    fn cant_castle_after_moving_king() {
        let position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let king_initial_location = Coords { y: 7, x: 4 };
        let one_above = Coords { y: 6, x: 4 };

        let after_move_up = position.after_move(&ChessMove::RegularMove(Move {
            origin: king_initial_location,
            destination: one_above,
        }));

        assert!(!after_move_up.white_can_castle_king_side);
        assert!(!after_move_up.white_can_castle_queen_side);
        assert!(!after_move_up.is_move_legal(&ChessMove::CastleLeft));
        assert!(!after_move_up.is_move_legal(&ChessMove::CastleRight));

        let after_move_back = after_move_up.after_move(&ChessMove::RegularMove(Move {
            origin: one_above,
            destination: king_initial_location,
        }));

        assert!(!after_move_back.black_can_castle_king_side);
        assert!(!after_move_back.black_can_castle_queen_side);
        assert!(!after_move_back.is_move_legal(&ChessMove::CastleLeft));
        assert!(!after_move_back.is_move_legal(&ChessMove::CastleRight));
    }

    #[test]
    fn cannot_castle_after_moving_rook() {
        let position = Position::from_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        let left_rook_initial_location = Coords { y: 7, x: 0 };
        let left_rook_up_one = Coords { y: 6, x: 0 };
        let right_rook_initial_location = Coords { y: 7, x: 7 };
        let right_rook_up_one = Coords { y: 6, x: 7 };

        let moved_left_rook_up_one = position
            .after_move(&ChessMove::RegularMove(Move {
                origin: left_rook_initial_location,
                destination: left_rook_up_one,
            }))
            .color_to_move(PieceColor::White);

        assert!(!moved_left_rook_up_one.white_can_castle_queen_side);
        assert!(moved_left_rook_up_one.white_can_castle_king_side);

        assert!(!moved_left_rook_up_one.is_move_legal(&ChessMove::CastleLeft));
        assert!(piece_at(
            &moved_left_rook_up_one.board,
            &Coords {
                y: PieceColor::White.homerow(),
                x: 5
            }
        )
        .is_none());
        assert!(piece_at(
            &moved_left_rook_up_one.board,
            &Coords {
                y: PieceColor::White.homerow(),
                x: 6
            }
        )
        .is_none());
        assert!(piece_at(
            &moved_left_rook_up_one.board,
            &Coords {
                y: PieceColor::White.homerow(),
                x: 4
            }
        )
        .is_some_and(|piece| {
            piece
                == Piece {
                    kind: PieceKind::King,
                    color: PieceColor::White,
                }
        }));
        assert!(piece_at(
            &moved_left_rook_up_one.board,
            &Coords {
                y: PieceColor::White.homerow(),
                x: 7
            }
        )
        .is_some_and(|piece| {
            piece
                == Piece {
                    kind: PieceKind::Rook,
                    color: PieceColor::White,
                }
        }));
        assert!(moved_left_rook_up_one.can_castle_king_side(&PieceColor::White));
        assert!(moved_left_rook_up_one
            .all_legal_moves()
            .contains(&ChessMove::CastleRight));
        assert!(moved_left_rook_up_one.is_move_legal(&ChessMove::CastleRight));

        let moved_right_rook_up_one = moved_left_rook_up_one
            .after_move(&ChessMove::RegularMove(Move {
                origin: right_rook_initial_location,
                destination: right_rook_up_one,
            }))
            .color_to_move(PieceColor::White);

        assert!(!moved_right_rook_up_one.white_can_castle_king_side);
        assert!(!moved_right_rook_up_one.is_move_legal(&ChessMove::CastleRight));

        let moved_rooks_back = moved_right_rook_up_one
            .after_move(&ChessMove::RegularMove(Move {
                origin: left_rook_up_one,
                destination: left_rook_initial_location,
            }))
            .after_move(&ChessMove::RegularMove(Move {
                origin: right_rook_up_one,
                destination: right_rook_initial_location,
            }));

        assert!(!moved_rooks_back.white_can_castle_king_side);
        assert!(!moved_rooks_back.white_can_castle_queen_side);
        assert!(!moved_rooks_back.is_move_legal(&ChessMove::CastleRight));
        assert!(!moved_rooks_back.is_move_legal(&ChessMove::CastleLeft));
    }

    #[test]
    fn detects_stalemate() {
        let mut position = Position::empty_board();
        position.board[0][0] = Some(Piece {
            kind: PieceKind::King,
            color: PieceColor::White,
        });
        position.board[2][1] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        });
        position.board[1][2] = Some(Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        });

        assert!(position.is_stalemate());
    }

    #[test]
    fn cannot_castle_queenside_while_in_check() {
        let position = Position::from_fen("8/8/8/8/8/8/2n5/R3K3 w Q - 0 1");
        assert!(!position.is_move_legal(&ChessMove::CastleLeft));
    }
    #[test]
    fn cannot_castle_kingside_while_in_check() {
        let position = Position::from_fen("8/8/8/8/8/8/2n5/4K2R w K - 0 1");
        assert!(!position.is_move_legal(&ChessMove::CastleRight));
    }
}
