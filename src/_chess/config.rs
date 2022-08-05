//! Config file that defines every constant
//!
//! | Component | axes   |
//! |-----------|--------|
//! | foo.0     | x-axes |
//! | foo.1     | y-axes |

/// The size in Pixel of the board only (the board is a square)
pub const BOARD_SIZE: (u32, u32) = (800, 800);

/// The size in pixel of the side screen
pub const SIDE_SCREEN_SIZE: (u32, u32) = (200, BOARD_SIZE.1);

/// Here we define the size of the screen (in pixel) for displaying the game
pub const SCREEN_SIZE: (u32, u32) = (BOARD_SIZE.0 + SIDE_SCREEN_SIZE.0, BOARD_SIZE.1);

/// Here we define the size of the grid in term of how many cells we will have
pub const GRID_SIZE: (i16, i16) = (8, 8);

/// Here we calculate the size of a cell (in pixel) in the grid
pub const GRID_CELL_SIZE: (i16, i16) = (
    BOARD_SIZE.0 as i16 / GRID_SIZE.0,
    BOARD_SIZE.1 as i16 / GRID_SIZE.1,
);
