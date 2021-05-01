use crate::{board::Board, game::Game, piece::PieceType::Pawn, Color, Position};
use std::option::Option;

#[derive(Clone, Debug, Copy)]
pub enum ChessMove {
    Regular {
        from: Position,
        to: Position,
    },
    EnPassant {
        from: Position,
        to: Position,
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
    pub(crate) fn to(&self) -> Position {
        *match self {
            ChessMove::Regular { from: _, to } => to,
            ChessMove::EnPassant { from: _, to } => to,
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

#[derive(Debug, Clone, Copy)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub(crate) struct MoveManager;

impl MoveManager {
    pub fn get_legal_moves(&self, game: &Game) -> Vec<ChessMove> {
        let player = game.current_player();
        let board = game.board();

        let mut legal_moves = Vec::new();
        for pos in Position::all_iter() {
            let mut legal_moves_from_pos = self.get_legal_moves_from(board, pos, player);
            legal_moves.append(&mut legal_moves_from_pos);
        }

        legal_moves
    }

    fn get_legal_moves_from(&self, board: &Board, from: Position, player: Color) -> Vec<ChessMove> {
        if let Some(piece) = board[from] {
            if piece.color() == player {
                return match piece.kind() {
                    Pawn => {
                        todo!()
                    }
                    Knight => {
                        todo!()
                    }
                    Bishop => self.get_legal_bishop_moves_from(board, from, player),
                    Rook => self.get_legal_rook_moves_from(board, from, player),
                    Queen => {
                        todo!()
                    }
                    King => {
                        todo!()
                    }
                };
            }
        }
        Vec::new()
    }

    fn get_legal_bishop_moves_from(
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

    fn get_legal_rook_moves_from(
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
    fn bishop_moves() {
        let manager = MoveManager;
        let board = Board::default();

        let bishop_moves_from_f4: Vec<Position> = manager
            .get_legal_bishop_moves_from(&board, F4, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_f4, vec![G5, H6, G3, E5, D6, C7, E3]);

        let bishop_moves_from_c1: Vec<Position> = manager
            .get_legal_bishop_moves_from(&board, C1, White)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(bishop_moves_from_c1, vec![]);
    }

    #[test]
    fn rook_moves() {
        let manager = MoveManager;
        let board = Board::default();

        let rook_moves_from_c5: Vec<Position> = manager
            .get_legal_rook_moves_from(&board, C5, Black)
            .iter()
            .map(|m| m.to())
            .collect();
        assert_eq!(
            rook_moves_from_c5,
            vec![D5, E5, F5, G5, H5, B5, A5, C6, C4, C3, C2]
        );
    }
}
