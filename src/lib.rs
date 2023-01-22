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

#[derive(Clone, Copy, Debug, PartialEq)]
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
