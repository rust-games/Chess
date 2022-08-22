use std::str::FromStr;

use crate::{Board, ChessMove, Color, Square};

/// The Result of the game.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum GameState {
    /// The game is still ongoing.
    #[default]
    Ongoing,
    /// A player is checkmates.
    Checkmates(Color),
    /// Draw by Stalemate.
    Stalemate,
    /// Draw by request accepted (ie. Mutual Agreement).
    DrawAccepted,
    /// Draw declared by a player.
    DrawDeclared,
    /// The [`Color`] has resigns.
    Resigns(Color),
}

/// A Standard Chess game.
///
/// TODO: Add a timer for each player.
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Chess {
    pub(crate) board: Board,
    pub(crate) square_focused: Option<Square>,
    pub(crate) history: Vec<String>,
    pub(crate) state: GameState,
}

impl Chess {
    /// Create a new instance of Chess.
    pub fn new(board: Board) -> Self {
        Chess {
            board,
            square_focused: None,
            history: vec![],
            state: GameState::Ongoing,
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
        self.state = GameState::Ongoing;
    }

    /// Return the [`State`][GameState] of the Game.
    pub fn state(&mut self) -> GameState {
        self.state
    }

    /// Base function to call when a user click on the screen.
    pub fn play(&mut self, from: Square, to: Square) {
        let m = ChessMove::new(from, to);
        if self.board.is_legal(m) {
            self.history.push(self.board.to_string());
            self.board.update(m);
            self.state = self.board.state();
        }
        self.square_focused = None;
    }

    /// [`Color`] offer a draw.
    #[cfg(any())]
    pub fn offer_draw(&mut self, color: Color) {}

    /// [`Color`] accept the draw. Assumes that a draw is offered.
    ///
    /// > **Caution**: This crate don't implement the offer_draw() method.
    ///   You need to react yourself to this action.
    pub fn accept_draw(&mut self) {
        self.state = GameState::DrawAccepted;
    }

    /// Verify if a player can legally declare a draw by 3-fold repetition or 50-move rule.
    pub fn can_declare_draw(&self) -> bool {
        let t = self.history.len();
        let fen_boards = [
            self.board
                .to_string()
                .split(' ')
                .next()
                .unwrap()
                .to_string(),
            self.history[t - 2].clone(),
            self.history[t - 4].clone(),
        ];
        if fen_boards[0] == fen_boards[1] && fen_boards[1] == fen_boards[2] {
            return true;
        }
        if self.board.halfmoves() >= 100 {
            return true;
        }
        false
    }

    /// Declare a draw by 3-fold repetition or 50-move rule. Assumes that a draw can be declare.
    pub fn declare_draw(&mut self) {
        self.state = GameState::DrawDeclared;
    }

    /// [`Color`] resigns the game.
    pub fn resign(&mut self, color: Color) {
        self.state = GameState::Resigns(color);
    }
}
