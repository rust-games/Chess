//! Describe all pieces and their interactions

use super::board::Board;
use super::common::Position;

pub(crate) trait Piece {
    fn get_side(&self) -> ();
    fn is_valid_move(&self, from: Position, to: Position, board: Board) -> bool;
}
