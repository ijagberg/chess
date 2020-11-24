use std::{
    convert::TryFrom,
    fmt::Display,
    ops::{Add, Sub},
};

use crate::Color;

/// A chess rank (horizontal line)
#[derive(Debug, Copy, PartialEq, Clone, Eq)]
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
    pub fn is_en_passant_rank(&self, color: Color) -> bool {
        match (self, color) {
            (Rank::Fourth, Color::Black) => true,
            (Rank::Fifth, Color::White) => true,
            _ => false,
        }
    }

    pub fn is_pawn_starting_rank(&self, color: Color) -> bool {
        match (self, color) {
            (Rank::Second, Color::White) => true,
            (Rank::Seventh, Color::Black) => true,
            _ => false,
        }
    }

    pub fn is_pawn_promotion_rank(&self, color: Color) -> bool {
        match (self, color) {
            (Rank::Second, Color::Black) => true,
            (Rank::Seventh, Color::White) => true,
            _ => false,
        }
    }
}

impl Add<u8> for Rank {
    type Output = Option<Rank>;

    fn add(self, rhs: u8) -> Self::Output {
        let mut v = u8::from(&self);

        v += rhs;

        match Rank::try_from(v) {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

impl Sub<u8> for Rank {
    type Output = Option<Rank>;

    fn sub(self, rhs: u8) -> Self::Output {
        let mut v = u8::from(&self);

        v = v.checked_sub(rhs)?;

        match Rank::try_from(v) {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

impl Add<i32> for Rank {
    type Output = Option<Rank>;

    fn add(self, rhs: i32) -> Self::Output {
        if rhs < 0 {
            let rhs = u8::try_from(rhs.abs()).ok()?;
            let result: Option<Rank> = self - rhs;
            result
        } else {
            let rhs = u8::try_from(rhs).ok()?;
            let result: Option<Rank> = self + rhs;
            result
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(self))
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        u8::from(self).partial_cmp(&u8::from(other))
    }
}

impl TryFrom<char> for Rank {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let rank = match value {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => return Err(()),
        };
        Ok(rank)
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let output = match value {
            1 => Rank::First,
            2 => Rank::Second,
            3 => Rank::Third,
            4 => Rank::Fourth,
            5 => Rank::Fifth,
            6 => Rank::Sixth,
            7 => Rank::Seventh,
            8 => Rank::Eighth,
            _ => return Err(()),
        };
        Ok(output)
    }
}

impl TryFrom<i32> for Rank {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v_u8 = u8::try_from(value).map_err(|_| ())?;
        Rank::try_from(v_u8)
    }
}

impl From<&Rank> for u8 {
    fn from(rank: &Rank) -> Self {
        match rank {
            Rank::First => 1,
            Rank::Second => 2,
            Rank::Third => 3,
            Rank::Fourth => 4,
            Rank::Fifth => 5,
            Rank::Sixth => 6,
            Rank::Seventh => 7,
            Rank::Eighth => 8,
        }
    }
}

impl From<&Rank> for i32 {
    fn from(rank: &Rank) -> Self {
        match rank {
            Rank::First => 1,
            Rank::Second => 2,
            Rank::Third => 3,
            Rank::Fourth => 4,
            Rank::Fifth => 5,
            Rank::Sixth => 6,
            Rank::Seventh => 7,
            Rank::Eighth => 8,
        }
    }
}

/// An iterator over ranks
///
/// By default it iterates in increasing order (`First` -> `Second`, `Second` -> `Third`)
pub struct RankIter {
    current: u8,
}

impl RankIter {
    /// Start a new `RankIter` at `start`
    ///
    /// # Example
    /// ```
    /// use game::{Rank, RankIter};
    ///
    /// let mut rank_iter = RankIter::start_at(Rank::First);
    /// assert_eq!(rank_iter.next(), Some(Rank::First));
    /// assert_eq!(rank_iter.next(), Some(Rank::Second));
    ///
    /// let mut rank_iter = RankIter::start_at(Rank::Eighth);
    /// assert_eq!(rank_iter.next(), Some(Rank::Eighth));
    /// assert_eq!(rank_iter.next(), None);
    /// ```
    pub fn start_at(start: Rank) -> Self {
        Self {
            current: u8::from(&start),
        }
    }
}

impl Iterator for RankIter {
    type Item = Rank;
    fn next(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if current_before > 8 {
            None
        } else {
            self.current += 1;
            Some(Rank::try_from(current_before).unwrap())
        }
    }
}

impl DoubleEndedIterator for RankIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if current_before == 0 {
            None
        } else {
            self.current -= 1;
            Some(Rank::try_from(current_before).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_iter() {
        assert_eq!(
            RankIter::start_at(Rank::First).collect::<Vec<_>>(),
            vec![
                Rank::First,
                Rank::Second,
                Rank::Third,
                Rank::Fourth,
                Rank::Fifth,
                Rank::Sixth,
                Rank::Seventh,
                Rank::Eighth,
            ]
        );

        assert_eq!(
            RankIter::start_at(Rank::Third,).collect::<Vec<_>>(),
            vec![
                Rank::Third,
                Rank::Fourth,
                Rank::Fifth,
                Rank::Sixth,
                Rank::Seventh,
                Rank::Eighth,
            ]
        );

        assert_eq!(
            RankIter::start_at(Rank::Fourth).rev().collect::<Vec<_>>(),
            vec![Rank::Fourth, Rank::Third, Rank::Second, Rank::First]
        );
    }

    #[test]
    fn test_add_i32() {
        assert_eq!(Rank::Third + (-1_i32), Some(Rank::Second));
        assert_eq!(Rank::Third + (-2_i32), Some(Rank::First));
        assert_eq!(Rank::Third + (-3_i32), None);
        assert_eq!(Rank::Third + (3_i32), Some(Rank::Sixth));
        assert_eq!(Rank::Third + (20_i32), None);
    }
}
