#![allow(unused)]
pub use piece::Piece;
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Debug, Display},
    str::FromStr,
};

mod chess_board;
mod chess_move;
mod game;
mod piece;
pub mod prelude;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opponent(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }

    /// Returns `true` if the color is [`Black`].
    ///
    /// [`Black`]: Color::Black
    #[must_use]
    pub fn is_black(&self) -> bool {
        matches!(self, Self::Black)
    }

    /// Returns `true` if the color is [`White`].
    ///
    /// [`White`]: Color::White
    #[must_use]
    pub fn is_white(&self) -> bool {
        matches!(self, Self::White)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Color::Black => "Black",
            Color::White => "White",
        };
        write!(f, "{}", output)
    }
}
