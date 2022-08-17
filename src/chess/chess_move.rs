use std::fmt;

use crate::{Board, Error, Square};

/// Represent a ChessMove in memory.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ChessMove {
    source: Square,
    dest: Square,
}

impl ChessMove {
    /// Create a new chess move.
    #[inline]
    pub fn new(source: Square, dest: Square) -> Self {
        ChessMove { source, dest }
    }

    /// Get the source [`Square`] (square the piece is currently on).
    #[inline]
    pub fn get_source(&self) -> Square {
        self.source
    }

    /// Get the destination [`Square`] (square the piece is going to).
    #[inline]
    pub fn get_dest(&self) -> Square {
        self.dest
    }

    /// Convert a SAN (Standard Algebraic Notation) move into a [`ChessMove`].
    ///
    /// ```
    /// use chess::{Board, ChessMove, Square};
    ///
    /// let board = Board::default();
    /// assert_eq!(
    ///     ChessMove::from_san(&board, "e4").expect("e4 is valid in the initial position"),
    ///     ChessMove::new(Square::E2, Square::E4)
    /// );
    /// ```
    pub fn from_san(board: &Board, move_text: &str) -> Result<ChessMove, Error> {
        todo!("ChessMOVE::from_san()")
    }
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.source, self.dest)
    }
}
