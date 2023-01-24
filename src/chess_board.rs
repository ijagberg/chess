use crate::piece::PieceType;
use crate::Color;
use crate::Piece;
use bitboard::*;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOrAssign;
use std::ops::Deref;
use std::ops::Neg;
use std::ops::Not;
use std::ops::Shl;
use std::ops::Shr;
use std::{fmt::Debug, ops::BitOr};

#[derive(Debug, Clone, Copy)]
pub struct ChessBoard {
    white_kings: Bitboard,
    black_kings: Bitboard,
    all_kings: Bitboard,
    white_queens: Bitboard,
    black_queens: Bitboard,
    all_queens: Bitboard,
    white_rooks: Bitboard,
    black_rooks: Bitboard,
    all_rooks: Bitboard,
    white_knights: Bitboard,
    black_knights: Bitboard,
    all_knights: Bitboard,
    white_bishops: Bitboard,
    black_bishops: Bitboard,
    all_bishops: Bitboard,
    white_pawns: Bitboard,
    black_pawns: Bitboard,
    all_pawns: Bitboard,
    white_pieces: Bitboard,
    black_pieces: Bitboard,
    all_pieces: Bitboard,
}

impl ChessBoard {
    pub fn new() -> Self {
        let white_kings = Bitboard::with_one(E1);
        let black_kings = Bitboard::with_one(E8);
        let all_kings = white_kings | black_kings;
        let white_queens = Bitboard::with_one(D1);
        let black_queens = Bitboard::with_one(D8);
        let all_queens = white_queens | black_queens;
        let white_rooks = Bitboard::with_ones([A1, H1]);
        let black_rooks = Bitboard::with_ones([A8, H8]);
        let all_rooks = white_rooks | black_rooks;
        let white_knights = Bitboard::with_ones([B1, G1]);
        let black_knights = Bitboard::with_ones([B8, G8]);
        let all_knights = white_knights | black_knights;
        let white_bishops = Bitboard::with_ones([C1, F1]);
        let black_bishops = Bitboard::with_ones([C8, F8]);
        let all_bishops = white_bishops | black_bishops;
        let white_pawns = Bitboard::with_ones(RANK_TWO);
        let black_pawns = Bitboard::with_ones(RANK_SEVEN);
        let all_pawns = white_pawns | black_pawns;
        let white_pieces =
            white_kings | white_queens | white_rooks | white_knights | white_bishops | white_pawns;
        let black_pieces =
            black_kings | black_queens | black_rooks | black_knights | black_bishops | black_pawns;
        let all_pieces = white_pieces | black_pieces;

        dbg!(Self {
            white_kings,
            black_kings,
            all_kings,
            white_queens,
            black_queens,
            all_queens,
            white_rooks,
            black_rooks,
            all_rooks,
            white_knights,
            black_knights,
            all_knights,
            white_bishops,
            black_bishops,
            all_bishops,
            white_pawns,
            black_pawns,
            all_pawns,
            white_pieces,
            black_pieces,
            all_pieces,
        })
    }

    pub fn full_occupancy(&self) -> Bitboard {
        self.all_pieces
    }

    pub fn get_occupancy_for_color(&self, color: Color) -> Bitboard {
        match color {
            Color::Black => self.black_pieces,
            Color::White => self.white_pieces,
        }
    }

    fn get_bitboard(&mut self, color: Color, kind: PieceType) -> &mut Bitboard {
        match (color, kind) {
            (Color::Black, PieceType::Pawn) => &mut self.black_pawns,
            (Color::Black, PieceType::Knight) => &mut self.black_knights,
            (Color::Black, PieceType::Bishop) => &mut self.black_bishops,
            (Color::Black, PieceType::Rook) => &mut self.black_rooks,
            (Color::Black, PieceType::Queen) => &mut self.black_queens,
            (Color::Black, PieceType::King) => &mut self.black_kings,
            (Color::White, PieceType::Pawn) => &mut self.white_pawns,
            (Color::White, PieceType::Knight) => &mut self.white_knights,
            (Color::White, PieceType::Bishop) => &mut self.white_bishops,
            (Color::White, PieceType::Rook) => &mut self.white_rooks,
            (Color::White, PieceType::Queen) => &mut self.white_queens,
            (Color::White, PieceType::King) => &mut self.white_kings,
        }
    }

    pub fn set_piece(&mut self, pos: Position, piece: Piece) -> Option<Piece> {
        let taken = self.take_piece(pos);

        let bb = self.get_bitboard(piece.color(), piece.kind());
        *bb |= pos;
        taken
    }

    pub fn take_piece(&mut self, pos: Position) -> Option<Piece> {
        match (self.get_color_of_pos(pos), self.get_kind_of_pos(pos)) {
            (None, None) => return None,
            (Some(color), Some(kind)) => {
                let taken = Piece::new(color, kind);
                self.remove_known_piece(pos, color, kind);
                return Some(taken);
            }
            _ => panic!(),
        }
    }

    fn remove_known_piece(&mut self, pos: Position, color: Color, kind: PieceType) {
        use Color::*;
        use PieceType::*;
        let pos_bb = Bitboard::with_one(pos);

        self.all_pieces &= !pos_bb;

        match color {
            Black => self.black_pieces &= !pos_bb,
            White => self.white_pieces &= !pos_bb,
        }

        match (color, kind) {
            (Black, Pawn) => {
                self.black_pawns &= !pos_bb;
                self.all_pawns &= !pos_bb;
            }
            (Black, Knight) => {
                self.black_knights &= !pos_bb;
                self.all_knights &= !pos_bb;
            }
            (Black, Bishop) => {
                self.black_bishops &= !pos_bb;
                self.all_bishops &= !pos_bb;
            }
            (Black, Rook) => {
                self.black_rooks &= !pos_bb;
                self.all_rooks &= !pos_bb;
            }
            (Black, Queen) => {
                self.black_queens &= !pos_bb;
                self.all_queens &= !pos_bb;
            }
            (Black, King) => {
                self.black_kings &= !pos_bb;
                self.all_kings &= !pos_bb;
            }
            (White, Pawn) => {
                self.white_pawns &= !pos_bb;
                self.all_pawns &= !pos_bb;
            }
            (White, Knight) => {
                self.white_knights &= !pos_bb;
                self.all_knights &= !pos_bb;
            }
            (White, Bishop) => {
                self.white_bishops &= !pos_bb;
                self.all_bishops &= !pos_bb;
            }
            (White, Rook) => {
                self.white_rooks &= !pos_bb;
                self.all_rooks &= !pos_bb;
            }
            (White, Queen) => {
                self.white_queens &= !pos_bb;
                self.all_queens &= !pos_bb;
            }
            (White, King) => {
                self.white_kings &= !pos_bb;
                self.all_kings &= !pos_bb;
            }
        }
    }

    pub fn has_piece_at(&self, pos: Position) -> bool {
        self.get_color_of_pos(pos).is_some()
    }

    fn get_color_of_pos(&self, pos: Position) -> Option<Color> {
        let white_pieces = self.white_pieces & pos;
        if white_pieces != 0 {
            return Some(Color::White);
        }

        let black_pieces = self.black_pieces & pos;
        if black_pieces != 0 {
            return Some(Color::Black);
        }

        None
    }

    fn get_kind_of_pos(&self, pos: Position) -> Option<PieceType> {
        if self.all_kings & pos != 0 {
            return Some(PieceType::King);
        }

        if self.all_queens & pos != 0 {
            return Some(PieceType::Queen);
        }

        if self.all_rooks & pos != 0 {
            return Some(PieceType::Rook);
        }

        if self.all_knights & pos != 0 {
            return Some(PieceType::Knight);
        }

        if self.all_bishops & pos != 0 {
            return Some(PieceType::Bishop);
        }

        if self.all_pawns & pos != 0 {
            return Some(PieceType::Pawn);
        }

        None
    }

    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        let bit_index = Bitboard::with_one(pos);

        let color = self.get_color_of_pos(pos)?;
        let kind = self.get_kind_of_pos(pos)?;

        Some(Piece::new(color, kind))
    }

    pub fn has_piece_of_color_at(&self, color: Color, pos: Position) -> bool {
        let bb = match color {
            Color::Black => self.black_pieces & pos,
            Color::White => self.white_pieces & pos,
        };
        bb != 0
    }

    pub fn knight_moves(&self, color: Color, pos: Position) -> Bitboard {
        Bitboard::knight_targets(pos)
            & match color {
                Color::Black => !self.black_pieces,
                Color::White => !self.white_pieces,
            }
    }

    pub fn king_moves(&self, color: Color, pos: Position) -> Bitboard {
        Bitboard::king_targets(pos)
            & match color {
                Color::Black => !self.black_pieces,
                Color::White => !self.white_pieces,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_color_of_pos_test() {
        let b = ChessBoard::new();
        assert_eq!(b.get_color_of_pos(E1), Some(Color::White));
        assert_eq!(b.get_color_of_pos(A7), Some(Color::Black));
        assert_eq!(b.get_color_of_pos(A6), None);
    }

    #[test]
    fn get_kind_of_pos_test() {
        let b = ChessBoard::new();
        assert_eq!(b.get_kind_of_pos(E1), Some(PieceType::King));
        assert_eq!(b.get_kind_of_pos(A7), Some(PieceType::Pawn));
        assert_eq!(b.get_kind_of_pos(A6), None);
    }

    #[test]
    fn get_piece_test() {
        let b = ChessBoard::new();
        assert_eq!(
            b.get_piece(E1),
            Some(Piece::new(Color::White, PieceType::King))
        );
        assert_eq!(
            b.get_piece(A7),
            Some(Piece::new(Color::Black, PieceType::Pawn))
        );
        assert_eq!(b.get_piece(A6), None);
    }

    #[test]
    fn knight_moves_test() {
        let b = ChessBoard::new();
        assert_eq!(b.knight_moves(Color::White, A1), Bitboard::with_one(B3));

        assert_eq!(
            b.knight_moves(Color::White, B1),
            Bitboard::with_ones([A3, C3])
        )
    }

    #[test]
    fn king_moves_test() {
        let b = ChessBoard::new();
        assert_eq!(b.king_moves(Color::White, E1), Bitboard::empty());
        assert_eq!(
            b.king_moves(Color::White, E3),
            Bitboard::with_ones([D3, D4, E4, F4, F3])
        );
    }

    #[test]
    fn take_piece_test() {
        let mut b = ChessBoard::new();
        assert_eq!(b.take_piece(E2).unwrap(), Piece::pawn(Color::White));
        assert_eq!(
            b.white_pawns,
            Bitboard::with_ones([A2, B2, C2, D2, F2, G2, H2])
        );
        assert_eq!(
            b.all_pawns,
            Bitboard::with_ones([A2, B2, C2, D2, F2, G2, H2, A7, B7, C7, D7, E7, F7, G7, H7])
        );
        assert_eq!(b.take_piece(A1).unwrap(), Piece::rook(Color::White));
        assert_eq!(b.white_rooks, Bitboard::with_ones([H1]));
        assert_eq!(b.all_rooks, Bitboard::with_ones([H1, A8, H8]));
    }
}
