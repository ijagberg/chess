use crate::Color;
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
