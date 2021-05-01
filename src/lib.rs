pub use board::Board;
pub use piece::Piece;
use simple_grid::{Grid, GridIndex};
use std::{
    convert::{TryFrom, TryInto},
    ops::{Index, IndexMut},
};

mod board;
mod chess_move;
mod consts;
mod game;
mod piece;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    pub(crate) fn add_offset(&self, offset: i32) -> Option<Self> {
        let v = u32::from(*self);
        match offset.is_negative() {
            true => v.checked_sub(offset.abs() as u32)?.try_into().ok(),
            false => v.checked_add(offset as u32)?.try_into().ok(),
        }
    }
}

impl TryFrom<u32> for Rank {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Rank::*;
        Ok(match value {
            1 => First,
            2 => Second,
            3 => Third,
            4 => Fourth,
            5 => Fifth,
            6 => Sixth,
            7 => Seventh,
            8 => Eighth,
            _ => return Err(()),
        })
    }
}

impl From<Rank> for u32 {
    fn from(rank: Rank) -> Self {
        use Rank::*;
        match rank {
            First => 1,
            Second => 2,
            Third => 3,
            Fourth => 4,
            Fifth => 5,
            Sixth => 6,
            Seventh => 7,
            Eighth => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub(crate) fn add_offset(&self, offset: i32) -> Option<Self> {
        let v = u32::from(*self);
        match offset.is_negative() {
            true => v.checked_sub(offset.abs() as u32)?.try_into().ok(),
            false => v.checked_add(offset as u32)?.try_into().ok(),
        }
    }
}

impl TryFrom<u32> for File {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use File::*;
        Ok(match value {
            1 => A,
            2 => B,
            3 => C,
            4 => D,
            5 => E,
            6 => F,
            7 => G,
            8 => H,
            _ => return Err(()),
        })
    }
}

impl From<File> for u32 {
    fn from(file: File) -> Self {
        use File::*;
        match file {
            A => 1,
            B => 2,
            C => 3,
            D => 4,
            E => 5,
            F => 6,
            G => 7,
            H => 8,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(File, Rank);

impl Position {
    pub fn new(file: File, rank: Rank) -> Self {
        Self(file, rank)
    }

    pub fn file(&self) -> File {
        self.0
    }

    pub fn rank(&self) -> Rank {
        self.1
    }

    pub(crate) fn add_offset(&self, file_step: i32, rank_step: i32) -> Option<Self> {
        Some(Self::new(
            self.file().add_offset(file_step)?,
            self.rank().add_offset(rank_step)?,
        ))
    }

    pub fn all_iter() -> impl Iterator<Item = Self> {
        consts::increasing_order()
    }
}

impl From<Position> for GridIndex {
    fn from(pos: Position) -> Self {
        let column = match pos.file() {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        };
        let row = match pos.rank() {
            Rank::First => 0,
            Rank::Second => 1,
            Rank::Third => 2,
            Rank::Fourth => 3,
            Rank::Fifth => 4,
            Rank::Sixth => 5,
            Rank::Seventh => 6,
            Rank::Eighth => 7,
        };

        Self::new(column, row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_offset() {
        let file = File::E;
        assert_eq!(file.add_offset(-2), Some(File::C));

        let rank = Rank::Second;
        assert_eq!(rank.add_offset(5), Some(Rank::Seventh));

        let pos = Position::new(File::E, Rank::Second);
        assert_eq!(
            pos.add_offset(-2, 5),
            Some(Position::new(File::C, Rank::Seventh))
        );
    }
}
