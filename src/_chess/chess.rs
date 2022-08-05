//! Main module of the game logic

use crate::chess::piece::Piece;
use super::board::Board;
use super::theme::*;

pub struct Chess {
    board: Board,
    gameover: bool,
    theme: Theme,
}

impl Chess {
    pub fn new() -> Self {
        Chess {
            board: Board::default(),
            gameover: false,
            theme: DEFAULT_THEME,
        }
    }
}
