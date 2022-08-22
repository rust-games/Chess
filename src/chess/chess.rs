use std::str::FromStr;

use crate::{Board, ChessMove, Color, Square};

/// Contains all actions supported within the game.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameAction {
    OfferDraw(Color),
    AcceptDraw,
    RefuseDraw,
    Resign(Color),
}

/// The Result of the game.
///
/// # Examples
///
/// ```
/// use chess::{Color, GameState};
///
/// let state = GameState::Checkmates(Color::Black);
/// assert_eq!(Some(Color::White), state.winner())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    /// The game is still ongoing.
    Ongoing,
    /// Checkmates for the given [`Color`] (ie. the looser).
    Checkmates(Color),
    /// Draw by Stalemate.
    Stalemate,
    /// Draw by request (ie. Mutual Agreement).
    DrawByRequest,
    /// The [`Color`] has resigns.
    Resigns(Color),
}

impl GameState {
    pub fn winner(&self) -> Option<Color> {
        match *self {
            GameState::Ongoing => None,
            GameState::Checkmates(color) => Some(!color),
            GameState::Stalemate => None,
            GameState::DrawByRequest => None,
            GameState::Resigns(color) => Some(!color),
        }
    }
}

/// A Standard Chess game.
///
/// TODO: Add a timer for each player.
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Chess {
    pub(crate) board: Board,
    pub(crate) square_focused: Option<Square>,
    pub(crate) history: Vec<String>,
}

impl Chess {
    /// Create a new instance of Chess.
    pub fn new(board: Board) -> Self {
        Chess {
            board,
            square_focused: Default::default(),
            history: Default::default(),
        }
    }

    /// Get the history of the game.
    ///
    /// The [`Vec`] contains a FEN-string.
    pub fn history(&self) -> Vec<String> {
        self.history.clone()
    }

    /// Go back one step in history.
    ///
    /// If the history is empty, reset the board to it's [`default`][Board::default] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::{Chess, Square};
    /// let mut chess = Chess::default();
    /// let expected = Chess::default();
    /// chess.play(Square::A2, Square::A4);
    /// chess.undo();
    ///
    /// assert_eq!(chess, expected);
    /// ```
    pub fn undo(&mut self) {
        if let Some(fen) = self.history.pop() {
            self.board = Board::from_str(fen.as_str()).expect("valid fen from history");
        }
    }

    /// Reset the Game (board and history).
    pub fn reset(&mut self) {
        self.board = Board::default();
        self.square_focused = None;
        self.history = vec![];
    }

    /// Return the [`State`][GameState] of the Game
    ///
    /// TODO: Verify here if we need the set the State to DrawByRequest or Resigns
    pub fn state(&self) -> GameState {
        let state = self.board.state();
        state
    }

    /// Base function to call when a user click on the screen.
    pub fn play(&mut self, from: Square, to: Square) {
        let m = ChessMove::new(from, to);
        if self.board.is_legal(m) {
            self.history.push(self.board.to_string());
            self.board.update(m);
        }
        self.square_focused = None;
    }
}
