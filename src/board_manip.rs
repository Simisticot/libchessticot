use crate::Coords;
use crate::Piece;
use crate::PieceKind;

pub fn move_piece(board: &mut [Vec<Option<Piece>>], origin: Coords, dest: Coords) {
    if let Some(origin_piece) = take_piece_at(board, origin) {
        put_piece_at(board, origin_piece, dest);
    }
}
pub fn piece_at(board: &[Vec<Option<Piece>>], loc: &Coords) -> Option<Piece> {
    board[loc.y as usize][loc.x as usize]
}

pub fn pawn_at(board: &[Vec<Option<Piece>>], loc: &Coords) -> bool {
    piece_at(board, loc).is_some_and(|piece| piece.kind == PieceKind::Pawn)
}

pub fn king_at(board: &[Vec<Option<Piece>>], loc: &Coords) -> bool {
    piece_at(board, loc).is_some_and(|piece| piece.kind == PieceKind::King)
}

pub fn take_piece_at(board: &mut [Vec<Option<Piece>>], loc: Coords) -> Option<Piece> {
    board[loc.y as usize][loc.x as usize].take()
}
pub fn put_piece_at(board: &mut [Vec<Option<Piece>>], piece: Piece, loc: Coords) {
    board[loc.y as usize][loc.x as usize] = Some(piece);
}
