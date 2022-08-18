/// Describe 8 directions.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

/// Numbers of line (vertical and horizontal).
pub const NUM_LINES: usize = 4;

/// Enumerate [`Directions`] in line (vertical and horizontal).
pub const ALL_LINE: [Direction; NUM_LINES] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

/// Numbers of diagonal.
pub const NUM_DIAGONAL: usize = 4;

/// Enumerate [`Directions`] in diagonal.
pub const ALL_DIAGONAL: [Direction; NUM_DIAGONAL] = [
    Direction::UpRight,
    Direction::DownRight,
    Direction::DownLeft,
    Direction::UpLeft,
];

/// Numbers of [`Direction`].
pub const NUM_DIRECTION: usize = NUM_LINES + NUM_DIAGONAL;

/// Enumerate all [`Direction`].
pub const ALL_DIRECTION: [Direction; NUM_DIRECTION] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];
