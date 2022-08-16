use crate::chess::Color;

/// What castle rights does a particular player have?
///
/// > rule: https://en.wikipedia.org/wiki/Castling
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub enum CastleRights {
    NoRights,
    KingSide,  // little Castle
    QueenSide, // Big Castle
    Both,
}

impl CastleRights {
    /// Convert `CastleRights` to `usize` for table lookups
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Convert `usize` to `CastleRights`.  Panic if invalid number.
    pub fn from_index(i: usize) -> CastleRights {
        match i {
            0 => CastleRights::NoRights,
            1 => CastleRights::KingSide,
            2 => CastleRights::QueenSide,
            3 => CastleRights::Both,
            e => panic!("IndexError for CastleRights: {}", e),
        }
    }

    /// Can I castle kingside?
    pub fn has_kingside(&self) -> bool {
        self.to_index() & 1 == 1
    }

    /// Can I castle queenside?
    pub fn has_queenside(&self) -> bool {
        self.to_index() & 2 == 2
    }
}
