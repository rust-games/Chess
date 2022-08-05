use std::fmt;
use std::str::FromStr;
use super::Error;
use super::Color;

/// Describe position in the chess board.
///
///  H  |   |   |   |   |   |   |   |   |
///  G  |   |   |   |   |   |   |   |   |
///  F  |   |   |   |   |   |   |   |   |
///  E  |   |   |   |   |   |   |   |   |
///  D  |   |   |   |   |   |   |   |   |
///  C  |   |   |   |   |   |   |   |   |
///  B  |   |   |   |   |   |   |   |   |
///  A  |   |   |   |   |   |   |   |   |
///       1   2   3   4   5   6   7   8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessPosition {
    x: u8,
    y: u8,
}

impl ChessPosition {
    pub fn new(x: u8, y: u8) -> Self {
        ChessPosition { x, y }
    }

    /// Return the rank (ie. the row).
    #[inline]
    pub fn get_rank(&self) -> u8 {
        self.y
    }

    /// Return the file (ie. the column)
    #[inline]
    pub fn get_file(&self) -> u8 {
        self.x
    }

    /// If there is a square above me, return that.  Otherwise, None.
    #[inline]
    pub fn up(&self) -> Option<Self> {
        match self.y {
            8 => None,
            _ => Some(ChessPosition::new(self.x, self.y + 1)),
        }
    }

    /// If there is a square below me, return that.  Otherwise, None.
    #[inline]
    pub fn down(&self) -> Option<Self> {
        match self.y {
            0 => None,
            _ => Some(ChessPosition::new(self.x, self.y - 1)),
        }
    }

    /// If there is a square to the left of me, return that.  Otherwise, None.
    #[inline]
    pub fn right(&self) -> Option<Self> {
        match self.x {
            8 => None,
            _ => Some(ChessPosition::new(self.x + 1, self.y)),
        }
    }

    /// If there is a square to the right of me, return that.  Otherwise, None.
    #[inline]
    pub fn left(&self) -> Option<Self> {
        match self.x {
            0 => None,
            _ => Some(ChessPosition::new(self.x - 1, self.y)),
        }
    }

    /// If there is a square "forward", given my `Color`, go in that direction.  Otherwise, None.
    #[inline]
    pub fn forward(&self, color: Color) -> Option<Self> {
        match color {
            Color::White => self.up(),
            Color::Black => self.down(),
        }
    }

    #[inline]
    pub fn backward(&self, color: Color) -> Option<Self> {
        match color {
            Color::White => self.down(),
            Color::Black => self.up(),
        }
    }
}

impl fmt::Display for ChessPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (('a' as u8) + self.y) as char,
            (('1' as u8) + self.x) as char
        )
    }
}

impl FromStr for ChessPosition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::InvalidGridPosition);
        }
        let ch: Vec<char> = s.chars().collect();
        match ch[0] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => {},
            _ => return Err(Error::InvalidGridPosition),
        }
        match ch[1] {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {},
            _ => return Err(Error::InvalidGridPosition),
        }
        Ok(ChessPosition::new(
            ch[0] as u8 - 'a' as u8 + 1,
            ch[1] as u8 - '1' as u8 + 1,
        ))
    }
}
