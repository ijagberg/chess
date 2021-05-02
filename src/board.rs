use crate::{consts::*, Color, File, Piece, Position, Rank};
use simple_grid::Grid;
use std::{
    collections::HashSet,
    convert::TryFrom,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid<Option<Piece>>,
}

impl Board {
    fn new(grid: Grid<Option<Piece>>) -> Self {
        Self { grid }
    }

    /// Returns the `Piece` at `pos`, or `None` if there is no piece there.
    ///
    /// ## Example
    /// ```rust
    /// # use chess::prelude::*;
    /// let board = Board::default();
    /// assert_eq!(board.get_piece(E1).unwrap(), Piece::new(Color::White, PieceType::King));
    /// ```
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        self[pos]
    }

    pub fn set_piece(&mut self, pos: Position, piece: Piece) -> Option<Piece> {
        self[pos].replace(piece)
    }

    pub fn take_piece(&mut self, pos: Position) -> Option<Piece> {
        self[pos].take()
    }

    /// Returns `true` if there is a piece at `pos`.
    ///
    /// ## Example
    /// ```rust
    /// # use chess::{consts::*, Board, Color};
    /// let board = Board::default();
    /// assert!(board.has_piece_at(E2));
    /// ```
    pub fn has_piece_at(&self, pos: Position) -> bool {
        self.get_piece(pos).is_some()
    }

    pub(crate) fn has_piece_with_color_at(&self, pos: Position, color: Color) -> bool {
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

pub(crate) fn get_perspective(board: &Board, highlighted_squares: &HashSet<Position>) -> String {
    let mut lines = Vec::new();

    for rank in (1..=8).rev().map(|r| Rank::try_from(r).unwrap()) {
        let mut pieces = Vec::new();
        for file in (1..=8).map(|f| File::try_from(f).unwrap()) {
            let index = Position::new(file, rank);
            let highlight = match highlighted_squares.contains(&index) {
                true => "X",
                false => " ",
            };
            let piece = match board[index] {
                Some(p) => format!("{}", p),
                None => " ".to_string(),
            };
            let output = format!("{}{} ", highlight, piece);

            pieces.push(output);
        }

        let mut line = format!("{}│", rank);
        line.push_str(&pieces.join("│"));
        line.push_str("│\n");

        lines.push(line);
    }

    let mut output = String::new();
    output.push_str(" ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
    output.push_str(&lines.join(" ├───┼───┼───┼───┼───┼───┼───┼───┤\n"));
    output.push_str(" └───┴───┴───┴───┴───┴───┴───┴───┘");
    output
}
