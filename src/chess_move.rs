use bitboard::{Bitboard, Position, Rank, INCREASING};

use crate::{chess_board::ChessBoard, game::Game, piece::PieceType, Color, Piece};
use std::option::Option;

pub const KNIGHT_OFFSETS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (1, -2),
    (-1, -2),
];

pub const KING_OFFSETS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ChessMove {
    Regular {
        from: Position,
        to: Position,
    },
    EnPassant {
        from: Position,
        to: Position,
        taken_original_index: Position,
        taken_index: Position,
    },
    Promotion {
        from: Position,
        to: Position,
        piece: PromotionPiece,
    },
    Castle {
        rook_from: Position,
        rook_to: Position,
        king_from: Position,
        king_to: Position,
    },
}

impl ChessMove {
    pub(crate) fn from(&self) -> Position {
        *match self {
            ChessMove::Regular { from, to } => from,
            ChessMove::EnPassant {
                from,
                to,
                taken_index,
                taken_original_index,
            } => from,
            ChessMove::Promotion { from, to, piece } => from,
            ChessMove::Castle {
                rook_from,
                rook_to,
                king_from,
                king_to,
            } => king_from,
        }
    }

    pub(crate) fn to(&self) -> Position {
        *match self {
            ChessMove::Regular { from: _, to } => to,
            ChessMove::EnPassant {
                from: _,
                to,
                taken_index,
                taken_original_index,
            } => to,
            ChessMove::Promotion {
                from: _,
                to,
                piece: _,
            } => to,
            ChessMove::Castle {
                rook_from: _,
                rook_to: _,
                king_from: _,
                king_to,
            } => king_to,
        }
    }

    pub(crate) fn promotion_moves(from: Position, to: Position) -> Vec<ChessMove> {
        use PromotionPiece as PP;
        [PP::Bishop, PP::Knight, PP::Rook, PP::Queen]
            .iter()
            .map(|&piece| ChessMove::Promotion { from, to, piece })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotionPiece {
    pub(crate) fn create_piece(&self, color: Color) -> Piece {
        match self {
            PromotionPiece::Knight => Piece::knight(color),
            PromotionPiece::Bishop => Piece::bishop(color),
            PromotionPiece::Rook => Piece::rook(color),
            PromotionPiece::Queen => Piece::queen(color),
        }
    }
}

#[derive(Debug)]
pub(crate) struct MoveManager {
    history: Vec<(Color, ChessMove, Option<Piece>)>,
    legal_moves: Vec<ChessMove>,
    black_king: Position,
    white_king: Position,
}

impl MoveManager {
    pub(crate) fn new(board: &ChessBoard) -> Self {
        let mut white_king = None;
        let mut black_king = None;
        for pos in INCREASING {
            if let Some(Piece {
                color,
                kind: PieceType::King,
            }) = board.get_piece(pos)
            {
                match color {
                    Color::Black => {
                        black_king = Some(pos);
                    }
                    Color::White => {
                        white_king = Some(pos);
                    }
                }
            }
        }
        let mut this = Self {
            history: vec![],
            legal_moves: vec![],
            black_king: black_king.expect("no black king on board"),
            white_king: white_king.expect("no white king on board"),
        };

        this
    }

    pub(crate) fn is_legal(&self, chess_move: ChessMove) -> bool {
        self.legal_moves.contains(&chess_move)
    }

    pub(crate) fn make_move(
        &mut self,
        board: &mut ChessBoard,
        player: Color,
        chess_move: ChessMove,
    ) -> Option<Piece> {
        let taken_piece;
        match chess_move {
            ChessMove::Regular { from, to } => {
                let piece = board.take_piece(from).unwrap();
                let taken = board.set_piece(to, piece);
                print!(" {:?} takes {:?}", piece.kind(), taken);
                if let Piece {
                    color,
                    kind: PieceType::King,
                } = piece
                {
                    match color {
                        Color::Black => {
                            self.black_king = to;
                        }
                        Color::White => {
                            self.white_king = to;
                        }
                    }
                }
                taken_piece = taken;
            }
            ChessMove::EnPassant {
                from,
                to,
                taken_original_index,
                taken_index,
            } => {
                let piece = board.take_piece(from).unwrap();
                board.set_piece(to, piece);
                let taken = board.take_piece(taken_index).unwrap();
                taken_piece = Some(taken);
            }
            ChessMove::Promotion {
                from,
                to,
                piece: promotion,
            } => {
                let piece = board.take_piece(from).unwrap();
                let taken = board.set_piece(to, promotion.create_piece(player));
                taken_piece = taken;
            }
            ChessMove::Castle {
                rook_from,
                rook_to,
                king_from,
                king_to,
            } => {
                // dont forget to update king pos
                let rook = board.take_piece(rook_from).unwrap();
                board.set_piece(rook_to, rook);
                let king = board.take_piece(king_from).unwrap();
                board.set_piece(king_to, king);
                taken_piece = None;
            }
        }
        self.history.push((player, chess_move, taken_piece));
        taken_piece
    }

    pub fn get_legal_moves(&self) -> &Vec<ChessMove> {
        &self.legal_moves
    }

    pub(crate) fn evaluate_legal_moves(&mut self, board: &ChessBoard, player: Color) {
        let mut legal_moves = Vec::with_capacity(60);
        for pos in board.get_occupancy_for_color(player).positions() {
            let mut legal_moves_from_pos = self.evaluate_legal_moves_from(board, pos, player);
            println!("naive legal moves from {pos}: {:?}", legal_moves_from_pos);
            legal_moves.append(&mut legal_moves_from_pos);
        }

        let mut actual_legal_moves = Vec::new();
        let mut board_clone = board.clone();
        for &legal_move in &legal_moves {
            print!("testing move: {}, {:?}", player, legal_move);
            self.make_move(&mut board_clone, player, legal_move);
            if !self.is_in_check(&board_clone, player) {
                println!(" LEGAL");
                actual_legal_moves.push(legal_move);
            } else {
                println!(" ILLEGAL");
            }
            self.undo_last_move(&mut board_clone);
        }

        self.legal_moves = actual_legal_moves;
    }

    fn undo_last_move(&mut self, board: &mut ChessBoard) {
        fn set_option(board: &mut ChessBoard, pos: Position, piece: Option<Piece>) {
            if let Some(piece) = piece {
                board.set_piece(pos, piece);
            }
        }

        if let Some((player, chess_move, taken_piece)) = self.history.pop() {
            println!("undoing move {:?} for player {}", chess_move, player);
            match chess_move {
                ChessMove::Regular { from, to } => {
                    let moved_piece = board.take_piece(to).unwrap();
                    board.set_piece(from, moved_piece);
                    if moved_piece.kind() == PieceType::King {
                        match player {
                            Color::Black => {
                                self.black_king = from;
                            }
                            Color::White => {
                                self.white_king = from;
                            }
                        }
                    }
                    set_option(board, to, taken_piece);
                }
                ChessMove::EnPassant {
                    from,
                    to,
                    taken_index,
                    taken_original_index,
                } => {
                    let moved_pawn = board.take_piece(to).unwrap();
                    board.set_piece(from, moved_pawn);
                    let taken_pawn = taken_piece.unwrap();
                    board.set_piece(taken_original_index, taken_pawn);
                }
                ChessMove::Promotion { from, to, piece } => {
                    let color = match from.rank() {
                        Rank::Two => Color::Black,
                        Rank::Seven => Color::White,
                        _ => panic!(),
                    };
                    board.set_piece(from, Piece::pawn(color));
                    board.take_piece(to);
                    set_option(board, to, taken_piece);
                }
                ChessMove::Castle {
                    rook_from,
                    rook_to,
                    king_from,
                    king_to,
                } => {
                    let king = board.take_piece(king_to).unwrap();
                    let rook = board.take_piece(rook_to).unwrap();
                    board.set_piece(king_from, king);
                    board.set_piece(rook_from, rook);
                }
            }
        }
    }

    fn previous_move(&self) -> Option<ChessMove> {
        self.history
            .last()
            .map(|(_player, chess_move, _taken_piece)| chess_move)
            .copied()
    }

    fn is_in_check(&self, board: &ChessBoard, player: Color) -> bool {
        match player {
            Color::Black => {
                let attackers = self.get_attackers(board, self.black_king, Color::White);
                if attackers > 0 {
                    println!("black king is in check, attackers: {}", attackers);
                    return true;
                }
                false
            }
            Color::White => {
                let attackers = self.get_attackers(board, self.white_king, Color::Black);
                if attackers > 0 {
                    println!("white king is in check, attackers: {}", attackers);
                    return true;
                }
                false
            }
        }
    }

    fn is_under_attack(&self, board: &ChessBoard, target: Position, attacker_color: Color) -> bool {
        use Color::*;
        use PieceType::*;

        self.get_attackers(board, target, attacker_color) > 0
    }

    fn get_attackers(
        &self,
        board: &ChessBoard,
        target: Position,
        attacker_color: Color,
    ) -> Bitboard {
        use Color::*;
        use PieceType::*;

        let mut attacker_bb = Bitboard::empty();
        for piece_type in PieceType::all_iter() {
            attacker_bb |= match (attacker_color, piece_type) {
                (Black, Pawn) => {
                    (Position::up_left(&target)
                        .map(|p| Bitboard::with_one(p))
                        .unwrap_or(Bitboard::empty())
                        | Position::up_right(&target)
                            .map(|p| Bitboard::with_one(p))
                            .unwrap_or(Bitboard::empty()))
                        & board.get_bitboard(Black, Pawn)
                }
                (Black, Knight) => {
                    Bitboard::knight_targets(target) & board.get_bitboard(Black, Knight)
                }
                (Black, Bishop) => {
                    Bitboard::bishop_targets(target, board.full_occupancy())
                        & board.get_bitboard(Black, Bishop)
                }
                (Black, Rook) => {
                    Bitboard::rook_targets(target, board.full_occupancy())
                        & board.get_bitboard(Black, Rook)
                }
                (Black, Queen) => {
                    Bitboard::queen_targets(target, board.full_occupancy())
                        & board.get_bitboard(Black, Queen)
                }
                (Black, King) => Bitboard::king_targets(target) & board.get_bitboard(Black, King),
                (White, Pawn) => {
                    (Position::down_left(&target)
                        .map(|p| Bitboard::with_one(p))
                        .unwrap_or(Bitboard::empty())
                        | Position::down_right(&target)
                            .map(|p| Bitboard::with_one(p))
                            .unwrap_or(Bitboard::empty()))
                        & board.get_bitboard(White, Pawn)
                }
                (White, Knight) => {
                    Bitboard::knight_targets(target) & board.get_bitboard(White, Knight)
                }
                (White, Bishop) => {
                    Bitboard::bishop_targets(target, board.full_occupancy())
                        & board.get_bitboard(White, Bishop)
                }
                (White, Rook) => {
                    Bitboard::rook_targets(target, board.full_occupancy())
                        & board.get_bitboard(White, Rook)
                }
                (White, Queen) => {
                    Bitboard::queen_targets(target, board.full_occupancy())
                        & board.get_bitboard(White, Queen)
                }
                (White, King) => Bitboard::king_targets(target) & board.get_bitboard(White, King),
            }
        }
        attacker_bb
    }

    fn evaluate_legal_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if let Some(piece) = board.get_piece(from) {
            if piece.color() == player {
                return match piece.kind() {
                    PieceType::Pawn => self.evaluate_legal_pawn_moves_from(board, from, player),
                    PieceType::Knight => self.evaluate_legal_knight_moves_from(board, from, player),
                    PieceType::Bishop => self.evaluate_legal_bishop_moves_from(board, from, player),
                    PieceType::Rook => self.evaluate_legal_rook_moves_from(board, from, player),
                    PieceType::Queen => self.evaluate_legal_queen_moves_from(board, from, player),
                    PieceType::King => self.evaluate_legal_king_moves_from(board, from, player),
                };
            }
        }

        println!("no piece on {from}");
        Vec::new()
    }

    fn evaluate_legal_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::One || from.rank() == Rank::Eight {
            // TODO: mark this as an error somehow?
            // There should never be a pawn on the first or eighth ranks.
            return Vec::new();
        }
        match player {
            Color::Black => self.evaluate_legal_black_pawn_moves_from(board, from),
            Color::White => self.evaluate_legal_white_pawn_moves_from(board, from),
        }
    }

    fn evaluate_legal_white_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
    ) -> Vec<ChessMove> {
        let mut legal_moves = Vec::with_capacity(10);
        if from.rank() == Rank::Seven {
            // from here it's only possible to promote

            // check position in front
            let up = from.up().unwrap();
            if !board.has_piece_at(up) {
                legal_moves.append(&mut ChessMove::promotion_moves(from, up));
            }
            if let Some(up_left) = from.up_left() {
                if board.has_piece_of_color_at(Color::Black, up_left) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, up_left));
                }
            }
            if let Some(up_right) = from.up_right() {
                if board.has_piece_of_color_at(Color::Black, up_right) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, up_right));
                }
            }
        } else {
            let targets =
                Bitboard::white_pawn_targets(from, board.get_occupancy_for_color(Color::Black))
                    & !board.get_occupancy_for_color(Color::White);
            for to in targets.positions() {
                legal_moves.push(ChessMove::Regular { from, to });
            }

            // en passant
            if from.rank() == Rank::Five {
                if let Some(ChessMove::Regular { from: f, to: t }) = self.previous_move() {
                    let left_file = from.file().left();
                    let right_file = from.file().right();
                    for file in [left_file, right_file].iter().filter_map(|&f| f) {
                        if f == Position::new(file, Rank::Seven)
                            && t == Position::new(file, Rank::Five)
                        {
                            legal_moves.push(ChessMove::EnPassant {
                                from,
                                to: Position::new(file, Rank::Six),
                                taken_index: Position::new(file, Rank::Five),
                                taken_original_index: Position::new(file, Rank::Seven),
                            });
                        }
                    }
                }
            }
        }
        legal_moves
    }

    fn evaluate_legal_black_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
    ) -> Vec<ChessMove> {
        let mut legal_moves = Vec::with_capacity(10);
        if from.rank() == Rank::Two {
            // from here it's only possible to promote

            // check position in front
            let down = from.down().unwrap();
            if !board.has_piece_at(down) {
                legal_moves.append(&mut ChessMove::promotion_moves(from, down));
            }
            if let Some(down_left) = from.down_left() {
                if board.has_piece_of_color_at(Color::White, down_left) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, down_left));
                }
            }
            if let Some(down_right) = from.down_right() {
                if board.has_piece_of_color_at(Color::White, down_right) {
                    legal_moves.append(&mut ChessMove::promotion_moves(from, down_right));
                }
            }
        } else {
            let targets =
                Bitboard::black_pawn_targets(from, board.get_occupancy_for_color(Color::White))
                    & !board.get_occupancy_for_color(Color::Black);
            for to in targets.positions() {
                legal_moves.push(ChessMove::Regular { from, to });
            }

            // en passant
            if from.rank() == Rank::Four {
                if let Some(ChessMove::Regular { from: f, to: t }) = self.previous_move() {
                    let left_file = from.file().left();
                    let right_file = from.file().right();
                    for file in [left_file, right_file].iter().filter_map(|&f| f) {
                        if f == Position::new(file, Rank::Two)
                            && t == Position::new(file, Rank::Four)
                        {
                            legal_moves.push(ChessMove::EnPassant {
                                from,
                                to: Position::new(file, Rank::Three),
                                taken_index: Position::new(file, Rank::Four),
                                taken_original_index: Position::new(file, Rank::Two),
                            });
                        }
                    }
                }
            }
        }
        legal_moves
    }

    fn evaluate_legal_king_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        use bitboard::{E1, E8};
        let targets = Bitboard::king_targets(from) & !board.get_occupancy_for_color(player);
        let mut legal_moves: Vec<ChessMove> = targets
            .positions()
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        match (player, from) {
            (Color::Black, E8) => {
                if let Some(mut castle_moves) = self.evaluate_black_castle(board) {
                    legal_moves.append(&mut castle_moves);
                }
            }
            (Color::White, E1) => {
                if let Some(mut castle_moves) = self.evaluate_white_castle(board) {
                    legal_moves.append(&mut castle_moves);
                }
            }
            _ => {}
        }
        legal_moves
    }

    fn evaluate_white_castle(&self, board: &ChessBoard) -> Option<Vec<ChessMove>> {
        use bitboard::*;
        // has king moved?
        if self.history_contains_from_position(E1) {
            dbg!("white king has moved");
            return None;
        }

        let mut moves = Vec::with_capacity(2);

        // check short castle
        if !dbg!(self.history_contains_from_position(H1))
            && !dbg!(board.has_piece_at(F1))
            && !dbg!(board.has_piece_at(G1))
            && !dbg!(self.is_under_attack(board, F1, Color::Black))
            && !dbg!(self.is_under_attack(board, G1, Color::Black))
        {
            moves.push(ChessMove::Castle {
                rook_from: H1,
                rook_to: F1,
                king_from: E1,
                king_to: G1,
            });
        }

        // check long castle
        if !self.history_contains_from_position(A1)
            && !board.has_piece_at(D1)
            && !board.has_piece_at(C1)
            && !board.has_piece_at(B1)
            && !self.is_under_attack(board, D1, Color::Black)
            && !self.is_under_attack(board, C1, Color::Black)
            && !self.is_under_attack(board, B1, Color::Black)
        {
            moves.push(ChessMove::Castle {
                rook_from: A1,
                rook_to: D1,
                king_from: E1,
                king_to: C1,
            })
        }

        return Some(moves);
    }

    fn evaluate_black_castle(&self, board: &ChessBoard) -> Option<Vec<ChessMove>> {
        use bitboard::*;

        // has king moved?
        if self.history_contains_from_position(E8) {
            dbg!("black king has moved");
            return None;
        }

        let mut moves = Vec::with_capacity(2);

        // check short castle
        if !self.history_contains_from_position(H8)
            && !board.has_piece_at(F8)
            && !board.has_piece_at(G8)
            && !self.is_under_attack(board, F8, Color::White)
            && !self.is_under_attack(board, G8, Color::White)
        {
            moves.push(ChessMove::Castle {
                rook_from: H8,
                rook_to: F8,
                king_from: E8,
                king_to: G8,
            });
        }

        // check long castle
        if !self.history_contains_from_position(A8)
            && !board.has_piece_at(D8)
            && !board.has_piece_at(C8)
            && !board.has_piece_at(B8)
            && !self.is_under_attack(board, D8, Color::White)
            && !self.is_under_attack(board, C8, Color::White)
            && !self.is_under_attack(board, B8, Color::White)
        {
            moves.push(ChessMove::Castle {
                rook_from: A8,
                rook_to: D8,
                king_from: E8,
                king_to: C8,
            })
        }

        return Some(moves);
    }

    fn evaluate_legal_knight_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::knight_targets(from) & !board.get_occupancy_for_color(player);

        let mut legal_moves = Vec::with_capacity(8);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to });
        }
        legal_moves

        // let mut positions = Vec::new();
        // for to in KNIGHT_OFFSETS
        //     .iter()
        //     .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        // {
        //     match board.get_piece(to) {
        //         Some(piece) => {
        //             if !piece.is_color(player) {
        //                 positions.push(to);
        //             }
        //         }
        //         None => {
        //             positions.push(to);
        //         }
        //     }
        // }

        // let legal_moves = positions
        //     .into_iter()
        //     .map(|to| ChessMove::Regular { from, to })
        //     .collect();

        // legal_moves
    }

    fn evaluate_legal_bishop_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::bishop_targets(from, board.full_occupancy())
            & !board.get_occupancy_for_color(player);
        let mut legal_moves = Vec::with_capacity(16);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to })
        }
        legal_moves
    }

    fn evaluate_legal_rook_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::rook_targets(from, board.full_occupancy())
            & !board.get_occupancy_for_color(player);

        let mut legal_moves = Vec::with_capacity(16);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to })
        }
        legal_moves
        // let mut positions = Vec::new();

        // let mut up_file = self.step_until_collision(from, 1, 0, board, player);
        // positions.append(&mut up_file);

        // let mut down_file = self.step_until_collision(from, -1, 0, board, player);
        // positions.append(&mut down_file);

        // let mut up_rank = self.step_until_collision(from, 0, 1, board, player);
        // positions.append(&mut up_rank);

        // let mut down_rank = self.step_until_collision(from, 0, -1, board, player);
        // positions.append(&mut down_rank);

        // let legal_moves = positions
        //     .into_iter()
        //     .map(|to| ChessMove::Regular { from, to })
        //     .collect();

        // legal_moves
    }

    fn evaluate_legal_queen_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let targets = Bitboard::queen_targets(from, board.full_occupancy())
            & !board.get_occupancy_for_color(player);
        let mut legal_moves = Vec::with_capacity(16);
        for to in targets.positions() {
            legal_moves.push(ChessMove::Regular { from, to })
        }
        legal_moves
    }

    /// Check if the move history contains any move that was made *from* the given index.
    fn history_contains_from_position(&self, from: Position) -> bool {
        self.history
            .iter()
            .any(|(_player, chess_move, _taken_piece)| chess_move.from() == from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitboard::*;
    use Color::*;

    #[test]
    fn legal_moves() {
        let board = ChessBoard::new();
        let mut manager = MoveManager::new(&board);
        manager.evaluate_legal_moves(&board, White);

        dbg!(&manager);

        let legal_moves: Vec<_> = manager
            .get_legal_moves()
            .iter()
            .map(|m| (m.from(), m.to()))
            .collect();

        dbg!(&legal_moves);

        assert_eq!(
            legal_moves,
            vec![
                (B1, A3),
                (B1, C3),
                (G1, F3),
                (G1, H3),
                (A2, A3),
                (A2, A4),
                (B2, B3),
                (B2, B4),
                (C2, C3),
                (C2, C4),
                (D2, D3),
                (D2, D4),
                (E2, E3),
                (E2, E4),
                (F2, F3),
                (F2, F4),
                (G2, G3),
                (G2, G4),
                (H2, H3),
                (H2, H4)
            ]
        );
    }

    #[test]
    fn bishop_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new(&board);

        let bishop_moves_from_f4: Vec<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, F4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_f4, vec![E3, G3, E5, G5, D6, H6, C7]);

        let bishop_moves_from_c1: Vec<Position> = manager
            .evaluate_legal_bishop_moves_from(&board, C1, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_c1, vec![]);
    }

    #[test]
    fn rook_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new(&board);

        let rook_moves_from_c5: Vec<Position> = manager
            .evaluate_legal_rook_moves_from(&board, C5, Black)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            rook_moves_from_c5,
            vec![C2, C3, C4, A5, B5, D5, E5, F5, G5, H5, C6]
        );
    }

    #[test]
    fn knight_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new(&board);

        let knight_moves_from_g4: Vec<Position> = manager
            .evaluate_legal_knight_moves_from(&board, G4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(knight_moves_from_g4, vec![E3, E5, F6, H6]);

        let knight_moves_from_d4: Vec<Position> = manager
            .evaluate_legal_knight_moves_from(&board, D4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(knight_moves_from_d4, vec![B3, F3, B5, F5, C6, E6]);
    }

    #[test]
    fn queen_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new(&board);

        let queen_moves_from_a4: Vec<Position> = manager
            .evaluate_legal_queen_moves_from(&board, A4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            queen_moves_from_a4,
            vec![A3, B3, B4, C4, D4, E4, F4, G4, H4, A5, B5, A6, C6, A7, D7]
        );
    }
}
