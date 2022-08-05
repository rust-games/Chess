use std::ops::Not;

/// Represent a color.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

/// How many colors are there?
pub const NUM_COLORS: usize = 2;

/// List all colors
pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];

impl Color {
    /// Convert the `Color` to a `usize` for table lookups.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Not for Color {
    type Output = Self;

    /// Get the other color.
    #[inline]
    fn not(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
