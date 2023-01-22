use bitboard::{Position, Rank, INCREASING};

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
        todo!()
        // let mut legal_moves = Vec::new();
        // for pos in INCREASING {
        //     let mut legal_moves_from_pos = self.evaluate_legal_moves_from(board, pos, player);
        //     legal_moves.append(&mut legal_moves_from_pos);
        // }

        // let mut actual_legal_moves = Vec::new();
        // let mut board_clone = board.clone();
        // for &legal_move in &legal_moves {
        //     print!("testing move: {}, {:?}", player, legal_move);
        //     self.make_move(&mut board_clone, player, legal_move);
        //     if !self.is_in_check(&board_clone, player) {
        //         println!(" LEGAL");
        //         actual_legal_moves.push(legal_move);
        //     } else {
        //         println!(" ILLEGAL");
        //     }
        //     self.undo_last_move(&mut board_clone);
        // }

        // self.legal_moves = actual_legal_moves;
    }

    fn undo_last_move(&mut self, board: &mut ChessBoard) {
        fn set_option(board: &mut ChessBoard, pos: Position, piece: Option<Piece>) {
            if let Some(piece) = piece {
                board.set_piece(pos, piece);
            }
        }

        if let Some((player, chess_move, taken_piece)) = self.history.pop() {
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
                if let Some((pos, piece)) =
                    self.is_under_attack(board, self.black_king, Color::Black)
                {
                    println!("black is in check by {:?} on {:?}", piece, pos);
                    return true;
                }
                false
            }
            Color::White => {
                if let Some((pos, piece)) =
                    self.is_under_attack(board, self.white_king, Color::White)
                {
                    println!("white is in check by {:?} on {:?}", piece, pos);
                    return true;
                }
                false
            }
        }
    }

    fn is_under_attack(
        &self,
        board: &ChessBoard,
        target: Position,
        color: Color,
    ) -> Option<(Position, Piece)> {
        todo!()
    }

    fn evaluate_legal_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        todo!()
        // if let Some(piece) = board.get_piece(from) {
        //     if piece.color() == player {
        //         return match piece.kind() {
        //             PieceType::Pawn => self.evaluate_legal_pawn_moves_from(board, from, player),
        //             PieceType::Knight => self.evaluate_legal_knight_moves_from(board, from, player),
        //             PieceType::Bishop => self.evaluate_legal_bishop_moves_from(board, from, player),
        //             PieceType::Rook => self.evaluate_legal_rook_moves_from(board, from, player),
        //             PieceType::Queen => self.evaluate_legal_queen_moves_from(board, from, player),
        //             PieceType::King => self.evaluate_legal_king_moves_from(board, from, player),
        //         };
        //     }
        // }
        // Vec::new()
    }

    fn evaluate_legal_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        todo!()
        // if from.rank() == Rank::One || from.rank() == Rank::Eight {
        //     // TODO: mark this as an error somehow?
        //     // There should never be a pawn on the first or eighthranks.
        //     return Vec::new();
        // }
        // match player {
        //     Color::Black => self.evaluate_legal_black_pawn_moves_from(board, from),
        //     Color::White => self.evaluate_legal_white_pawn_moves_from(board, from),
        // }
    }

    fn evaluate_legal_white_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
    ) -> Vec<ChessMove> {
        todo!()
        // if from.rank() == Rank::Seven {
        //     // from here it's only possible to promote

        //     let mut moves = Vec::new();

        //     // check position in front
        //     let to = Position::new(from.file(), Rank::Eight);
        //     if !board.has_piece_at(to) {
        //         moves.append(&mut ChessMove::promotion_moves(from, to));
        //     }

        //     for to in [(-1, 1), (1, 1)]
        //         .iter()
        //         .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        //     {
        //         if board.has_piece_with_color_at(to, Color::Black) {
        //             moves.append(&mut ChessMove::promotion_moves(from, to));
        //         }
        //     }

        //     return moves;
        // } else {
        //     let mut positions = Vec::new();

        //     // regular moves, up rank since the pawn is white
        //     let one_step_forward = from.add_offset(0, 1).unwrap();
        //     if !board.has_piece_at(one_step_forward) {
        //         positions.push(one_step_forward);
        //         if from.rank() == Rank::Second {
        //             let two_steps_forward = Position::new(from.file(), Rank::Fourth);
        //             if !board.has_piece_at(two_steps_forward) {
        //                 positions.push(two_steps_forward);
        //             }
        //         }
        //     }

        //     if let Some(front_left) = from.add_offset(-1, 1) {
        //         if board.has_piece_with_color_at(front_left, Color::Black) {
        //             positions.push(front_left);
        //         }
        //     }

        //     if let Some(front_right) = from.add_offset(1, 1) {
        //         if board.has_piece_with_color_at(front_right, Color::Black) {
        //             positions.push(front_right);
        //         }
        //     }

        //     let mut legal_moves: Vec<ChessMove> = positions
        //         .into_iter()
        //         .map(|to| ChessMove::Regular { from, to })
        //         .collect();

        //     if from.rank() == Rank::Fifth {
        //         if let Some(ChessMove::Regular { from: f, to: t }) = self.previous_move() {
        //             let left_file = from.file().add_offset(-1);
        //             let right_file = from.file().add_offset(1);
        //             for file in [left_file, right_file].iter().filter_map(|&f| f) {
        //                 if f == Position::new(file, Rank::Seventh)
        //                     && t == Position::new(file, Rank::Fifth)
        //                 {
        //                     legal_moves.push(ChessMove::EnPassant {
        //                         from,
        //                         to: Position::new(file, Rank::Sixth),
        //                         taken_index: Position::new(file, Rank::Fifth),
        //                         taken_original_index: Position::new(file, Rank::Seventh),
        //                     });
        //                 }
        //             }
        //         }
        //     }

        //     legal_moves
        // }
    }

    fn evaluate_legal_black_pawn_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
    ) -> Vec<ChessMove> {
        todo!()
        // if from.rank() == Rank::Second {
        //     // from here it's only possible to promote

        //     let mut moves = Vec::new();

        //     // check position in front
        //     let to = Position::new(from.file(), Rank::First);
        //     if !board.has_piece_at(to) {
        //         moves.append(&mut ChessMove::promotion_moves(from, to));
        //     }

        //     for to in [(-1, -1), (1, -1)]
        //         .iter()
        //         .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        //     {
        //         if board.has_piece_with_color_at(to, Color::White) {
        //             moves.append(&mut ChessMove::promotion_moves(from, to));
        //         }
        //     }

        //     return moves;
        // } else {
        //     let mut positions = Vec::new();

        //     // regular moves, up rank since the pawn is white
        //     let one_step_forward = from.add_offset(0, -1).unwrap();
        //     if !board.has_piece_at(one_step_forward) {
        //         positions.push(one_step_forward);
        //         if from.rank() == Rank::Seventh {
        //             let two_steps_forward = Position::new(from.file(), Rank::Fifth);
        //             if !board.has_piece_at(two_steps_forward) {
        //                 positions.push(two_steps_forward);
        //             }
        //         }
        //     }

        //     if let Some(front_left) = from.add_offset(1, -1) {
        //         if board.has_piece_with_color_at(front_left, Color::White) {
        //             positions.push(front_left);
        //         }
        //     }

        //     if let Some(front_right) = from.add_offset(-1, -1) {
        //         if board.has_piece_with_color_at(front_right, Color::White) {
        //             positions.push(front_right);
        //         }
        //     }

        //     let mut legal_moves: Vec<ChessMove> = positions
        //         .into_iter()
        //         .map(|to| ChessMove::Regular { from, to })
        //         .collect();

        //     if from.rank() == Rank::Fourth {
        //         if let Some(ChessMove::Regular { from: f, to: t }) = self.previous_move() {
        //             let left_file = from.file().add_offset(-1);
        //             let right_file = from.file().add_offset(1);
        //             for file in [left_file, right_file].iter().filter_map(|&f| f) {
        //                 if f == Position::new(file, Rank::Second)
        //                     && t == Position::new(file, Rank::Fourth)
        //                 {
        //                     legal_moves.push(ChessMove::EnPassant {
        //                         from,
        //                         to: Position::new(file, Rank::Third),
        //                         taken_index: Position::new(file, Rank::Fourth),
        //                         taken_original_index: Position::new(file, Rank::Second),
        //                     });
        //                 }
        //             }
        //         }
        //     }

        //     legal_moves
        // }
    }

    fn evaluate_legal_king_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        todo!()
        // let mut positions = Vec::new();
        // for to in KING_OFFSETS
        //     .iter()
        //     .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        // {
        //     match board.get_piece(to) {
        //         Option::Some(piece) => {
        //             if !piece.is_color(player) {
        //                 positions.push(to);
        //             }
        //         }
        //         Option::None => {
        //             positions.push(to);
        //         }
        //     }
        // }

        // let mut legal_moves: Vec<ChessMove> = positions
        //     .into_iter()
        //     .map(|to| ChessMove::Regular { from, to })
        //     .collect();

        // match (player, from) {
        //     (Color::Black, E8) => legal_moves.append(&mut self.evaluate_black_castle(board)),
        //     (Color::White, E1) => legal_moves.append(&mut self.evaluate_white_castle(board)),
        //     _ => {}
        // }

        // legal_moves
    }

    fn evaluate_white_castle(&self, board: &ChessBoard) -> Vec<ChessMove> {
        todo!()
        // // has king moved?
        // if self.history_contains_from_position(E1) {
        //     return Vec::new();
        // }

        // let mut moves = Vec::new();

        // // check short castle
        // if !self.history_contains_from_position(H1)
        //     && !board.has_piece_at(F1)
        //     && !board.has_piece_at(G1)
        //     && !self.is_under_attack(board, F1, Color::White).is_some()
        //     && !self.is_under_attack(board, G1, Color::White).is_some()
        // {
        //     moves.push(ChessMove::Castle {
        //         rook_from: H1,
        //         rook_to: F1,
        //         king_from: E1,
        //         king_to: G1,
        //     });
        // }

        // // check long castle
        // if !self.history_contains_from_position(A1)
        //     && !board.has_piece_at(D1)
        //     && !board.has_piece_at(C1)
        //     && !board.has_piece_at(B1)
        //     && !self.is_under_attack(board, D1, Color::White).is_some()
        //     && !self.is_under_attack(board, C1, Color::White).is_some()
        //     && !self.is_under_attack(board, B1, Color::White).is_some()
        // {
        //     moves.push(ChessMove::Castle {
        //         rook_from: A1,
        //         rook_to: D1,
        //         king_from: E1,
        //         king_to: C1,
        //     })
        // }

        // return moves;
    }

    fn evaluate_black_castle(&self, board: &ChessBoard) -> Vec<ChessMove> {
        todo!()
        // // has king moved?
        // if self.history_contains_from_position(E8) {
        //     return Vec::new();
        // }

        // let mut moves = Vec::new();

        // // check short castle
        // if !self.history_contains_from_position(H8)
        //     && !board.has_piece_at(F8)
        //     && !board.has_piece_at(G8)
        //     && !self.is_under_attack(board, F8, Color::Black).is_some()
        //     && !self.is_under_attack(board, G8, Color::Black).is_some()
        // {
        //     moves.push(ChessMove::Castle {
        //         rook_from: H8,
        //         rook_to: F8,
        //         king_from: E8,
        //         king_to: G8,
        //     });
        // }

        // // check long castle
        // if !self.history_contains_from_position(A8)
        //     && !board.has_piece_at(D8)
        //     && !board.has_piece_at(C8)
        //     && !board.has_piece_at(B8)
        //     && !self.is_under_attack(board, D8, Color::Black).is_some()
        //     && !self.is_under_attack(board, C8, Color::Black).is_some()
        //     && !self.is_under_attack(board, B8, Color::Black).is_some()
        // {
        //     moves.push(ChessMove::Castle {
        //         rook_from: A8,
        //         rook_to: D8,
        //         king_from: E8,
        //         king_to: C8,
        //     })
        // }

        // return moves;
    }

    fn evaluate_legal_knight_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        todo!()
        
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
        todo!()
        // let mut positions = Vec::new();

        // let mut up_file_up_rank = self.step_until_collision(from, 1, 1, board, player);
        // positions.append(&mut up_file_up_rank);

        // let mut up_file_down_rank = self.step_until_collision(from, 1, -1, board, player);
        // positions.append(&mut up_file_down_rank);

        // let mut down_file_up_rank = self.step_until_collision(from, -1, 1, board, player);
        // positions.append(&mut down_file_up_rank);

        // let mut down_file_down_rank = self.step_until_collision(from, -1, -1, board, player);
        // positions.append(&mut down_file_down_rank);

        // let legal_moves = positions
        //     .into_iter()
        //     .map(|to| ChessMove::Regular { from, to })
        //     .collect();

        // legal_moves
    }

    fn evaluate_legal_rook_moves_from(
        &self,
        board: &ChessBoard,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        todo!()
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
        todo!()
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
                (A2, A3),
                (A2, A4),
                (B1, C3),
                (B1, A3),
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
                (G1, H3),
                (G1, F3),
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
        assert_eq!(bishop_moves_from_f4, vec![G5, H6, G3, E5, D6, C7, E3]);

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
            vec![D5, E5, F5, G5, H5, B5, A5, C6, C4, C3, C2]
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
        assert_eq!(knight_moves_from_g4, vec![H6, F6, E5, E3]);

        let knight_moves_from_d4: Vec<Position> = manager
            .evaluate_legal_knight_moves_from(&board, D4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(knight_moves_from_d4, vec![F5, F3, E6, C6, B5, B3]);
    }

    #[test]
    fn queen_moves() {
        let board = ChessBoard::new();
        let manager = MoveManager::new(&board);

        let king_moves_from_a4: Vec<Position> = manager
            .evaluate_legal_king_moves_from(&board, A4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(king_moves_from_a4, vec![A5, B5, B4, B3, A3]);
    }
}
