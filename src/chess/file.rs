use std::str::FromStr;

use crate::{Error, BOARD_SIZE};

/// Describe a file (column) on a chess board.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
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

/// Numbers of [`File`]
pub const NUM_FILES: usize = BOARD_SIZE.1 as usize;

/// Enumerate all files.
pub const ALL_FILES: [File; NUM_FILES] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl File {
    /// Gets a [`File`] from an integer index.
    ///
    /// > **Note**: If index is not in the range 0..=7, wrap around.
    #[inline]
    pub fn new(index: usize) -> Self {
        ALL_FILES[index % NUM_FILES]
    }

    /// Convert this [`File`] into a [`usize`] between 0 and 7 inclusive.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Go one file to the left.
    ///
    /// > **Note**: If impossible, wrap around.
    #[inline]
    pub fn left(&self) -> Self {
        File::new(self.to_index().wrapping_sub(1))
    }

    /// Go one file to the right.
    ///
    /// > **Note**: If impossible, wrap around.
    #[inline]
    pub fn right(&self) -> Self {
        File::new(self.to_index() + 1)
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidFile);
        }
        match s.chars().next().unwrap() {
            'a' | 'A' => Ok(File::A),
            'b' | 'B' => Ok(File::B),
            'c' | 'C' => Ok(File::C),
            'd' | 'D' => Ok(File::D),
            'e' | 'E' => Ok(File::E),
            'f' | 'F' => Ok(File::F),
            'g' | 'G' => Ok(File::G),
            'h' | 'H' => Ok(File::H),
            _ => Err(Error::InvalidFile),
        }
    }
}
