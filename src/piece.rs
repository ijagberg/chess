use crate::Color;
use std::fmt::Display;
use PieceType::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub(crate) color: Color,
    pub(crate) kind: PieceType,
}

impl Piece {
    pub fn new(color: Color, kind: PieceType) -> Self {
        Self { color, kind }
    }

    pub fn kind(&self) -> PieceType {
        self.kind
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn is_color(&self, color: Color) -> bool {
        self.color() == color
    }

    pub fn pawn(color: Color) -> Self {
        Self::new(color, Pawn)
    }

    pub fn bishop(color: Color) -> Self {
        Self::new(color, Bishop)
    }

    pub fn knight(color: Color) -> Self {
        Self::new(color, Knight)
    }

    pub fn rook(color: Color) -> Self {
        Self::new(color, Rook)
    }

    pub fn queen(color: Color) -> Self {
        Self::new(color, Queen)
    }

    pub fn king(color: Color) -> Self {
        Self::new(color, King)
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match (&self.color(), &self.kind()) {
            (Color::Black, PieceType::Pawn) => "♟︎",
            (Color::Black, PieceType::Knight) => "♞",
            (Color::Black, PieceType::Bishop) => "♝",
            (Color::Black, PieceType::Rook) => "♜",
            (Color::Black, PieceType::Queen) => "♛",
            (Color::Black, PieceType::King) => "♚",
            (Color::White, PieceType::Pawn) => "♙",
            (Color::White, PieceType::Knight) => "♘",
            (Color::White, PieceType::Bishop) => "♗",
            (Color::White, PieceType::Rook) => "♖",
            (Color::White, PieceType::Queen) => "♕",
            (Color::White, PieceType::King) => "♔",
        };
        write!(f, "{}", output)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    /// Returns `true` if the piece_type is [`Pawn`].
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn)
    }

    /// Returns `true` if the piece_type is [`Knight`].
    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight)
    }

    /// Returns `true` if the piece_type is [`Bishop`].
    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop)
    }

    /// Returns `true` if the piece_type is [`Rook`].
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook)
    }

    /// Returns `true` if the piece_type is [`Queen`].
    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen)
    }

    /// Returns `true` if the piece_type is [`King`].
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King)
    }
}

pub(crate) mod constructors {
    use crate::piece::PieceType::{self, *};
    use crate::Color::{self, *};
    use crate::Piece;

    fn p(color: Color, kind: PieceType) -> Piece {
        Piece::new(color, kind)
    }

    pub fn white_pawn() -> Piece {
        p(White, Pawn)
    }

    pub fn black_pawn() -> Piece {
        p(Black, Pawn)
    }

    pub fn white_knight() -> Piece {
        p(White, Knight)
    }

    pub fn black_knight() -> Piece {
        p(Black, Knight)
    }

    pub fn white_bishop() -> Piece {
        p(White, Bishop)
    }

    pub fn black_bishop() -> Piece {
        p(Black, Bishop)
    }

    pub fn white_rook() -> Piece {
        p(White, Rook)
    }

    pub fn black_rook() -> Piece {
        p(Black, Rook)
    }

    pub fn white_queen() -> Piece {
        p(White, Queen)
    }

    pub fn black_queen() -> Piece {
        p(Black, Queen)
    }

    pub fn white_king() -> Piece {
        p(White, King)
    }

    pub fn black_king() -> Piece {
        p(Black, King)
    }
}
