#[cfg(feature = "fmt")]
pub mod fmt;

use crate::{square::Square, ChessIndex, Color, Piece};
use simple_grid::Grid;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct ChessBoard {
    squares: Grid<Square>,
}

impl ChessBoard {
    pub(crate) fn move_piece(&mut self, from: ChessIndex, to: ChessIndex) {
        if let Some(piece) = self.take_piece(from) {
            self.set_piece(to, piece);
        }
    }

    pub fn piece_at(&self, idx: ChessIndex) -> Option<&Piece> {
        self[idx].piece()
    }

    pub fn set_piece(&mut self, idx: ChessIndex, mut piece: Piece) -> Option<Piece> {
        piece.add_index_to_history(idx);
        if let Some(taken_piece) = self[idx].take_piece() {
            self[idx].set_piece(piece);
            Some(taken_piece)
        } else {
            self[idx].set_piece(piece);
            None
        }
    }

    pub fn take_piece(&mut self, idx: ChessIndex) -> Option<Piece> {
        self[idx].take_piece()
    }
}

impl Index<ChessIndex> for ChessBoard {
    type Output = Square;
    fn index(&self, index: ChessIndex) -> &Self::Output {
        &self.squares[&index]
    }
}

impl IndexMut<ChessIndex> for ChessBoard {
    fn index_mut(&mut self, index: ChessIndex) -> &mut Self::Output {
        &mut self.squares[&index]
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        let squares = vec![
            // rank 1
            Square::empty(Color::Black), // a1
            Square::empty(Color::White), // b1
            Square::empty(Color::Black), // c1
            Square::empty(Color::White), // d1
            Square::empty(Color::Black), // e1
            Square::empty(Color::White), // f1
            Square::empty(Color::Black), // g1
            Square::empty(Color::White), // h1
            // rank 2
            Square::empty(Color::White), // a2
            Square::empty(Color::Black), // b2
            Square::empty(Color::White), // c2
            Square::empty(Color::Black), // d2
            Square::empty(Color::White), // e2
            Square::empty(Color::Black), // f2
            Square::empty(Color::White), // g2
            Square::empty(Color::Black), // h2
            // rank 3
            Square::empty(Color::Black), // a3
            Square::empty(Color::White), // b3
            Square::empty(Color::Black), // c3
            Square::empty(Color::White), // d3
            Square::empty(Color::Black), // e3
            Square::empty(Color::White), // f3
            Square::empty(Color::Black), // g3
            Square::empty(Color::White), // h3
            // rank 4
            Square::empty(Color::White), // a4
            Square::empty(Color::Black), // b4
            Square::empty(Color::White), // c4
            Square::empty(Color::Black), // d4
            Square::empty(Color::White), // e4
            Square::empty(Color::Black), // f4
            Square::empty(Color::White), // g4
            Square::empty(Color::Black), // h4
            // rank 5
            Square::empty(Color::Black), // a5
            Square::empty(Color::White), // b5
            Square::empty(Color::Black), // c5
            Square::empty(Color::White), // d5
            Square::empty(Color::Black), // e5
            Square::empty(Color::White), // f5
            Square::empty(Color::Black), // g5
            Square::empty(Color::White), // h5
            // rank 6
            Square::empty(Color::White), // a6
            Square::empty(Color::Black), // b6
            Square::empty(Color::White), // c6
            Square::empty(Color::Black), // d6
            Square::empty(Color::White), // e6
            Square::empty(Color::Black), // f6
            Square::empty(Color::White), // g6
            Square::empty(Color::Black), // h6
            // rank 7
            Square::empty(Color::Black), // a7
            Square::empty(Color::White), // b7
            Square::empty(Color::Black), // c7
            Square::empty(Color::White), // d7
            Square::empty(Color::Black), // e7
            Square::empty(Color::White), // f7
            Square::empty(Color::Black), // g7
            Square::empty(Color::White), // h7
            // rank 8
            Square::empty(Color::White), // a8
            Square::empty(Color::Black), // b8
            Square::empty(Color::White), // c8
            Square::empty(Color::Black), // d8
            Square::empty(Color::White), // e8
            Square::empty(Color::Black), // f8
            Square::empty(Color::White), // g8
            Square::empty(Color::Black), // h8
        ];

        Self {
            squares: Grid::new(8, 8, squares),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::*;
    use crate::Color::*;

    #[test]
    fn set_piece() {
        let mut board = ChessBoard::default();

        assert!(board.piece_at(E4).is_none());
        assert!(board.piece_at(E5).is_none());

        board.set_piece(E4, Piece::bishop(White));
        assert!(board.piece_at(E4).is_some());

        board.set_piece(E5, Piece::king(White)); // we can place more kings if we wanted to
        assert!(board.piece_at(E5).is_some());
    }

    #[test]
    fn take_piece() {
        let mut board = ChessBoard::default();

        assert!(board.take_piece(E2).is_none());

        board.set_piece(E2, Piece::pawn(Color::White));

        assert!(board.take_piece(E2).unwrap().is_pawn());
    }
}
