use crate::{Color, Piece};

#[derive(Debug, Clone)]
pub struct Square {
    color: Color,
    piece: Option<Piece>,
}

impl Square {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn take_piece(&mut self) -> Option<Piece> {
        self.piece.take()
    }

    pub fn piece(&self) -> Option<&Piece> {
        match &self.piece {
            Some(p) => Some(p),
            None => None,
        }
    }

    pub fn piece_mut(&mut self) -> Option<&mut Piece> {
        if let Some(ref mut p) = self.piece {
            Some(p)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.piece = None;
    }

    pub fn empty(color: Color) -> Self {
        Self::new(color, None)
    }

    pub fn occupied(color: Color, piece: Piece) -> Self {
        Self::new(color, Some(piece))
    }

    pub fn new(color: Color, piece: Option<Piece>) -> Self {
        Self { color, piece }
    }

    pub fn set_piece(&mut self, piece: Piece) -> Option<Piece> {
        self.piece.replace(piece)
    }
}
