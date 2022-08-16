use crate::{ChessMove, Color};

/// Contains all actions supported within the game
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Action {
    MakeMove(ChessMove),
    OfferDraw(Color),
    AcceptDraw,
    DeclareDraw,
    Resign(Color),
}

/// What was the result of this game?
///
/// ```
/// use chess::{Color, GameState}
///
/// let state = GameState::Checkmates(Color::Black);
/// assert!("The winner is: White", state.winner())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Checkmates(Color), // Color has loose
    Resigns(Color),    // Color resigns
    Stalemate,         // Draw by situation
    DrawAccepted,      // Draw by request
    DrawDeclared,      // Draw request (waiting an answer) -> don't stop the timer
}
