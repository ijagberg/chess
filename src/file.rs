use std::{
    convert::TryFrom,
    fmt::Display,
    ops::{Add, Sub},
};

/// A chess file (vertical line)
#[derive(Debug, Copy, PartialEq, Clone, Eq)]
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

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        u8::from(self).cmp(&u8::from(other))
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Add<u8> for File {
    type Output = Option<File>;

    fn add(self, rhs: u8) -> Self::Output {
        let mut v = u8::from(&self);

        v += rhs;

        match File::try_from(v) {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

impl Sub<u8> for File {
    type Output = Option<File>;

    fn sub(self, rhs: u8) -> Self::Output {
        let mut v = u8::from(&self);

        v = v.checked_sub(rhs)?;

        match File::try_from(v) {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = char::from(self);
        write!(f, "{}", output)
    }
}

impl From<&File> for char {
    fn from(file: &File) -> Self {
        match file {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

impl TryFrom<char> for File {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let file = match value {
            'a' | 'A' => File::A,
            'b' | 'B' => File::B,
            'c' | 'C' => File::C,
            'd' | 'D' => File::D,
            'e' | 'E' => File::E,
            'f' | 'F' => File::F,
            'g' | 'G' => File::G,
            'h' | 'H' => File::H,
            _ => return Err(()),
        };

        Ok(file)
    }
}

impl TryFrom<u8> for File {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let output = match value {
            1 => File::A,
            2 => File::B,
            3 => File::C,
            4 => File::D,
            5 => File::E,
            6 => File::F,
            7 => File::G,
            8 => File::H,
            _ => return Err(()),
        };
        Ok(output)
    }
}

impl TryFrom<i32> for File {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v_u8 = u8::try_from(value).map_err(|_| ())?;
        File::try_from(v_u8)
    }
}

impl From<&File> for u8 {
    fn from(file: &File) -> Self {
        match file {
            File::A => 1,
            File::B => 2,
            File::C => 3,
            File::D => 4,
            File::E => 5,
            File::F => 6,
            File::G => 7,
            File::H => 8,
        }
    }
}

impl From<&File> for i32 {
    fn from(file: &File) -> Self {
        match file {
            File::A => 1,
            File::B => 2,
            File::C => 3,
            File::D => 4,
            File::E => 5,
            File::F => 6,
            File::G => 7,
            File::H => 8,
        }
    }
}

pub struct FileIter {
    current: u8,
}

impl FileIter {
    /// Start a new `FileIter` at `start`
    ///
    /// # Example
    /// ```
    /// use game::{File, FileIter};
    ///
    /// let mut file_iter = FileIter::start_at(File::A);
    /// assert_eq!(file_iter.next(), Some(File::A));
    /// assert_eq!(file_iter.next(), Some(File::B));
    ///
    /// let mut file_iter = FileIter::start_at(File::H);
    /// assert_eq!(file_iter.next(), Some(File::H));
    /// assert_eq!(file_iter.next(), None);
    /// ```
    pub fn start_at(start: File) -> Self {
        Self {
            current: u8::from(&start),
        }
    }
}

impl Iterator for FileIter {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if self.current > 8 {
            None
        } else {
            self.current += 1;
            Some(File::try_from(current_before).unwrap())
        }
    }
}

impl DoubleEndedIterator for FileIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if current_before == 0 {
            None
        } else {
            self.current -= 1;
            Some(File::try_from(current_before).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_iter() {
        assert_eq!(
            FileIter::start_at(File::A).collect::<Vec<_>>(),
            vec![
                File::A,
                File::B,
                File::C,
                File::D,
                File::E,
                File::F,
                File::G,
                File::H,
            ]
        );

        assert_eq!(
            FileIter::start_at(File::C).collect::<Vec<_>>(),
            vec![File::C, File::D, File::E, File::F, File::G, File::H]
        );

        assert_eq!(
            FileIter::start_at(File::D).rev().collect::<Vec<_>>(),
            vec![File::D, File::C, File::B, File::A]
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(File::A + 1, Some(File::B));
        assert_eq!(File::A + 2, Some(File::C));
        assert_eq!(File::A + 10, None);
    }

    #[test]
    fn test_sub() {
        assert_eq!(File::A - 1, None);
        assert_eq!(File::B - 1, Some(File::A));
    }
}
