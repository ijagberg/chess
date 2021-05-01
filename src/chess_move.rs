use crate::{board::Board, game::Game, piece::PieceType, Color, Piece, Position, Rank};
use std::option::Option;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ChessMove {
    Regular {
        from: Position,
        to: Position,
    },
    EnPassant {
        from: Position,
        to: Position,
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub(crate) struct MoveManager {
    history: Vec<(ChessMove, Option<Piece>)>,
    legal_moves: Vec<ChessMove>,
}

impl MoveManager {
    pub(crate) fn new(board: &Board, player: Color) -> Self {
        let mut this = Self {
            history: vec![],
            legal_moves: vec![],
        };

        this.evaluate_legal_moves(board, player);

        this
    }

    pub(crate) fn is_legal(&self, chess_move: ChessMove) -> bool {
        self.legal_moves.contains(&chess_move)
    }

    pub(crate) fn make_move(&mut self, board: &mut Board, chess_move: ChessMove) -> Option<Piece> {
        let taken_piece;
        match chess_move {
            ChessMove::Regular { from, to } => {
                let piece = board.take_piece(from).unwrap();
                let taken = board.set_piece(to, piece);
                self.history.push((chess_move, taken));
                taken_piece = taken;
            }
            ChessMove::EnPassant {
                from,
                to,
                taken_index,
            } => {
                let piece = board.take_piece(from).unwrap();
                board.set_piece(to, piece);
                let taken = board.take_piece(taken_index).unwrap();
                self.history.push((chess_move, Some(taken)));
                taken_piece = Some(taken);
            }
            ChessMove::Promotion { from, to, piece } => {
                todo!()
            }
            ChessMove::Castle {
                rook_from,
                rook_to,
                king_from,
                king_to,
            } => {
                todo!()
            }
        }
        self.legal_moves.clear();
        dbg!("made move", chess_move, taken_piece);
        taken_piece
    }

    pub fn get_legal_moves(&self) -> &Vec<ChessMove> {
        &self.legal_moves
    }

    pub(crate) fn evaluate_legal_moves(&mut self, board: &Board, player: Color) {
        let mut legal_moves = Vec::new();
        for pos in Position::all_iter() {
            let mut legal_moves_from_pos = self.evaluate_legal_moves_from(board, pos, player);
            legal_moves.append(&mut legal_moves_from_pos);
        }

        // TODO: for each move, check if it puts the player in check and remove if so

        dbg!("evaluated legal moves", &player, &legal_moves);

        self.legal_moves = legal_moves;
    }

    fn evaluate_legal_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if let Some(piece) = board[from] {
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
        Vec::new()
    }

    fn evaluate_legal_pawn_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::First || from.rank() == Rank::Eighth {
            // TODO: mark this as an error somehow?
            // There should never be a pawn on the first or eighthranks.
            return Vec::new();
        }
        match player {
            Color::Black => self.evaluate_legal_black_pawn_moves_from(board, from),
            Color::White => self.evaluate_legal_white_pawn_moves_from(board, from),
        }
    }

    fn evaluate_legal_white_pawn_moves_from(
        &self,
        board: &Board,
        from: Position,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::Seventh {
            // from here it's only possible to promote
            todo!()
        } else {
            let mut positions = Vec::new();

            // regular moves, up rank since the pawn is white
            let one_step_forward = from.add_offset(0, 1).unwrap();
            if !board.has_piece_at(one_step_forward) {
                positions.push(one_step_forward);
                if from.rank() == Rank::Second {
                    let two_steps_forward = Position::new(from.file(), Rank::Fourth);
                    if !board.has_piece_at(two_steps_forward) {
                        positions.push(two_steps_forward);
                    }
                }
            }

            if let Some(front_left) = from.add_offset(-1, 1) {
                if board.has_piece_with_color_at(front_left, Color::Black) {
                    positions.push(front_left);
                }
            }

            if let Some(front_right) = from.add_offset(1, 1) {
                if board.has_piece_with_color_at(front_right, Color::Black) {
                    positions.push(front_right);
                }
            }

            if from.rank() == Rank::Fifth {
                // holy hell
            }

            let legal_moves = positions
                .into_iter()
                .map(|to| ChessMove::Regular { from, to })
                .collect();

            legal_moves
        }
    }

    fn evaluate_legal_black_pawn_moves_from(
        &self,
        board: &Board,
        from: Position,
    ) -> Vec<ChessMove> {
        if from.rank() == Rank::Second {
            // from here it's only possible to promote
            todo!()
        } else {
            let mut positions = Vec::new();

            // regular moves, up rank since the pawn is white
            let one_step_forward = from.add_offset(0, -1).unwrap();
            if !board.has_piece_at(one_step_forward) {
                positions.push(one_step_forward);
                if from.rank() == Rank::Seventh {
                    let two_steps_forward = Position::new(from.file(), Rank::Fifth);
                    if !board.has_piece_at(two_steps_forward) {
                        positions.push(two_steps_forward);
                    }
                }
            }

            if let Some(front_left) = from.add_offset(1, -1) {
                if board.has_piece_with_color_at(front_left, Color::White) {
                    positions.push(front_left);
                }
            }

            if let Some(front_right) = from.add_offset(-1, -1) {
                if board.has_piece_with_color_at(front_right, Color::White) {
                    positions.push(front_right);
                }
            }

            if from.rank() == Rank::Fourth {
                // en passant
            }

            let legal_moves = positions
                .into_iter()
                .map(|to| ChessMove::Regular { from, to })
                .collect();

            legal_moves
        }
    }

    fn evaluate_legal_king_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        const KING_OFFSETS: [(i32, i32); 8] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        let mut positions = Vec::new();
        for to in KING_OFFSETS
            .iter()
            .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        {
            match board.get_piece(to) {
                Option::Some(piece) => {
                    if !piece.is_color(player) {
                        positions.push(to);
                    }
                }
                Option::None => {
                    positions.push(to);
                }
            }
        }

        // TODO: castling

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn evaluate_legal_knight_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        const KNIGHT_OFFSETS: [(i32, i32); 8] = [
            (2, 1),
            (2, -1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (1, -2),
            (-1, -2),
        ];
        let mut positions = Vec::new();
        for to in KNIGHT_OFFSETS
            .iter()
            .filter_map(|&(file_step, rank_step)| from.add_offset(file_step, rank_step))
        {
            match board.get_piece(to) {
                Option::Some(piece) => {
                    if !piece.is_color(player) {
                        positions.push(to);
                    }
                }
                Option::None => {
                    positions.push(to);
                }
            }
        }

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn evaluate_legal_bishop_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();

        let mut up_file_up_rank = self.step_until_collision(from, 1, 1, board, player);
        positions.append(&mut up_file_up_rank);

        let mut up_file_down_rank = self.step_until_collision(from, 1, -1, board, player);
        positions.append(&mut up_file_down_rank);

        let mut down_file_up_rank = self.step_until_collision(from, -1, 1, board, player);
        positions.append(&mut down_file_up_rank);

        let mut down_file_down_rank = self.step_until_collision(from, -1, -1, board, player);
        positions.append(&mut down_file_down_rank);

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn evaluate_legal_rook_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();

        let mut up_file = self.step_until_collision(from, 1, 0, board, player);
        positions.append(&mut up_file);

        let mut down_file = self.step_until_collision(from, -1, 0, board, player);
        positions.append(&mut down_file);

        let mut up_rank = self.step_until_collision(from, 0, 1, board, player);
        positions.append(&mut up_rank);

        let mut down_rank = self.step_until_collision(from, 0, -1, board, player);
        positions.append(&mut down_rank);

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn evaluate_legal_queen_moves_from(
        &self,
        board: &Board,
        from: Position,
        player: Color,
    ) -> Vec<ChessMove> {
        let mut positions = Vec::new();

        let mut up_file_up_rank = self.step_until_collision(from, 1, 1, board, player);
        positions.append(&mut up_file_up_rank);

        let mut up_file_down_rank = self.step_until_collision(from, 1, -1, board, player);
        positions.append(&mut up_file_down_rank);

        let mut down_file_up_rank = self.step_until_collision(from, -1, 1, board, player);
        positions.append(&mut down_file_up_rank);

        let mut down_file_down_rank = self.step_until_collision(from, -1, -1, board, player);
        positions.append(&mut down_file_down_rank);

        let mut up_file = self.step_until_collision(from, 1, 0, board, player);
        positions.append(&mut up_file);

        let mut down_file = self.step_until_collision(from, -1, 0, board, player);
        positions.append(&mut down_file);

        let mut up_rank = self.step_until_collision(from, 0, 1, board, player);
        positions.append(&mut up_rank);

        let mut down_rank = self.step_until_collision(from, 0, -1, board, player);
        positions.append(&mut down_rank);

        let legal_moves = positions
            .into_iter()
            .map(|to| ChessMove::Regular { from, to })
            .collect();

        legal_moves
    }

    fn step_until_collision(
        &self,
        start: Position,
        file_step: i32,
        rank_step: i32,
        board: &Board,
        player: Color,
    ) -> Vec<Position> {
        let mut positions = Vec::new();
        for steps in 1.. {
            if let Some(position) = start.add_offset(file_step * steps, rank_step * steps) {
                match board.get_piece(position) {
                    Option::Some(piece) => {
                        if !piece.is_color(player) {
                            positions.push(position);
                        }
                        break;
                    }
                    Option::None => {
                        positions.push(position);
                    }
                }
            } else {
                break;
            }
        }

        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::*;
    use Color::*;

    #[test]
    fn legal_moves() {
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

        let legal_moves: Vec<_> = manager
            .get_legal_moves()
            .iter()
            .map(|m| (m.from(), m.to()))
            .collect();

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
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

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
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

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
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

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
        let board = Board::default();
        let manager = MoveManager::new(&board, White);

        let king_moves_from_a4: Vec<Position> = manager
            .evaluate_legal_king_moves_from(&board, A4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(king_moves_from_a4, vec![A5, B5, B4, B3, A3]);
    }
}
