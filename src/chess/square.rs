use std::fmt;
use std::str::FromStr;

use crate::{Error, File, Rank, GRID_SIZE, NUM_FILES, NUM_RANKS};

#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

/// How many files are there?
pub const NUM_SQUARES: usize = (GRID_SIZE.0 * GRID_SIZE.1) as usize;

/// Enumerate all files
#[rustfmt::skip]
pub const ALL_SQUARES: [Square; NUM_SQUARES] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
];

impl Square {
    /// Create a new square, from an index.
    /// > Note: It is invalid, but allowed, to pass in a number >= 64. Doing so will crash stuff.
    ///
    /// ```
    /// use chess::Square;
    ///
    /// assert_eq!(Square::new(0), Square::A1);
    /// assert_eq!(Square::new(63), Square::H8);
    /// ```
    #[inline]
    pub fn new(index: usize) -> Self {
        ALL_SQUARES[index % NUM_SQUARES]
    }

    /// Convert this `Square` into a `usize` from 0 to 63 inclusive.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Make a square from file and rank
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::{File, Rank, Square};
    ///
    /// // Make the A1 square
    /// let square = Square::make_square(Rank::First, File::A);
    /// ```
    #[inline]
    pub fn make_square(file: File, rank: Rank) -> Square {
        Square::new(file.to_index() + rank.to_index() * GRID_SIZE.0 as usize)
    }

    /// Return the file of this square.
    ///
    /// ```
    /// use chess::{File, Rank, Square};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.get_file(), File::D);
    /// ```
    #[inline]
    pub fn file(&self) -> File {
        File::new(self.to_index() % NUM_FILES)
    }

    /// Return the rank of this square.
    ///
    /// ```
    /// use chess::{File, Rank, Square};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.get_rank(), Rank::Seventh);
    /// ```
    #[inline]
    pub fn rank(&self) -> Rank {
        Rank::new(self.to_index() / NUM_RANKS)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (('a' as u8) + (self.file() as u8)) as char,
            (('1' as u8) + (self.rank() as u8)) as char
        )
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::InvalidSquare);
        }
        let ch: Vec<char> = s.chars().collect();
        match ch[0] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => {}
            _ => {
                return Err(Error::InvalidSquare);
            }
        }
        match ch[1] {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {}
            _ => {
                return Err(Error::InvalidSquare);
            }
        }
        Ok(Square::make_square(
            File::new((ch[0] as usize) - ('a' as usize)),
            Rank::new((ch[1] as usize) - ('1' as usize)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        for file in (0..8).map(File::new) {
            for rank in (0..8).map(Rank::new) {
                let square = Square::make_square(file, rank);
                assert_eq!(square.file(), file);
                assert_eq!(square.rank(), rank);
            }
        }
    }
}
