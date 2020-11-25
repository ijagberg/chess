#![allow(dead_code)]

mod chess_board;
mod chess_index;
mod chess_move;
mod consts;
mod file;
mod piece;
mod rank;
mod square;

pub use chess_board::ChessBoard;
pub use chess_index::*;
pub use chess_move::*;
pub use file::{File, FileIter};
pub use piece::*;
pub use rank::{Rank, RankIter};

use consts::*;
use std::{convert::TryFrom, fmt::Display};

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opponent(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Color::Black => "Black",
            Color::White => "White",
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    board: ChessBoard,
    white_king: ChessIndex,
    black_king: ChessIndex,
    white_taken: Vec<Piece>,
    black_taken: Vec<Piece>,
    history: Vec<ChessBoard>,
    move_history: Vec<ChessMove>,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn board(&self) -> &ChessBoard {
        &self.board
    }

    pub fn white_king(&self) -> ChessIndex {
        self.white_king
    }

    pub fn black_king(&self) -> ChessIndex {
        self.black_king
    }

    pub fn white_taken(&self) -> &Vec<Piece> {
        &self.white_taken
    }

    pub fn black_taken(&self) -> &Vec<Piece> {
        &self.black_taken
    }

    pub fn board_history(&self) -> &Vec<ChessBoard> {
        &self.history
    }

    pub fn move_history(&self) -> &Vec<ChessMove> {
        &self.move_history
    }

    /// Check if a square is in check
    pub fn is_checked(&self, index: ChessIndex, color: Color) -> bool {
        if let Some(_knight_idx) = self.is_checked_by_knight(index, color) {
            return true;
        }

        if let Some(_pawn_idx) = self.is_checked_by_pawn(index, color) {
            return true;
        }

        if let Some(_vert_hori_idx) = self.is_checked_vertical_horizontal(index, color) {
            return true;
        }

        if let Some(_diag_idx) = self.is_checked_diagonal(index, color) {
            return true;
        }

        false
    }

    pub fn is_king_checked(&self, color: Color) -> bool {
        match color {
            Color::Black => self.is_checked(self.black_king, color),
            Color::White => self.is_checked(self.white_king, color),
        }
    }

    fn is_checked_vertical_horizontal(
        &self,
        king_index: ChessIndex,
        king_color: Color,
    ) -> Option<ChessIndex> {
        const HORI_VERT_OFFSETS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for &(file_offset, rank_offset) in &HORI_VERT_OFFSETS {
            if let Some(p) =
                self.find_opponent_piece_along(king_index, file_offset, rank_offset, king_color)
            {
                if p.is_rook() || p.is_queen() || p.is_king() {
                    return Some(*p.history().last().unwrap());
                }
            }
        }

        None
    }

    fn is_checked_diagonal(&self, king_index: ChessIndex, king_color: Color) -> Option<ChessIndex> {
        const DIAGONAL_OFFSETS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        for &(file_offset, rank_offset) in &DIAGONAL_OFFSETS {
            if let Some(p) =
                self.find_opponent_piece_along(king_index, file_offset, rank_offset, king_color)
            {
                if p.is_bishop() || p.is_queen() || p.is_king() {
                    return Some(*p.history().last().unwrap());
                }
            }
        }
        None
    }

    fn is_checked_by_pawn(&self, king_index: ChessIndex, king_color: Color) -> Option<ChessIndex> {
        let opponent_color = king_color.opponent();
        match king_color {
            Color::White => {
                // black pawn on king's rank + 1
                let offsets: Vec<(i32, i32)> = vec![(-1, 1), (1, 1)];

                for (file_offset, rank_offset) in offsets {
                    if let Ok(to_index) = ChessIndex::try_from((
                        u8::from(&king_index.file()) as i32 + file_offset,
                        u8::from(&king_index.rank()) as i32 + rank_offset,
                    )) {
                        if self.board[to_index]
                            .piece()
                            .map(|p| p.is_pawn() && p.color() == opponent_color)
                            .unwrap_or(false)
                        {
                            return Some(to_index);
                        }
                    }
                }
            }
            Color::Black => {
                // white pawn on king's rank - 1
                let offsets = vec![(-1, -1), (1, -1)];

                for (file_offset, rank_offset) in offsets {
                    if let Ok(to_index) = ChessIndex::try_from((
                        u8::from(&king_index.file()) as i32 + file_offset,
                        u8::from(&king_index.rank()) as i32 + rank_offset,
                    )) {
                        if self.board[to_index]
                            .piece()
                            .map(|p| p.is_pawn() && p.color() == opponent_color)
                            .unwrap_or(false)
                        {
                            return Some(to_index);
                        }
                    }
                }
            }
        }

        None
    }

    fn is_checked_by_knight(
        &self,
        king_index: ChessIndex,
        king_color: Color,
    ) -> Option<ChessIndex> {
        const KNIGHT_OFFSETS: [(i32, i32); 8] = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];

        for knight_index in KNIGHT_OFFSETS
            .iter()
            .filter_map(|&(file_offset, rank_offset)| king_index.step(file_offset, rank_offset))
        {
            if self
                .board()
                .piece_at(knight_index)
                .map(|p| p.is_knight() && p.color() == king_color.opponent())
                .unwrap_or(false)
            {
                return Some(knight_index);
            }
        }
        None
    }

    fn find_opponent_piece_along(
        &self,
        start: ChessIndex,
        file_step: i32,
        rank_step: i32,
        our_color: Color,
    ) -> Option<&Piece> {
        self.moves_to_opponents_piece(start, file_step, rank_step, our_color)
            .last()
            .map(|m| self.board.piece_at(m.to_idx()))
            .unwrap_or(None)
    }

    fn undo_last_move(&mut self) {
        if !self.history.is_empty() {
            self.board = self.history.pop().unwrap();
        }
    }

    pub fn valid_moves_from(&self, from_index: ChessIndex) -> Vec<ChessMove> {
        let mut clone = self.clone();

        let piece = match clone.board[from_index].piece() {
            Some(p) => p,
            None => return Vec::new(),
        };
        let piece_color = piece.color();

        let valid_moves = match piece.piece_type() {
            PieceType::Pawn => self.valid_pawn_moves_from(from_index, piece_color),
            PieceType::Knight => self.valid_knight_moves_from(from_index, piece_color),
            PieceType::Bishop => self.valid_bishop_moves_from(from_index, piece_color),
            PieceType::Rook => self.valid_rook_moves_from(from_index, piece_color),
            PieceType::Queen => self.valid_queen_moves_from(from_index, piece_color),
            PieceType::King => self.valid_king_moves_from(from_index, piece_color),
        };

        let mut actual_valid_moves = Vec::new();
        for valid_move in valid_moves {
            clone.execute_move(valid_move);
            if clone.is_king_checked(piece_color) {
                // can't actually make this move
            } else {
                actual_valid_moves.push(valid_move);
            }
            clone.undo_last_move();
        }

        actual_valid_moves
    }

    fn valid_pawn_moves_from(&self, pawn_idx: ChessIndex, pawn_color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        let forward_offset = match pawn_color {
            Color::Black => -1_i32,
            Color::White => 1_i32,
        };

        let forward_idx = match pawn_idx.rank() + forward_offset {
            Some(rank) => ChessIndex::new(pawn_idx.file(), rank),
            None => {
                // we are at the end of the board,
                // this should not happen i think?
                // either way there are no valid pawn moves from here
                return moves;
            }
        };

        let diagonals: Vec<ChessIndex> = vec![forward_idx.file() - 1, forward_idx.file() + 1]
            .into_iter()
            .filter_map(|file: Option<File>| {
                file.map(|file| ChessIndex::new(file, forward_idx.rank()))
            })
            .collect();

        if pawn_idx.rank().is_pawn_promotion_rank(pawn_color) {
            // promotion moves
            if self.board[forward_idx].piece().is_none() {
                // pawn can move forward
                moves.append(&mut ChessMove::promotions(pawn_idx, forward_idx));
            }
            for diagonal_idx in diagonals {
                match self.board[diagonal_idx].piece() {
                    Some(piece) if piece.color() == pawn_color.opponent() => {
                        moves.append(&mut ChessMove::promotions(pawn_idx, diagonal_idx));
                    }
                    _ => {
                        // can't move to the diagonal if it is empty (except en passant) or if we have a piece there
                    }
                }
            }
        } else {
            if self.board[forward_idx].piece().is_none() {
                // pawn can move forward
                moves.push(ChessMove::regular(pawn_idx, forward_idx));
                if pawn_idx.rank().is_pawn_starting_rank(pawn_color) {
                    let forward_forward_idx = ChessIndex::new(
                            pawn_idx.file(),
                            (pawn_idx.rank() + 2 * forward_offset).expect("if the rank is the starting rank, then we should be able to add 2 to it"),
                        );
                    if self.board[forward_forward_idx].piece().is_none() {
                        // can move two steps forward since we haven't made a move yet
                        moves.push(ChessMove::regular(pawn_idx, forward_forward_idx));
                    }
                }
            }
            for diagonal_idx in diagonals {
                match self.board[diagonal_idx].piece() {
                    Some(piece) if piece.color() == pawn_color.opponent() => {
                        moves.push(ChessMove::regular(pawn_idx, diagonal_idx))
                    }
                    _ => {
                        // can't move to the diagonal if it is empty (except en passant) or if we have a piece there
                    }
                }
            }
        }

        // en passant
        if pawn_idx.rank().is_en_passant_rank(pawn_color) {
            // pawn is placed on the rank where en passant can be made
            let left_right: Vec<ChessIndex> = vec![pawn_idx.file() - 1, pawn_idx.file() + 1]
                .into_iter()
                .filter_map(|file: Option<File>| {
                    file.map(|file| ChessIndex::new(file, pawn_idx.rank()))
                })
                .collect();

            for other_idx in left_right {
                if let Some(other_piece) = self.board[other_idx].piece() {
                    if other_piece.is_pawn()
                        && other_piece.color() == pawn_color.opponent()
                        && self.can_en_passant(other_idx)
                    {
                        moves.push(ChessMove::en_passant(
                            pawn_idx,
                            ChessIndex::new(
                                other_idx.file(),
                                (other_idx.rank() + forward_offset).unwrap(),
                            ),
                            other_idx,
                        ));
                    }
                }
            }
        }

        moves
    }

    /// Checks
    fn can_en_passant(&self, pawn_idx: ChessIndex) -> bool {
        if self
            .board
            .piece_at(pawn_idx)
            .map(|p| !p.is_pawn())
            .unwrap_or(true)
        {
            return false;
        }

        match self.move_history.last() {
            Some(ChessMove::Regular(rm)) => {
                rm.to_idx() == pawn_idx
                    && (i32::from(&rm.to_idx().rank()) - i32::from(&rm.from_idx().rank())).abs()
                        == 2
            }
            _ => false,
        }
    }

    fn valid_king_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        for idx in from_index.neighbors() {
            match self.board().piece_at(idx) {
                Some(p) if p.color() == piece_color => {}
                _ => moves.push(ChessMove::regular(from_index, idx)),
            }
        }

        moves
    }

    fn valid_rook_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        // increasing rank
        moves.append(&mut self.moves_to_opponents_piece(from_index, 0, 1, piece_color));

        // decreasing rank
        moves.append(&mut self.moves_to_opponents_piece(from_index, 0, -1, piece_color));

        // increasing file
        moves.append(&mut self.moves_to_opponents_piece(from_index, 1, 0, piece_color));

        // decreasing file
        moves.append(&mut self.moves_to_opponents_piece(from_index, -1, 0, piece_color));

        moves.into_iter().map(ChessMove::Regular).collect()
    }

    fn valid_knight_moves_from(
        &self,
        from_index: ChessIndex,
        piece_color: Color,
    ) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        for to_index in from_index.knight_moves() {
            match self.board[to_index].piece() {
                Some(p) if p.color() == piece_color => {}
                _ => {
                    moves.push(ChessMove::regular(from_index, to_index));
                }
            }
        }
        moves
    }

    fn valid_bishop_moves_from(
        &self,
        from_index: ChessIndex,
        piece_color: Color,
    ) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        // increasing file, increasing rank
        moves.append(&mut self.moves_to_opponents_piece(from_index, 1, 1, piece_color));

        // increasing file, decreasing rank
        moves.append(&mut self.moves_to_opponents_piece(from_index, 1, -1, piece_color));

        // decreasing file, increasing rank
        moves.append(&mut self.moves_to_opponents_piece(from_index, -1, 1, piece_color));

        // decreasing file, decreasing rank
        moves.append(&mut self.moves_to_opponents_piece(from_index, -1, -1, piece_color));

        moves.into_iter().map(ChessMove::Regular).collect()
    }

    fn valid_queen_moves_from(&self, from_index: ChessIndex, piece_color: Color) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        moves.append(&mut self.valid_rook_moves_from(from_index, piece_color));
        moves.append(&mut self.valid_bishop_moves_from(from_index, piece_color));

        moves
    }

    fn add_taken_piece(&mut self, player: Color, piece: Piece) {
        match player {
            Color::Black => {
                self.black_taken.push(piece);
            }
            Color::White => {
                self.white_taken.push(piece);
            }
        }
    }

    pub fn execute_move(&mut self, chess_move: ChessMove) {
        let prev = self.board.clone();

        match chess_move {
            ChessMove::Regular(regular_move) => self.execute_regular_move(regular_move),
            ChessMove::Castle(castle_move) => self.execute_castle_move(castle_move),
            ChessMove::Promotion(promotion_move) => self.execute_promotion_move(promotion_move),
            ChessMove::EnPassant(en_passant_move) => self.execute_en_passant_move(en_passant_move),
        }

        self.move_history.push(chess_move);
        self.history.push(prev);
    }

    fn execute_promotion_move(&mut self, promotion_move: PromotionMove) {
        let pawn = self.board[promotion_move.from_idx()].take_piece().unwrap();

        let taken_piece = self.board[promotion_move.to_idx()].take_piece();

        if let Some(taken_piece) = taken_piece {
            self.add_taken_piece(pawn.color(), taken_piece);
        }

        self.board.set_piece(
            promotion_move.to_idx(),
            Piece::new(promotion_move.promotion_piece(), pawn.color()),
        );
    }

    fn execute_en_passant_move(&mut self, en_passant_move: EnPassantMove) {
        if self.board[en_passant_move.to_idx()].piece().is_some() {
            // invalid move
            panic!();
        }

        let pawn = self.board[en_passant_move.from_idx()].take_piece().unwrap();

        let taken_pawn = self.board[en_passant_move.taken_pawn_idx()]
            .take_piece()
            .unwrap();

        if !taken_pawn.is_pawn() {
            panic!();
        }

        self.add_taken_piece(pawn.color(), taken_pawn);

        self.board.set_piece(en_passant_move.to_idx(), pawn);
    }

    fn execute_castle_move(&mut self, castle_move: CastleMove) {
        let king = self.board[castle_move.king_from()]
            .take_piece()
            .expect("must be a king at from index");
        let rook = self.board[castle_move.rook_from()]
            .take_piece()
            .expect("must be a rook at from index");

        if self.board[castle_move.king_to()].piece().is_some()
            || self.board[castle_move.rook_to()].piece().is_some()
        {
            panic!();
        }

        self.board.set_piece(castle_move.king_to(), king);
        self.board.set_piece(castle_move.rook_to(), rook);
    }

    fn execute_regular_move<T>(&mut self, regular_move: T)
    where
        RegularMove: From<T>,
    {
        let regular_move = RegularMove::from(regular_move);
        let from = regular_move.from_idx();
        let to = regular_move.to_idx();

        let from_piece = self.board[from]
            .take_piece()
            .unwrap_or_else(|| panic!("no piece on from: {}", from));

        let to_piece = self.board[to].take_piece();
        if let Some(piece) = &to_piece {
            if piece.color() == from_piece.color() {
                panic!();
            }
        }

        if let Some(taken_piece) = to_piece {
            self.add_taken_piece(from_piece.color(), taken_piece);
        }

        if from_piece.is_king() {
            match from_piece.color() {
                Color::Black => {
                    self.black_king = to;
                }
                Color::White => {
                    self.white_king = to;
                }
            }
        }
        self.board.set_piece(to, from_piece);
    }

    pub fn is_move_valid(&self, chess_move: ChessMove) -> bool {
        let from_idx = match chess_move {
            ChessMove::Regular(reg) => reg.from_idx(),
            ChessMove::Castle(cas) => cas.king_from(),
            ChessMove::Promotion(prom) => prom.from_idx(),
            ChessMove::EnPassant(en) => en.from_idx(),
        };
        let valid_moves_from = self.valid_moves_from(from_idx);
        valid_moves_from.contains(&chess_move)
    }

    fn can_castle(
        &self,
        king_index: ChessIndex,
        rook_index: ChessIndex,
    ) -> Result<CastleMove, CanCastleError> {
        let (king, rook) = match (
            self.board[king_index].piece(),
            self.board[rook_index].piece(),
        ) {
            (Some(king), Some(rook))
                if king.is_king() && rook.is_rook() && king.color() == rook.color() =>
            {
                (king, rook)
            }
            _ => {
                // either there is no piece at `king_index` or the piece is not a king
                return Err(CanCastleError::WrongPieces);
            }
        };

        let color = king.color();

        if king.has_made_move() {
            return Err(CanCastleError::PieceHasMadeMove(king_index));
        }
        if rook.has_made_move() {
            return Err(CanCastleError::PieceHasMadeMove(rook_index));
        }

        // check that squares between the king and rook are empty and not in check
        let indices_between = ChessIndex::indices_between(king_index, rook_index);
        debug_assert!(
            indices_between.len() == 4 || indices_between.len() == 5,
            format!("{:?}", (indices_between.len(), king_index, rook_index))
        );
        for index_in_between in indices_between {
            if index_in_between != king_index
                && index_in_between != rook_index
                && self.board[index_in_between].piece().is_some()
            {
                // square between the king and rook is not empty
                return Err(CanCastleError::PiecesBetween);
            }
            if self.is_checked(index_in_between, color) {
                return Err(CanCastleError::SquareInCheck(index_in_between));
            }
        }

        let (king_to, rook_to) = if king_index.file() < rook_index.file() {
            (
                ChessIndex::new((king_index.file() + 2).unwrap(), king_index.rank()),
                ChessIndex::new((king_index.file() + 1).unwrap(), rook_index.rank()),
            )
        } else {
            (
                ChessIndex::new((king_index.file() - 2).unwrap(), king_index.rank()),
                ChessIndex::new((king_index.file() - 1).unwrap(), rook_index.rank()),
            )
        };

        Ok(CastleMove::new(king_index, king_to, rook_index, rook_to))
    }

    /// Creates and consumes an iterator which steps by the given `file_step` and `rank_step` arguments until some other piece is reached
    fn moves_to_opponents_piece(
        &self,
        start: ChessIndex,
        file_step: i32,
        rank_step: i32,
        color: Color,
    ) -> Vec<RegularMove> {
        let mut moves = Vec::new();
        for idx in (0..)
            .map(|n| {
                if let (Some(file), Some(rank)) = (
                    File::try_from(i32::from(&start.file()) + n * file_step).ok(),
                    Rank::try_from(i32::from(&start.rank()) + n * rank_step).ok(),
                ) {
                    let idx = ChessIndex::new(file, rank);
                    Some(idx)
                } else {
                    None
                }
            })
            .take_while(|idx| idx.is_some())
            .skip(1)
        {
            let idx = idx.expect("should always be some because we checked `idx.is_some()` above");
            match self.board[idx].piece() {
                Some(p) => {
                    if p.color() == color.opponent() {
                        moves.push(RegularMove::new(start, idx));
                    }
                    break;
                }
                None => {
                    moves.push(RegularMove::new(start, idx));
                }
            }
        }

        moves
    }

    fn score(&self, player: Color) -> u8 {
        match player {
            Color::Black => self.black_taken.iter().map(|p| piece_value(p)).sum(),
            Color::White => self.white_taken.iter().map(|p| piece_value(p)).sum(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum CanCastleError {
    WrongPieces,
    SquareInCheck(ChessIndex),
    PiecesBetween,
    PieceHasMadeMove(ChessIndex),
}

impl Display for CanCastleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            CanCastleError::SquareInCheck(idx) => {
                format!("can't castle because {} is in check", idx)
            }
            CanCastleError::WrongPieces => {
                "can't castle because at least one of the given squares was wrong".to_string()
            }
            CanCastleError::PiecesBetween => {
                "can't castle because there are pieces between the king and rook".to_string()
            }
            CanCastleError::PieceHasMadeMove(idx) => format!(
                "can't castle because the piece at {} has already moved",
                idx
            ),
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum MovePieceError {
    NoPieceToMove(ChessIndex),
    OwnPieceAtTarget,
}

impl Display for MovePieceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            MovePieceError::NoPieceToMove(index) => format!("no piece at {}", index),
            MovePieceError::OwnPieceAtTarget => "can't move to a square you occupy".to_string(),
        };

        write!(f, "{}", output)
    }
}

fn piece_value(p: &Piece) -> u8 {
    match p.piece_type() {
        PieceType::Pawn => 1,
        PieceType::Knight => 3,
        PieceType::Bishop => 3,
        PieceType::Rook => 5,
        PieceType::Queen => 9,
        PieceType::King => 0,
    }
}

impl Default for Game {
    fn default() -> Self {
        use crate::Color::*;

        let mut board = ChessBoard::default();

        board.set_piece(A1, Piece::rook(White));
        board.set_piece(B1, Piece::knight(White));
        board.set_piece(C1, Piece::bishop(White));
        board.set_piece(D1, Piece::queen(White));
        board.set_piece(E1, Piece::king(White));
        board.set_piece(F1, Piece::bishop(White));
        board.set_piece(G1, Piece::knight(White));
        board.set_piece(H1, Piece::rook(White));

        board.set_piece(A2, Piece::pawn(White));
        board.set_piece(B2, Piece::pawn(White));
        board.set_piece(C2, Piece::pawn(White));
        board.set_piece(D2, Piece::pawn(White));
        board.set_piece(E2, Piece::pawn(White));
        board.set_piece(F2, Piece::pawn(White));
        board.set_piece(G2, Piece::pawn(White));
        board.set_piece(H2, Piece::pawn(White));

        board.set_piece(A7, Piece::pawn(Black));
        board.set_piece(B7, Piece::pawn(Black));
        board.set_piece(C7, Piece::pawn(Black));
        board.set_piece(D7, Piece::pawn(Black));
        board.set_piece(E7, Piece::pawn(Black));
        board.set_piece(F7, Piece::pawn(Black));
        board.set_piece(G7, Piece::pawn(Black));
        board.set_piece(H7, Piece::pawn(Black));

        board.set_piece(A8, Piece::rook(Black));
        board.set_piece(B8, Piece::knight(Black));
        board.set_piece(C8, Piece::bishop(Black));
        board.set_piece(D8, Piece::queen(Black));
        board.set_piece(E8, Piece::king(Black));
        board.set_piece(F8, Piece::bishop(Black));
        board.set_piece(G8, Piece::knight(Black));
        board.set_piece(H8, Piece::rook(Black));

        Self {
            board,
            white_king: E1,
            black_king: E7,
            white_taken: Vec::new(),
            black_taken: Vec::new(),
            history: Vec::new(),
            move_history: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use Color::*;

    use super::*;

    #[test]
    fn test_rook_moves() {
        let mut game = Game::new();
        game.execute_regular_move(RegularMove::new(A1, E4));

        print_board("rook in center", &game);

        assert_eq!(
            game.valid_rook_moves_from(E4, White),
            vec![
                ChessMove::regular(E4, E5),
                ChessMove::regular(E4, E6),
                ChessMove::regular(E4, E7),
                ChessMove::regular(E4, E3),
                ChessMove::regular(E4, F4),
                ChessMove::regular(E4, G4),
                ChessMove::regular(E4, H4),
                ChessMove::regular(E4, D4),
                ChessMove::regular(E4, C4),
                ChessMove::regular(E4, B4),
                ChessMove::regular(E4, A4),
            ]
        );
    }

    #[test]
    fn test_knight_moves() {
        let mut game = Game::new();

        game.execute_regular_move((G1, E4));

        print_board("knight in center", &game);

        assert_eq!(
            game.valid_knight_moves_from(E4, White),
            vec![
                ChessMove::regular(E4, G5),
                ChessMove::regular(E4, G3),
                ChessMove::regular(E4, C5),
                ChessMove::regular(E4, C3),
                ChessMove::regular(E4, F6),
                ChessMove::regular(E4, D6),
            ]
        );
    }

    #[test]
    fn test_bishop_moves() {
        let mut game = Game::new();

        game.execute_regular_move((F1, F4));

        print_board("bishop on F4", &game);

        assert_eq!(
            game.valid_bishop_moves_from(F4, White),
            vec![
                ChessMove::regular(F4, G5),
                ChessMove::regular(F4, H6),
                ChessMove::regular(F4, G3),
                ChessMove::regular(F4, E5),
                ChessMove::regular(F4, D6),
                ChessMove::regular(F4, C7),
                ChessMove::regular(F4, E3),
            ]
        );
    }

    #[test]
    fn test_queen_moves() {
        let mut game = Game::new();
        game.execute_regular_move((D1, D4));

        print_board("queen on D4", &game);

        assert_eq!(
            game.valid_queen_moves_from(D4, White),
            vec![
                ChessMove::regular(D4, D5),
                ChessMove::regular(D4, D6),
                ChessMove::regular(D4, D7),
                ChessMove::regular(D4, D3),
                ChessMove::regular(D4, E4),
                ChessMove::regular(D4, F4),
                ChessMove::regular(D4, G4),
                ChessMove::regular(D4, H4),
                ChessMove::regular(D4, C4),
                ChessMove::regular(D4, B4),
                ChessMove::regular(D4, A4),
                ChessMove::regular(D4, E5),
                ChessMove::regular(D4, F6),
                ChessMove::regular(D4, G7),
                ChessMove::regular(D4, E3),
                ChessMove::regular(D4, C5),
                ChessMove::regular(D4, B6),
                ChessMove::regular(D4, A7),
                ChessMove::regular(D4, C3),
            ]
        );
    }

    #[test]
    fn test_king_moves() {
        let mut game = Game::new();

        print_board("initial", &game);

        compare_regular_moves(game.valid_king_moves_from(E1, White), vec![]);

        game.execute_regular_move((E1, E4));

        print_board("king on E4", &game);

        compare_regular_moves(
            game.valid_king_moves_from(E4, White),
            vec![
                ChessMove::regular(E4, F5),
                ChessMove::regular(E4, F3),
                ChessMove::regular(E4, D5),
                ChessMove::regular(E4, D3),
                ChessMove::regular(E4, F4),
                ChessMove::regular(E4, D4),
                ChessMove::regular(E4, E5),
                ChessMove::regular(E4, E3),
            ],
        );

        game.execute_regular_move((F7, F6));
        print_board("-", &game);
        compare_regular_moves(
            game.valid_moves_from(E4),
            vec![
                ChessMove::regular(E4, F5),
                ChessMove::regular(E4, F3),
                ChessMove::regular(E4, D5),
                ChessMove::regular(E4, D3),
                ChessMove::regular(E4, F4),
                ChessMove::regular(E4, D4),
                ChessMove::regular(E4, E3),
            ],
        );
    }

    #[test]
    fn test_is_checked_by_pawn() {
        let mut game = Game::new();

        game.execute_regular_move((E1, E4));

        print_board("king on E4", &game);

        assert_eq!(game.is_checked_by_pawn(E4, White), None);

        game.execute_regular_move((D7, D5));

        print_board("pawn checking king", &game);

        assert_eq!(game.is_checked_by_pawn(E4, White), Some(D5));
    }

    #[test]
    fn test_is_checked_by_knight() {
        let mut game = Game::new();

        game.execute_regular_move((E1, E4));

        print_board("king on E4", &game);

        assert_eq!(game.is_checked_by_knight(E4, White), None);

        game.execute_regular_move((G8, F6));

        print_board("knight checking from F6", &game);

        assert_eq!(game.is_checked_by_knight(E4, White), Some(F6));
    }

    #[test]
    fn test_is_checked_by_bishop() {
        let mut game = Game::new();

        game.execute_regular_move((E1, E4));

        print_board("king on E4", &game);

        assert_eq!(game.is_checked_diagonal(E4, White), None);

        game.execute_regular_move((C8, G6));

        print_board("bishop checking from G6", &game);

        assert_eq!(game.is_checked_diagonal(E4, White), Some(G6));

        game.execute_regular_move((F2, F5));

        print_board("white bishop blocks the check", &game);

        assert_eq!(game.is_checked_diagonal(E4, White), None);
    }

    #[test]
    fn test_is_checked_by_rook() {
        let mut game = Game::new();

        game.execute_regular_move((E1, E4));

        print_board("king on E4", &game);

        assert_eq!(game.is_checked_vertical_horizontal(E4, White), None);

        game.execute_regular_move((H8, H4));

        print_board("rook checking from H4", &game);

        assert_eq!(game.is_checked_vertical_horizontal(E4, White), Some(H4));

        game.execute_regular_move((G2, G4));

        print_board("white pawn blocks the check", &game);

        assert_eq!(game.is_checked_vertical_horizontal(E4, White), None);
    }

    #[test]
    fn test_is_checked_by_queen() {
        let mut game = Game::new();

        game.execute_regular_move((E1, E4));

        print_board("king on E4", &game);

        assert_eq!(game.is_checked(E4, White), false);

        game.execute_regular_move((D8, C6));

        print_board("queen checking from C6", &game);

        assert_eq!(game.is_checked_diagonal(E4, White), Some(C6));

        game.execute_regular_move((C6, B4));

        print_board("queen checking from B4", &game);

        assert_eq!(game.is_checked_vertical_horizontal(E4, White), Some(B4));

        game.execute_regular_move((C1, D4));

        print_board("white bishop blocking check", &game);

        assert_eq!(game.is_checked_vertical_horizontal(E4, White), None);
    }

    #[test]
    fn test_can_castle() {
        let mut game = Game::new();

        assert_eq!(game.can_castle(E1, H1), Err(CanCastleError::PiecesBetween)); // can't castle because there are pieces between
        assert_eq!(game.can_castle(E1, A1), Err(CanCastleError::PiecesBetween)); // can't castle because there are pieces between
        assert_eq!(game.can_castle(E1, F1), Err(CanCastleError::WrongPieces)); // can't castle because F1 is a bishop
        assert_eq!(game.can_castle(D1, H1), Err(CanCastleError::WrongPieces)); // can't castle because D1 is a queen

        // move bishop and knight and pawns out of the way
        game.board[F1].take_piece();
        game.board[G1].take_piece();
        game.board[F2].take_piece();
        game.board[G2].take_piece();

        // move black rook to check square between white king and white rook
        game.execute_regular_move((H8, G6));

        assert_eq!(
            game.can_castle(E1, H1),
            Err(CanCastleError::SquareInCheck(G1))
        ); // can't castle because G1 is in check

        // move black rook away
        game.board[G6].take_piece();

        assert_eq!(game.can_castle(E1, H1), Ok(CastleMove::new(E1, G1, H1, F1))); // can castle now

        // move white rook to G1
        game.execute_regular_move((H1, G1));

        assert_eq!(
            game.can_castle(E1, G1),
            Err(CanCastleError::PieceHasMadeMove(G1))
        ); // rook has moved now
    }

    #[test]
    fn test_execute_castle_move() {
        let mut game = Game::new();

        print_board("initial", &game);

        game.board[F1].clear();
        game.board[G1].clear();

        print_board("cleared", &game);

        let castle_move = game.can_castle(E1, H1).unwrap();

        game.execute_castle_move(castle_move);

        print_board("after", &game);

        assert_eq!(game.board[E1].piece(), None);
        assert_eq!(game.board[H1].piece(), None);

        let rook = game.board[F1].piece().unwrap();
        assert_eq!(rook.history(), &vec![H1, F1]);

        let king = game.board[G1].piece().unwrap();
        assert!(king.is_king());
        assert_eq!(king.history(), &vec![E1, G1]);
    }

    #[test]
    fn test_moves_to_opponents_piece() {
        let mut game = Game::new();

        game.execute_regular_move((D1, E5));

        // increasing rank
        assert_eq!(
            game.moves_to_opponents_piece(E5, 0, 1, White),
            vec![RegularMove::new(E5, E6), RegularMove::new(E5, E7),]
        );

        // decreasing rank
        assert_eq!(
            game.moves_to_opponents_piece(E5, 0, -1, White),
            vec![RegularMove::new(E5, E4), RegularMove::new(E5, E3),]
        );

        // increasing file
        assert_eq!(
            game.moves_to_opponents_piece(E5, 1, 0, White),
            vec![
                RegularMove::new(E5, F5),
                RegularMove::new(E5, G5),
                RegularMove::new(E5, H5),
            ]
        );

        // decreasing file
        assert_eq!(
            game.moves_to_opponents_piece(E5, -1, 0, White),
            vec![
                RegularMove::new(E5, D5),
                RegularMove::new(E5, C5),
                RegularMove::new(E5, B5),
                RegularMove::new(E5, A5),
            ]
        );

        // diagonal
        // increasing rank, increasing file
        assert_eq!(
            game.moves_to_opponents_piece(E5, 1, 1, White),
            vec![RegularMove::new(E5, F6), RegularMove::new(E5, G7),]
        );

        // diagonal
        // increasing rank, decreasing file
        assert_eq!(
            game.moves_to_opponents_piece(E5, -1, 1, White),
            vec![RegularMove::new(E5, D6), RegularMove::new(E5, C7),]
        );

        // diagonal
        // decreasing rank, increasing file
        assert_eq!(
            game.moves_to_opponents_piece(E5, 1, -1, White),
            vec![RegularMove::new(E5, F4), RegularMove::new(E5, G3),]
        );

        // diagonal
        // decreasing rank, decreasing file
        assert_eq!(
            game.moves_to_opponents_piece(E5, -1, -1, White),
            vec![RegularMove::new(E5, D4), RegularMove::new(E5, C3),]
        );
    }

    #[test]
    fn test_valid_pawn_moves_from() {
        let mut game = Game::new();

        print_board("initial", &game);

        assert_eq!(
            game.valid_pawn_moves_from(D2, White),
            vec![ChessMove::regular(D2, D3), ChessMove::regular(D2, D4)]
        );

        game.execute_regular_move((D2, D4));

        print_board("white pawn moved forward", &game);

        assert_eq!(
            game.valid_pawn_moves_from(D4, White),
            vec![ChessMove::regular(D4, D5)]
        );

        game.execute_regular_move((E7, E5));

        print_board("black pawn moved forward", &game);

        assert_eq!(
            game.valid_pawn_moves_from(D4, White),
            vec![ChessMove::regular(D4, D5), ChessMove::regular(D4, E5)]
        );

        game.execute_regular_move((C7, C5));

        print_board("second black pawn moved forward", &game);

        assert_eq!(
            game.valid_pawn_moves_from(D4, White),
            vec![
                ChessMove::regular(D4, D5),
                ChessMove::regular(D4, C5),
                ChessMove::regular(D4, E5),
            ]
        );
    }

    #[test]
    fn test_pawn_promotion() {
        let mut game = Game::new();

        print_board("initial", &game);

        game.board[A7].clear();
        game.execute_regular_move((D2, A7));

        print_board("white pawn in promotion position", &game);

        let actual_valid_moves = game.valid_pawn_moves_from(A7, White);
        assert_eq!(
            actual_valid_moves,
            vec![
                ChessMove::promotion(A7, B8, PieceType::Knight),
                ChessMove::promotion(A7, B8, PieceType::Rook),
                ChessMove::promotion(A7, B8, PieceType::Queen),
                ChessMove::promotion(A7, B8, PieceType::Bishop),
            ]
        );

        game.execute_move(actual_valid_moves[1]);
        print_board("promoted to rook on B8", &game);
        assert_eq!(
            game.board[B8].piece().unwrap().piece_type(),
            PieceType::Rook
        );
    }

    #[test]
    fn test_pawn_promotion_2() {
        let mut game = Game::new();

        game.board = ChessBoard::default();
        game.board.set_piece(E1, Piece::king(White));
        game.board.set_piece(E7, Piece::pawn(White));
        game.board.set_piece(E8, Piece::rook(Black));
        game.board.set_piece(D8, Piece::bishop(Black));
        game.board.set_piece(H8, Piece::king(Black));

        print_board(
            "white pawn can't promote because it would check the white king",
            &game,
        );

        let actual_valid_moves = game.valid_moves_from(E7);
        assert_eq!(actual_valid_moves, vec![]);

        game.board.take_piece(E8).unwrap();

        print_board("white pawn can promote", &game);

        let actual_valid_moves = game.valid_moves_from(E7);

        let mut expected_moves = Vec::new();
        expected_moves.append(&mut ChessMove::promotions(E7, E8));
        expected_moves.append(&mut ChessMove::promotions(E7, D8));
        assert_eq!(actual_valid_moves, expected_moves);
    }

    #[test]
    fn test_en_passant() {
        let mut game = Game::new();

        print_board("initial", &game);

        game.execute_move(ChessMove::regular(D2, D5));
        game.execute_move(ChessMove::regular(E7, E5));

        print_board("black pawn moves two steps", &game);

        let mut valid_moves = game.valid_pawn_moves_from(D5, White);
        assert_eq!(
            valid_moves,
            vec![
                ChessMove::regular(D5, D6),
                ChessMove::en_passant(D5, E6, E5),
            ]
        );

        let en_passant_move = valid_moves.remove(1);

        game.execute_move(en_passant_move);

        print_board("after en passant", &game);
    }

    #[test]
    fn test_can_en_passant() {
        let mut game = Game::new();

        let white_pawn = game.board.take_piece(E2).unwrap();
        game.board.set_piece(E5, white_pawn);
        game.execute_move(ChessMove::regular(F7, F5));

        assert!(game.can_en_passant(F5));

        game.execute_move(ChessMove::regular(A7, A6));

        assert!(!game.can_en_passant(F5));

        let mut game = Game::new();
        let white_pawn = game.board.take_piece(D2).unwrap();
        game.board.set_piece(D5, white_pawn);
        game.execute_move(ChessMove::regular(E7, E5));

        assert!(game.can_en_passant(E5));
    }

    #[test]
    fn test_undo_last_move() {
        let mut game = Game::new();

        print_board("initial", &game);

        game.execute_move(ChessMove::regular(E2, E4));

        print_board("white pawn to E4", &game);

        game.execute_move(ChessMove::regular(E7, E5));

        print_board("black pawn to E5", &game);

        game.undo_last_move();

        print_board("undo last move", &game);

        assert_eq!(game.board[E5].piece(), None);
        assert!(game.board[E7].piece().is_some());
    }

    fn compare_regular_moves(actual: Vec<ChessMove>, expected: Vec<ChessMove>) {
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.into_iter().zip(expected.into_iter()) {
            assert_eq!(actual, expected);
        }
    }

    fn print_board(title: &str, game: &Game) {
        println!("{}:", title);
        println!("{}", game.board);
        println!();
    }
}
