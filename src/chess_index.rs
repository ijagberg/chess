use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display},
    str::FromStr,
};

use simple_grid::GridIndex;

use crate::{File, FileIter, Rank, RankIter};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ChessIndex(pub(crate) File, pub(crate) Rank);

impl ChessIndex {
    pub fn new(file: File, rank: Rank) -> Self {
        Self(file, rank)
    }

    pub(crate) fn linear_value(&self) -> usize {
        (8 * (u8::from(&self.rank()) - 1) + (u8::from(&self.file()) - 1)) as usize
    }

    pub fn rank(&self) -> Rank {
        self.1
    }

    pub fn file(&self) -> File {
        self.0
    }

    pub fn step(&self, file_offset: i32, rank_offset: i32) -> Option<Self> {
        if let (Some(file), Some(rank)) = (
            File::try_from(i32::from(&self.file()) + file_offset).ok(),
            Rank::try_from(i32::from(&self.rank()) + rank_offset).ok(),
        ) {
            Some(ChessIndex::new(file, rank))
        } else {
            None
        }
    }

    pub fn diagonals(&self) -> Vec<Self> {
        const DIAGONAL_OFFSETS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        DIAGONAL_OFFSETS
            .iter()
            .filter_map(|&(file_offset, rank_offset)| self.step(file_offset, rank_offset))
            .collect()
    }

    pub fn cardinal_dirs(&self) -> Vec<Self> {
        const CARDINAL_OFFSETS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        CARDINAL_OFFSETS
            .iter()
            .filter_map(|&(file_offset, rank_offset)| self.step(file_offset, rank_offset))
            .collect()
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let mut diags = self.diagonals();
        diags.append(&mut self.cardinal_dirs());
        diags
    }

    pub fn knight_moves(&self) -> Vec<Self> {
        const KNIGHT_OFFSETS: [(i32, i32); 8] = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];
        KNIGHT_OFFSETS
            .iter()
            .filter_map(|&(file_offset, rank_offset)| self.step(file_offset, rank_offset))
            .collect()
    }

    pub fn indices_between<T>(from: T, to: T) -> Vec<ChessIndex>
    where
        ChessIndex: From<T>,
    {
        let from: ChessIndex = from.into();
        let to: ChessIndex = to.into();

        if from.file() == to.file() {
            let file = from.file();
            // iterate horizontally
            if from.rank() <= to.rank() {
                RankIter::start_at(from.rank())
                    .take_while(|r| r <= &to.rank())
                    .map(|r| ChessIndex::new(file, r))
                    .collect()
            } else {
                RankIter::start_at(from.rank())
                    .rev()
                    .take_while(|r| r >= &to.rank())
                    .map(|r| ChessIndex::new(file, r))
                    .collect()
            }
        } else if from.rank() == to.rank() {
            let rank = from.rank();
            // iterate vertically
            if from.file() <= to.file() {
                FileIter::start_at(from.file())
                    .take_while(|f| f <= &to.file())
                    .map(|f| ChessIndex::new(f, rank))
                    .collect()
            } else {
                FileIter::start_at(from.file())
                    .rev()
                    .take_while(|f| f >= &to.file())
                    .map(|f| ChessIndex::new(f, rank))
                    .collect()
            }
        } else {
            vec![]
        }
    }
}

impl From<(File, Rank)> for ChessIndex {
    fn from((file, rank): (File, Rank)) -> Self {
        ChessIndex::new(file, rank)
    }
}

/// So that we can index into simple_grid::Grid using a ChessIndex
impl From<&ChessIndex> for GridIndex {
    fn from(ci: &ChessIndex) -> Self {
        GridIndex::new(u8::from(&ci.1) as usize - 1, u8::from(&ci.0) as usize - 1)
    }
}

impl TryFrom<(i32, i32)> for ChessIndex {
    type Error = ();
    fn try_from((file, rank): (i32, i32)) -> Result<Self, Self::Error> {
        let file = u8::try_from(file).map_err(|_| ())?;
        let rank = u8::try_from(rank).map_err(|_| ())?;
        match (File::try_from(file), Rank::try_from(rank)) {
            (Ok(f), Ok(r)) => Ok(ChessIndex::new(f, r)),
            _ => Err(()),
        }
    }
}

impl FromStr for ChessIndex {
    type Err = ParseChessIndexError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseChessIndexError::LengthNot2);
        }

        let file_char = s.as_bytes()[0] as char;
        let file =
            File::try_from(file_char).map_err(|_| ParseChessIndexError::InvalidFile(file_char))?;

        let rank_char = s.as_bytes()[1] as char;
        let rank =
            Rank::try_from(rank_char).map_err(|_| ParseChessIndexError::InvalidRank(rank_char))?;

        Ok(ChessIndex::from((file, rank)))
    }
}

impl Display for ChessIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl Debug for ChessIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseChessIndexError {
    LengthNot2,
    InvalidFile(char),
    InvalidRank(char),
}

impl Display for ParseChessIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseChessIndexError::LengthNot2 => {
                "format should be 'xy', x: file, y: rank".to_string()
            }
            ParseChessIndexError::InvalidFile(file) => format!("invalid file: '{}'", file),
            ParseChessIndexError::InvalidRank(rank) => format!("invalid rank: '{}'", rank),
        };

        write!(f, "{}", output)
    }
}

impl Error for ParseChessIndexError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::*;

    #[test]
    fn from_str() {
        assert_eq!(ChessIndex::from_str("A1").unwrap(), A1);
        assert_eq!(ChessIndex::from_str("H5").unwrap(), H5);
        assert_eq!(
            ChessIndex::from_str("11"),
            Err(ParseChessIndexError::InvalidFile('1'))
        );
        assert_eq!(
            ChessIndex::from_str("AA"),
            Err(ParseChessIndexError::InvalidRank('A'))
        );
        assert_eq!(
            ChessIndex::from_str("A2 "),
            Err(ParseChessIndexError::LengthNot2)
        );
    }

    #[test]
    fn indices_between() {
        assert_eq!(ChessIndex::indices_between(E4, E7), vec![E4, E5, E6, E7]);
        assert_eq!(ChessIndex::indices_between(E7, E4), vec![E7, E6, E5, E4]);
        assert_eq!(ChessIndex::indices_between(E4, F3), vec![]);
        assert_eq!(ChessIndex::indices_between(A1, D1), vec![A1, B1, C1, D1]);
        assert_eq!(
            ChessIndex::indices_between(E1, A1),
            vec![E1, D1, C1, B1, A1]
        );
    }

    #[test]
    fn knight_moves() {
        assert_eq!(E4.knight_moves(), vec![G5, G3, C5, C3, F6, F2, D6, D2]);
        assert_eq!(H4.knight_moves(), vec![F5, F3, G6, G2]);
        assert_eq!(A1.knight_moves(), vec![C2, B3]);
    }

    #[test]
    fn neighbors() {
        assert_eq!(D5.neighbors(), vec![E6, E4, C6, C4, E5, C5, D6, D4]);
        assert_eq!(C1.neighbors(), vec![D2, B2, D1, B1, C2]);
        assert_eq!(A1.neighbors(), vec![B2, B1, A2]);
    }

    #[test]
    fn step() {
        assert_eq!(A1.step(1, 3), Some(B4));
        assert_eq!(A1.step(10, 2), None);
    }
}
