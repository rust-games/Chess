//! The board module that manage all interaction with the board

use super::config::*;
use super::piece::Piece;
use grid::Grid;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Board<P> where P: Piece {
    board: Grid<P>,
    //historic: Vec<(Position, Position)>,
    //theme: Theme,
}

impl<P> Default for Board<P> {
    fn default() -> Self {
        let mut board = Board::new();
        board.init();
        board
    }
}

impl<P> Board<P> {
    /// Create a new Board
    pub fn new() -> Self {
        Board {
            board: Grid::new(GRID_SIZE.1 as usize, GRID_SIZE.0 as usize),
        }
    }

    /// Initialize the board for a new game
    pub fn init(&mut self) {
        todo!()
    }
}
