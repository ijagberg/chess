use std::{collections::HashMap, convert::TryFrom};

use crate::{files, piece::PieceType, ranks, Board, Color, File, Piece, Position, Rank};
use simple_grid::Grid;

pub struct Evaluator {
    pawn_value: i32,
    pawn_table: Grid<i32>,
    knight_value: i32,
    knight_table: Grid<i32>,
    bishop_value: i32,
    bishop_table: Grid<i32>,
    rook_value: i32,
    rook_table: Grid<i32>,
    queen_value: i32,
    queen_table: Grid<i32>,
    king_value: i32,
    king_table: Grid<i32>,
}

impl Evaluator {
    fn new(
        pawn_value: i32,
        pawn_table: Grid<i32>,
        knight_value: i32,
        knight_table: Grid<i32>,
        bishop_value: i32,
        bishop_table: Grid<i32>,
        rook_value: i32,
        rook_table: Grid<i32>,
        queen_value: i32,
        queen_table: Grid<i32>,
        king_value: i32,
        king_table: Grid<i32>,
    ) -> Self {
        Self {
            pawn_value,
            pawn_table,
            knight_value,
            knight_table,
            bishop_value,
            bishop_table,
            rook_value,
            rook_table,
            queen_value,
            queen_table,
            king_value,
            king_table,
        }
    }

    fn evaluate_piece(&self, board: &Board, piece: &Piece, position: Position) -> i32 {
        let mirror_position = position.mirrored();
        let result = match (piece.kind(), piece.color()) {
            (PieceType::Pawn, Color::Black) => self.pawn_table[mirror_position] * self.pawn_value,
            (PieceType::Pawn, Color::White) => self.pawn_table[position] * self.pawn_value,
            (PieceType::Knight, Color::Black) => {
                self.knight_table[mirror_position] * self.knight_value
            }
            (PieceType::Knight, Color::White) => self.knight_table[position] * self.knight_value,
            (PieceType::Bishop, Color::Black) => {
                self.bishop_table[mirror_position] * self.bishop_value
            }
            (PieceType::Bishop, Color::White) => self.bishop_table[position] * self.bishop_value,
            (PieceType::Rook, Color::Black) => self.rook_table[mirror_position] * self.rook_value,
            (PieceType::Rook, Color::White) => self.rook_table[position] * self.rook_value,
            (PieceType::Queen, Color::Black) => {
                self.queen_table[mirror_position] * self.queen_value
            }
            (PieceType::Queen, Color::White) => self.queen_table[position] * self.queen_value,
            (PieceType::King, Color::Black) => self.king_table[mirror_position] * self.king_value,
            (PieceType::King, Color::White) => self.king_table[position] * self.king_value,
        };
        println!(
            "{} on {:?} has value {}",
            piece.speech_string(),
            position,
            result
        );
        result
    }

    pub fn evaluate_chess_board(&self, board: &Board) -> (i32, i32) {
        let mut white_eval: i32 = 0;
        let mut black_eval: i32 = 0;

        for position in Position::all_iter() {
            if let Some(piece) = board.get_piece(position) {
                match piece.color() {
                    Color::Black => black_eval += self.evaluate_piece(board, &piece, position),
                    Color::White => white_eval += self.evaluate_piece(board, &piece, position),
                }
            }
        }

        (white_eval, black_eval)
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        let reverse_vec = |mut v: Vec<i32>| {
            v.reverse();
            v
        };

        Self::new(
            100,
            Grid::new(
                8,
                8,
                reverse_vec(vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20,
                    10, 10, 5, 5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0,
                    -10, -5, 5, 5, 10, 10, -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
                ]),
            ),
            320,
            Grid::new(
                8,
                8,
                reverse_vec(vec![
                    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0,
                    10, 15, 15, 10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15,
                    0, -30, -30, 5, 10, 15, 15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50,
                    -40, -30, -30, -30, -30, -40, -50,
                ]),
            ),
            330,
            Grid::new(
                8,
                8,
                reverse_vec(vec![
                    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5,
                    10, 10, 5, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0,
                    -10, -10, 10, 10, 10, 10, 10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10,
                    -10, -10, -10, -10, -10, -20,
                ]),
            ),
            500,
            Grid::new(
                8,
                8,
                reverse_vec(vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5,
                    -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5,
                    -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 5, 5, 0, 0, 0,
                ]),
            ),
            900,
            Grid::new(
                8,
                8,
                reverse_vec(vec![
                    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5,
                    5, 5, 0, -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5,
                    5, 5, 0, -10, -10, 0, 5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
                ]),
            ),
            20000,
            Grid::new(
                8,
                8,
                reverse_vec(vec![
                    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30,
                    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30,
                    -20, -30, -30, -40, -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10,
                    20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20,
                ]),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_chess_board_test() {
        let board = Board::default();
        let eval = Evaluator::default();

        assert_eq!(eval.evaluate_chess_board(&board), (-35700, -35700));
    }
}
