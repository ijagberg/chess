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

    pub fn white(kind: PieceType) -> Self {
        Self::new(Color::White, kind)
    }

    pub fn black(kind: PieceType) -> Self {
        Self::new(Color::Black, kind)
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
