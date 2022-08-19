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

impl Direction {
    /// Verify if a direction is contain in another.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::Direction;
    ///
    /// assert_eq!(Direction::Up.has(Direction::Up), true);
    /// assert_eq!(Direction::Up.has(Direction::Right), false);
    /// assert_eq!(Direction::UpRight.has(Direction::Up), true);
    /// assert_eq!(Direction::UpRight.has(Direction::Right), true);
    /// // but it's not symmetric
    /// assert_eq!(Direction::Up.has(Direction::UpRight), false);
    /// ```
    pub fn has(&self, direction: Direction) -> bool {
        match *self {
            Direction::Up if direction == Direction::Up => true,
            Direction::UpRight => match direction {
                Direction::Up | Direction::Right => true,
                _ => false,
            },
            Direction::Right if direction == Direction::Right => true,
            Direction::DownRight => match direction {
                Direction::Down | Direction::Right => true,
                _ => false,
            },
            Direction::Down if direction == Direction::Down => true,
            Direction::DownLeft => match direction {
                Direction::Down | Direction::Left => true,
                _ => false,
            },
            Direction::Left if direction == Direction::Left => true,
            Direction::UpLeft => match direction {
                Direction::Up | Direction::Left => true,
                _ => false,
            },
            _ => false,
        }
    }
}
