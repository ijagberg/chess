use crate::Color;

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
