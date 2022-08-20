//! Config file that defines every constant
//!
//! | Component | axes   |
//! |-----------|--------|
//! | foo.0     | x-axes |
//! | foo.1     | y-axes |

/// The pixel-size of the board only (the board has a square form).
pub const BOARD_PX_SIZE: (u32, u32) = (800, 800);

/// The pixel-size of the side screen.
pub const SIDE_SCREEN_PX_SIZE: (u32, u32) = (300, BOARD_PX_SIZE.1);

/// The pixel-size of the screen.
pub const SCREEN_PX_SIZE: (u32, u32) = (BOARD_PX_SIZE.0 + SIDE_SCREEN_PX_SIZE.0, BOARD_PX_SIZE.1);

/// Number of cells in the Board.
pub const BOARD_SIZE: (i16, i16) = (8, 8);

/// The pixel-size of a Board's cell.
pub const BOARD_CELL_PX_SIZE: (i16, i16) = (
    BOARD_PX_SIZE.0 as i16 / BOARD_SIZE.0,
    BOARD_PX_SIZE.1 as i16 / BOARD_SIZE.1,
);
