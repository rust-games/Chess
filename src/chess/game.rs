use ggez::event::{self, KeyCode, MouseButton};
use ggez::{graphics, Context, GameError};
use log::{debug, info};

use crate::{Board, ChessMove, Color, File, Rank, Square, Theme, BOARD_PX_SIZE, THEME_DEFAULT};

/// Contains all actions supported within the game.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Action {
    MakeMove(ChessMove),
    OfferDraw(Color),
    AcceptDraw,
    DeclareDraw,
    Resign(Color),
}

/// The Result of the game.
///
/// # Examples
///
/// ```
/// use chess::{Color, GameState};
///
/// let state = GameState::Checkmates(Color::Black);
/// assert!("The winner is: White", state.winner())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Checkmates for the given [`Color`] (ie. the looser).
    Checkmates(Color),
    /// The [`Color`] has resigns.
    Resigns(Color),
    /// Draw by Stalemate.
    Stalemate,
    /// Draw by request (ie. Mutual Agreement).
    DrawByRequest,
}

impl GameState {
    pub fn winner(&self) -> Option<Color> {
        match *self {
            GameState::Checkmates(color) => Some(!color),
            GameState::Resigns(color) => Some(!color),
            GameState::Stalemate => None,
            GameState::DrawByRequest => None,
        }
    }
}

/// A Standard Chess game.
pub struct Chess {
    board: Board,
    square_selected: Option<Square>,
    theme: Theme,
    historic: Vec<String>,
}

impl Chess {
    /// Create a new instance of Chess.
    pub fn new() -> Self {
        Chess {
            board: Board::default(),
            square_selected: None,
            theme: THEME_DEFAULT,
            historic: vec![],
        }
    }

    /// Set the theme of the game.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess::{Chess, SANDCASTLE_THEME};
    /// let mut game = Chess::new();
    /// game.set_theme(SANDCASTLE_THEME);
    /// ```
    #[allow(unused)]
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Reset the Game.
    pub fn reset(&mut self) {
        self.board = Board::default();
        self.square_selected = None;
        self.historic = vec![];
    }

    /// Base function to call when a user click on the screen.
    pub fn click(&mut self, x: f32, y: f32) {
        if x < BOARD_PX_SIZE.0 as f32 {
            self.click_on_board(x, y);
        } else {
            self.click_on_side(x, y);
        }
    }

    /// React when the user click on the board screen.
    ///
    /// It is the callers responsibility to ensure the coordinate is in the board.
    fn click_on_board(&mut self, x: f32, y: f32) {
        let current_square = Square::from_screen(x, y);
        debug!("Click at: ({x},{y}) -> on the square: {current_square}");
        match self.square_selected {
            Some(square_selected) => self.play(square_selected, current_square),
            None => self.square_selected = Some(current_square),
        }
    }

    /// React when the user click on the side screen.
    ///
    /// It is the callers responsibility to ensure the coordinate is in the side.
    fn click_on_side(&self, x: f32, y: f32) {
        todo!("click_on_side()")
    }

    /// Base function to call when a user click on the screen.
    pub fn play(&mut self, from: Square, to: Square) {
        debug!("The player {:?} play {} to {}", self.board.side_to_move(), from, to);
        let m = ChessMove::new(from, to);
        if self.board.is_legal(m) {
            self.board.update(m).expect("valid move");
            self.historic.push(self.board.to_string());
        }
        self.square_selected = None;
    }
}

impl event::EventHandler<GameError> for Chess {
    /// Update will happen on every frame before it is drawn.
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    /// Render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // First we clear the screen and set the background color
        graphics::clear(ctx, self.theme.background_color);

        // Draw the board and his content
        //self.draw_board(ctx)?;

        // If the game is over draw a popup to show the score
        /*if self.chess.is_game_over() {
            self.draw_winner(ctx)?;
        }*/

        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx)?;

        // And return success.
        Ok(())
    }

    /// Called every time a mouse button gets pressed
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.click(x, y);
        }
    }

    /// Called every time a key gets pressed
    /// Inputs are managed here
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => {
                info!("EXIT from key Escape");
                event::quit(ctx);
            }
            KeyCode::R => {
                debug!("RESET from key R");
                self.reset();
            }
            _ => {}
        };
    }
}
