//! Describe the board and interaction with it

use crate::{Color, Square, GRID_SIZE};

pub struct Board {
    squares: [Square; (GRID_SIZE.0 * GRID_SIZE.1) as usize],
    side_to_move: Color,
    en_passant: Option<Square>,
}
