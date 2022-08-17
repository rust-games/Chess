//! Describe the board and interaction with it.

use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::{
    CastleRights, ChessMove, Color, Error, File, Piece, Rank, Square, ALL_FILES, ALL_RANKS,
    NUM_COLORS, NUM_SQUARES,
};

/// A representation of a chess board.
///
/// # Examples
///
/// ```
/// use chess::{Square, Board, Color, Piece, ChessMove};
///
/// let mut board = Board::default();
/// // 8 | r n b q k b n r
/// // 7 | p p p p p p p p
/// // 6 | . . . . . . . .
/// // 5 | . . . . . . . .
/// // 4 | . . . . . . . .
/// // 3 | . . . . . . . .
/// // 2 | P P P P P P P P
/// // 1 | R N B Q K B N R
/// //   +----------------
/// //     A B C D E F G H
///
/// assert_eq!(board.on(Square::E8), Some((Piece::King, Color::Black)));
///
/// // White move the pawn from E2 to E4
/// let m = ChessMove::new(Square::E2, Square::E4);
/// board.update(m)?;
/// // 8 | r n b q k b n r
/// // 7 | p p p p p p p p
/// // 6 | . . . . . . . .
/// // 5 | . . . . . . . .
/// // 4 | . . . . P . . .
/// // 3 | . . . . . . . .
/// // 2 | P P P P . P P P
/// // 1 | R N B Q K B N R
/// //   +----------------
/// //     A B C D E F G H
///
/// assert_eq!(board.on(Square::E4), Some((Piece::Pawn, Color::White)));
/// assert_eq!(board.side_to_move(), Color::Black);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
    squares: [Option<(Piece, Color)>; NUM_SQUARES],
    side_to_move: Color,
    castle_rights: [CastleRights; NUM_COLORS],
    en_passant: Option<Square>,
    halfmoves: u64,
    fullmoves: u64,
}

impl Board {
    /// Create a new empty board.
    ///
    /// Consider using the [`Default`] trait to initialize the board.
    pub fn new() -> Self {
        Board {
            squares: [None; NUM_SQUARES],
            side_to_move: Color::White,
            castle_rights: [CastleRights::NoRights; NUM_COLORS],
            en_passant: None,
            halfmoves: 0,
            fullmoves: 1,
        }
    }

    /// Get the [`Color`] of the player who has to play.
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Get the [`CastleRights`] for a given side.
    pub fn castle_rights(&self, color: Color) -> CastleRights {
        self.castle_rights[color.to_index()]
    }

    /// Get the [`Square`] (if exist) of the En Passant.
    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    /// Get the halfmoves number.
    pub fn halfmoves(&self) -> u64 {
        self.halfmoves
    }

    /// Get the fullmoves number.
    pub fn fullmoves(&self) -> u64 {
        self.fullmoves
    }

    /// Check if the [`Move`][ChessMove] is legal.
    pub fn is_legal(&self, m: ChessMove) -> bool {
        todo!()
    }

    /// Update the chessboard according to the chess rules.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::{Board, ChessMove, Square};
    ///
    /// let mut board = Board::default();
    /// let m = ChessMove::new(Square::E2, Square::E4);
    ///
    /// board.update(m).expect("Valid move");
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidMove`]: The move doesn't respect the chess rules.
    pub fn update(&mut self, m: ChessMove) -> Result<(), Error> {
        if !self.is_legal(m) {
            return Err(Error::InvalidMove {
                board: *self,
                invalid_move: m,
            });
        }

        todo!();
        // manage normal, castle, en-passant move and promotion
        // ...

        self.halfmoves += 1;
        self.side_to_move = !self.side_to_move;
        self.en_passant = None;
        if self.side_to_move == Color::Black {
            self.fullmoves += 1;
        }
        Ok(())
    }

    /// Get the [`Piece`] at a given [`Square`].
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        match self.squares[square.to_index()] {
            Some((piece, _)) => Some(piece),
            None => None,
        }
    }

    /// Get the [`Color`] at a given [`Square`].
    pub fn color_on(&self, square: Square) -> Option<Color> {
        match self.squares[square.to_index()] {
            Some((_, color)) => Some(color),
            None => None,
        }
    }

    /// Get the [`Color`] at a given [`Square`].
    pub fn on(&self, square: Square) -> Option<(Piece, Color)> {
        match self.squares[square.to_index()] {
            Some((piece, color)) => Some((piece, color)),
            None => None,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut cur_rank = Rank::Eighth;
        let mut cur_file = File::A;
        let mut board = Board::new();

        let tokens: Vec<&str> = value.split(' ').collect();
        if tokens.len() < 6 {
            return Err(Error::InvalidFen {
                fen: value.to_string(),
            });
        }

        let pieces = tokens[0];
        let side = tokens[1];
        let castles = tokens[2];
        let ep = tokens[3];
        let halfmoves = tokens[4]; // increment each time a piece move (ie. a player play) => at 100 100 the game is a draw
        let fullmoves = tokens[5]; // increment each time black piece move (ie. black player play)

        // Piece Placement
        for x in pieces.chars() {
            match x {
                '/' => {
                    cur_rank = cur_rank.down();
                    cur_file = File::A;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    cur_file = File::new(cur_file.to_index() + (x as usize) - ('0' as usize));
                }
                'r' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Rook, Color::Black));
                    cur_file = cur_file.right();
                }
                'R' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Rook, Color::White));
                    cur_file = cur_file.right();
                }
                'n' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Knight, Color::Black));
                    cur_file = cur_file.right();
                }
                'N' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Knight, Color::White));
                    cur_file = cur_file.right();
                }
                'b' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Bishop, Color::Black));
                    cur_file = cur_file.right();
                }
                'B' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Bishop, Color::White));
                    cur_file = cur_file.right();
                }
                'p' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Pawn, Color::Black));
                    cur_file = cur_file.right();
                }
                'P' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Pawn, Color::White));
                    cur_file = cur_file.right();
                }
                'q' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Queen, Color::Black));
                    cur_file = cur_file.right();
                }
                'Q' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::Queen, Color::White));
                    cur_file = cur_file.right();
                }
                'k' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::King, Color::Black));
                    cur_file = cur_file.right();
                }
                'K' => {
                    board[Square::make_square(cur_file, cur_rank)] =
                        Some((Piece::King, Color::White));
                    cur_file = cur_file.right();
                }
                _ => {
                    return Err(Error::InvalidFen {
                        fen: value.to_string(),
                    });
                }
            }
        }

        // Side to move
        match side {
            "w" | "W" => board.side_to_move = Color::White,
            "b" | "B" => board.side_to_move = Color::Black,
            _ => {
                return Err(Error::InvalidFen {
                    fen: value.to_string(),
                })
            }
        }

        // Castling Rights
        if castles.contains("K") && castles.contains("Q") {
            board.castle_rights[Color::White.to_index()] = CastleRights::Both;
        } else if castles.contains("K") {
            board.castle_rights[Color::White.to_index()] = CastleRights::KingSide;
        } else if castles.contains("Q") {
            board.castle_rights[Color::White.to_index()] = CastleRights::QueenSide;
        } else {
            board.castle_rights[Color::White.to_index()] = CastleRights::NoRights;
        }

        if castles.contains("k") && castles.contains("q") {
            board.castle_rights[Color::Black.to_index()] = CastleRights::Both;
        } else if castles.contains("k") {
            board.castle_rights[Color::Black.to_index()] = CastleRights::KingSide;
        } else if castles.contains("q") {
            board.castle_rights[Color::Black.to_index()] = CastleRights::QueenSide;
        } else {
            board.castle_rights[Color::Black.to_index()] = CastleRights::NoRights;
        }

        // Possible En Passant Targets
        if let Ok(square) = Square::from_str(&ep) {
            board.en_passant = Some(square);
        }

        // halfmoves and fullmoves
        board.halfmoves = halfmoves.parse().unwrap_or(0);
        board.fullmoves = fullmoves.parse().unwrap_or(1);

        Ok(board)
    }
}

impl Index<Square> for Board {
    type Output = Option<(Piece, Color)>;

    fn index(&self, index: Square) -> &Self::Output {
        &self.squares[index.to_index()]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.squares[index.to_index()]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Piece Placement
        let mut count = 0;
        for rank in ALL_RANKS.iter().rev() {
            for file in ALL_FILES.iter() {
                let square_index = Square::make_square(*file, *rank).to_index();

                if let Some((piece, color)) = self.squares[square_index] {
                    if count != 0 {
                        write!(f, "{}", count)?;
                        count = 0;
                    }
                    write!(f, "{}", piece.to_string(color))?;
                } else {
                    count += 1;
                }
            }

            if count != 0 {
                write!(f, "{}", count)?;
            }

            if *rank != Rank::First {
                write!(f, "/")?;
            }
            count = 0;
        }

        write!(f, " ")?;

        // Side to move
        if self.side_to_move == Color::White {
            write!(f, "w ")?;
        } else {
            write!(f, "b ")?;
        }

        // Castling Rights
        write!(
            f,
            "{}",
            self.castle_rights[Color::White.to_index()].to_string(Color::White)
        )?;
        write!(
            f,
            "{}",
            self.castle_rights[Color::Black.to_index()].to_string(Color::Black)
        )?;
        if self.castle_rights[0] == CastleRights::NoRights
            && self.castle_rights[1] == CastleRights::NoRights
        {
            write!(f, "-")?;
        }

        write!(f, " ")?;

        // Possible En Passant Targets
        if let Some(sq) = self.en_passant() {
            write!(f, "{}", sq)?;
        } else {
            write!(f, "-")?;
        }

        // halfmoves and fullmoves
        write!(f, " {} {}", self.halfmoves, self.fullmoves)
    }
}
