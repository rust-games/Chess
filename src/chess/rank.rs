use std::str::FromStr;

use crate::{Error, BOARD_SIZE};

/// Describe a rank (row) on a chess board.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
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

/// Numbers of [`Rank`]
pub const NUM_RANKS: usize = BOARD_SIZE.1 as usize;

/// Enumerate all ranks.
pub const ALL_RANKS: [Rank; NUM_RANKS] = [
    Rank::First,
    Rank::Second,
    Rank::Third,
    Rank::Fourth,
    Rank::Fifth,
    Rank::Sixth,
    Rank::Seventh,
    Rank::Eighth,
];

impl Rank {
    /// Gets a [`Rank`] from an integer index.
    ///
    /// > **Note**: If index is not in the range 0..=7, wrap around.
    #[inline]
    pub fn new(index: usize) -> Self {
        ALL_RANKS[index % NUM_RANKS]
    }

    /// Convert this [`Rank`] into a [`usize`] between 0 and 7 (inclusive).
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Go one rank down.
    ///
    /// > **Note**: If impossible, wrap around.
    #[inline]
    pub fn down(&self) -> Self {
        let idx = self.to_index();
        match idx {
            0 => Rank::new(NUM_RANKS - 1),
            _ => Rank::new(idx - 1),
        }
    }

    /// Go one rank up.
    ///
    /// > **Note**: If impossible, wrap around.
    #[inline]
    pub fn up(&self) -> Self {
        Rank::new(self.to_index() + 1)
    }
}

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidRank);
        }
        match s.chars().next().unwrap() {
            '1' => Ok(Rank::First),
            '2' => Ok(Rank::Second),
            '3' => Ok(Rank::Third),
            '4' => Ok(Rank::Fourth),
            '5' => Ok(Rank::Fifth),
            '6' => Ok(Rank::Sixth),
            '7' => Ok(Rank::Seventh),
            '8' => Ok(Rank::Eighth),
            _ => Err(Error::InvalidRank),
        }
    }
}
