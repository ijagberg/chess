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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        Self::construct(
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
        )
    }

    fn construct(
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
    ) -> Self {
        Self {
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
        }
    }

    pub fn clear(&mut self) {
        let empty = Bitboard::empty();

        self.all_pieces = empty;
        self.white_pieces = empty;
        self.black_pieces = empty;

        self.all_kings = empty;
        self.white_kings = empty;
        self.black_kings = empty;

        self.all_queens = empty;
        self.white_queens = empty;
        self.black_queens = empty;

        self.all_rooks = empty;
        self.white_rooks = empty;
        self.black_rooks = empty;

        self.all_bishops = empty;
        self.white_bishops = empty;
        self.black_bishops = empty;

        self.all_knights = empty;
        self.white_knights = empty;
        self.black_knights = empty;

        self.all_pawns = empty;
        self.white_pawns = empty;
        self.black_pawns = empty;
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

    pub fn get_bitboard(&self, color: Color, kind: PieceType) -> Bitboard {
        match (color, kind) {
            (Color::Black, PieceType::Pawn) => self.black_pawns,
            (Color::Black, PieceType::Knight) => self.black_knights,
            (Color::Black, PieceType::Bishop) => self.black_bishops,
            (Color::Black, PieceType::Rook) => self.black_rooks,
            (Color::Black, PieceType::Queen) => self.black_queens,
            (Color::Black, PieceType::King) => self.black_kings,
            (Color::White, PieceType::Pawn) => self.white_pawns,
            (Color::White, PieceType::Knight) => self.white_knights,
            (Color::White, PieceType::Bishop) => self.white_bishops,
            (Color::White, PieceType::Rook) => self.white_rooks,
            (Color::White, PieceType::Queen) => self.white_queens,
            (Color::White, PieceType::King) => self.white_kings,
        }
    }

    fn get_bitboard_mut(&mut self, color: Color, kind: PieceType) -> &mut Bitboard {
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
        use Color::*;
        use PieceType::*;

        let taken = self.take_piece(pos);

        self.all_pieces |= pos;

        match piece.kind() {
            Pawn => self.all_pawns |= pos,
            Knight => self.all_knights |= pos,
            Bishop => self.all_bishops |= pos,
            Rook => self.all_rooks |= pos,
            Queen => self.all_queens |= pos,
            King => self.all_kings |= pos,
        }

        match piece.color() {
            Black => {
                self.black_pieces |= pos;
                match piece.kind() {
                    Pawn => self.black_pawns |= pos,
                    Knight => self.black_knights |= pos,
                    Bishop => self.black_bishops |= pos,
                    Rook => self.black_rooks |= pos,
                    Queen => self.black_queens |= pos,
                    King => self.black_kings |= pos,
                }
            }
            White => {
                self.white_pieces |= pos;
                match piece.kind() {
                    Pawn => self.white_pawns |= pos,
                    Knight => self.white_knights |= pos,
                    Bishop => self.white_bishops |= pos,
                    Rook => self.white_rooks |= pos,
                    Queen => self.white_queens |= pos,
                    King => self.white_kings |= pos,
                }
            }
        }

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
            (color, kind) => panic!("{:?}, {:?}", color, kind),
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

    pub fn from_fen(fen: &str) -> Result<Self, ()> {
        use Color::*;
        use PieceType::*;
        let mut board = ChessBoard::new();
        board.clear();

        let rows: Vec<_> = fen.split('/').collect();
        for (row, rank) in rows.iter().zip(Rank::Eight.down_all()) {
            let mut current_file = File::A;
            for c in row.chars() {
                let pos = Position::new(current_file, rank);
                let bb = Bitboard::with_one(pos);
                if let Some(digit) = c.to_digit(10) {
                    // c empty squares starting at `current_file`
                    let digit = digit as i32;
                    if digit > 8 {
                        return Err(());
                    } else if digit + i32::from(u8::from(current_file)) == 8 {
                        continue;
                    } else {
                        current_file = current_file.add_offset(digit as i32).ok_or(())?;
                    }
                } else {
                    if current_file != File::H {
                        current_file = current_file.right().unwrap();
                    }
                    match c {
                        'p' => {
                            board.black_pawns |= bb;
                            board.all_pawns |= bb;
                            board.black_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'n' => {
                            board.black_knights |= bb;
                            board.all_knights |= bb;
                            board.black_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'b' => {
                            board.black_bishops |= bb;
                            board.all_bishops |= bb;
                            board.black_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'r' => {
                            board.black_rooks |= bb;
                            board.all_rooks |= bb;
                            board.black_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'q' => {
                            board.black_queens |= bb;
                            board.all_queens |= bb;
                            board.black_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'k' => {
                            board.black_kings |= bb;
                            board.all_kings |= bb;
                            board.black_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'P' => {
                            board.white_pawns |= bb;
                            board.all_pawns |= bb;
                            board.white_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'N' => {
                            board.white_knights |= bb;
                            board.all_knights |= bb;
                            board.white_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'B' => {
                            board.white_bishops |= bb;
                            board.all_bishops |= bb;
                            board.white_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'R' => {
                            board.white_rooks |= bb;
                            board.all_rooks |= bb;
                            board.white_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'Q' => {
                            board.white_queens |= bb;
                            board.all_queens |= bb;
                            board.white_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        'K' => {
                            board.white_kings |= bb;
                            board.all_kings |= bb;
                            board.white_pieces |= bb;
                            board.all_pieces |= bb;
                        }
                        _ => return Err(()),
                    }
                }
            }
        }
        Ok(board)
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

    #[test]
    fn from_fen_test() {
        use PieceType::*;

        let board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
        assert_eq!(board, ChessBoard::new());

        let board = ChessBoard::from_fen("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8").unwrap();
        for pos in INCREASING_A1_B1 {
            let piece = board.get_piece(pos);
            if [A4, B5, D6, E5, F4, H5].contains(&pos) {
                assert_eq!(
                    piece.unwrap(),
                    Piece::black(Pawn),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else if [A3, B4, D5, E4, F3, H4].contains(&pos) {
                assert_eq!(
                    piece.unwrap(),
                    Piece::white(Pawn),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else if pos == F7 {
                assert_eq!(
                    piece.unwrap(),
                    Piece::black(King),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else if pos == H3 {
                assert_eq!(
                    piece.unwrap(),
                    Piece::white(King),
                    "pos: {}, piece: {:?}",
                    pos,
                    piece
                );
            } else {
                assert!(piece.is_none(), "pos: {}, piece: {:?}", pos, piece);
            }
        }
        // assert_eq!(board, ChessBoard::new());
    }
}
