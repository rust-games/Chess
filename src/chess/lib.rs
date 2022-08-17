//! # Rust Chess Library
//!
//! This is a chess library for rust.
//!
//! ## Examples
//!
//! see [`main.rs`]()

mod board;
pub use crate::board::*;

mod game;
pub use crate::game::*;

mod piece;
pub use crate::piece::*;

mod color;
pub use crate::color::*;

mod castle_rights;
pub use crate::castle_rights::*;

mod error;
pub use crate::error::*;

mod config;
pub use crate::config::*;

mod square;
pub use crate::square::*;

mod file;
pub use crate::file::*;

mod rank;
pub use crate::rank::*;

mod chess_move;
pub use crate::chess_move::*;

mod theme;
pub use crate::theme::*;
