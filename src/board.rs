use crate::{consts::*, Color, Piece, Position};
use simple_grid::Grid;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid<Option<Piece>>,
}

impl Board {
    fn new(grid: Grid<Option<Piece>>) -> Self {
        Self { grid }
    }

    pub fn get_piece(&self, pos: Position) -> &Option<Piece> {
        &self[pos]
    }

    pub fn set_piece(&mut self, pos: Position, piece: Piece) {
        self[pos] = Some(piece);
    }

    pub fn has_piece_at(&self, pos: Position) -> bool {
        self.get_piece(pos).is_some()
    }

    pub fn has_piece_with_color_at(&self, pos: Position, color: Color) -> bool {
        self.get_piece(pos)
            .map(|p| p.color() == color)
            .unwrap_or(false)
    }
}

impl Default for Board {
    fn default() -> Self {
        use crate::piece::PieceType::*;
        use crate::Color::*;

        let grid = Grid::new(8, 8, vec![None; 64]);

        let mut board = Self::new(grid);

        // white pieces
        board.set_piece(A1, Piece::new(White, Rook));
        board.set_piece(B1, Piece::new(White, Knight));
        board.set_piece(C1, Piece::new(White, Bishop));
        board.set_piece(D1, Piece::new(White, Queen));
        board.set_piece(E1, Piece::new(White, King));
        board.set_piece(F1, Piece::new(White, Bishop));
        board.set_piece(G1, Piece::new(White, Knight));
        board.set_piece(H1, Piece::new(White, Rook));
        board.set_piece(A2, Piece::new(White, Pawn));
        board.set_piece(B2, Piece::new(White, Pawn));
        board.set_piece(C2, Piece::new(White, Pawn));
        board.set_piece(D2, Piece::new(White, Pawn));
        board.set_piece(E2, Piece::new(White, Pawn));
        board.set_piece(F2, Piece::new(White, Pawn));
        board.set_piece(G2, Piece::new(White, Pawn));
        board.set_piece(H2, Piece::new(White, Pawn));

        // black pieces
        board.set_piece(A8, Piece::new(Black, Rook));
        board.set_piece(B8, Piece::new(Black, Knight));
        board.set_piece(C8, Piece::new(Black, Bishop));
        board.set_piece(D8, Piece::new(Black, Queen));
        board.set_piece(E8, Piece::new(Black, King));
        board.set_piece(F8, Piece::new(Black, Bishop));
        board.set_piece(G8, Piece::new(Black, Knight));
        board.set_piece(H8, Piece::new(Black, Rook));
        board.set_piece(A7, Piece::new(Black, Pawn));
        board.set_piece(B7, Piece::new(Black, Pawn));
        board.set_piece(C7, Piece::new(Black, Pawn));
        board.set_piece(D7, Piece::new(Black, Pawn));
        board.set_piece(E7, Piece::new(Black, Pawn));
        board.set_piece(F7, Piece::new(Black, Pawn));
        board.set_piece(G7, Piece::new(Black, Pawn));
        board.set_piece(H7, Piece::new(Black, Pawn));

        board
    }
}

impl Index<Position> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.grid[index]
    }
}
