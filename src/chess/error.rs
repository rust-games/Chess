use thiserror::Error;

/// Sometimes, bad stuff happens.
#[derive(Debug, Error)]
pub enum Error {
    /// The FEN (Forsyth-Edwards Notation) string is invalid
    #[Error("Invalid FEN string: {}", fen)]
    InvalidFen { fen: String },

    /// The board created from BoardBuilder was found to be invalid
    #[Error("The board specified did not pass sanity checks.  Are you sure the kings exist and the side to move cannot capture the opposing king?")]
    InvalidBoard,

    /// An attempt was made to create a square from an invalid string
    #[Error("The string specified does not contain a valid algebraic notation square")]
    InvalidGridPosition,

    /// An attempt was made to create a move from an invalid SAN string
    #[Error("The string specified does not contain a valid SAN notation move")]
    InvalidSanMove,
}
