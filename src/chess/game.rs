use super::Color;

/// Contains all actions supported within the game
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub enum Action {
    MakeMove(ChessMove),
    OfferDraw(Color),
    AcceptDraw,
    DeclareDraw,
    Resign(Color),
}

/// What was the result of this game?
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum GameState {
    Checkmates(Color), // Color has loose
    Resigns(Color),    // Color resigns
    Stalemate,         // Draw by situation
    DrawAccepted,      // Draw by request
    DrawDeclared,      // Draw request (waiting an answer) -> don't stop the timer
}
