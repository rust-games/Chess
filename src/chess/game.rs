use ggez::event::{self, KeyCode, MouseButton};
use ggez::{graphics, Context, GameError, GameResult};
use glam::Vec2;
use log::{debug, info, trace, warn};

use crate::{
    Board, ChessMove, Color, Square, Theme, ALL_SQUARES, BOARD_CELL_PX_SIZE, BOARD_PX_SIZE,
    BOARD_SIZE, THEME_DEFAULT,
};

/// Contains all actions supported within the game.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameAction {
    OfferDraw(Color),
    AcceptDraw,
    RefuseDraw,
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
/// assert!(Some(Color::White), state.winner())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// The game is still ongoing.
    Ongoing,
    /// Checkmates for the given [`Color`] (ie. the looser).
    Checkmates(Color),
    /// Draw by Stalemate.
    Stalemate,
    /// Draw by request (ie. Mutual Agreement).
    DrawByRequest,
    /// The [`Color`] has resigns.
    Resigns(Color),
}

impl GameState {
    pub fn winner(&self) -> Option<Color> {
        match *self {
            GameState::Ongoing => None,
            GameState::Checkmates(color) => Some(!color),
            GameState::Stalemate => None,
            GameState::DrawByRequest => None,
            GameState::Resigns(color) => Some(!color),
        }
    }
}

/// A Standard Chess game.
///
/// TODO: Add a timer for each player.
pub struct Chess {
    board: Board,
    square_focused: Option<Square>,
    theme: Theme,
    historic: Vec<String>,
}

impl Chess {
    /// Create a new instance of Chess.
    pub fn new() -> Self {
        Chess {
            board: Board::default(),
            square_focused: None,
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
        self.square_focused = None;
        self.historic = vec![];
    }

    /// Return the [`State`][GameState] of the Game
    pub fn state(&self) -> GameState {
        warn!("state(): NotImplementedYet");
        GameState::Ongoing
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
        match self.square_focused {
            Some(square_selected) => self.play(square_selected, current_square),
            None => {
                if self.board.color_on_is(current_square, self.board.side_to_move()) {
                    self.square_focused = Some(current_square);
                }
            },
        }
    }

    /// React when the user click on the side screen.
    ///
    /// It is the callers responsibility to ensure the coordinate is in the side.
    fn click_on_side(&self, x: f32, y: f32) {
        // todo
        trace!("Click at: ({x},{y}) -> on the side screen")
    }

    /// Base function to call when a user click on the screen.
    pub fn play(&mut self, from: Square, to: Square) {
        debug!(
            "The player {:?} play {} to {}",
            self.board.side_to_move(),
            from,
            to
        );
        let m = ChessMove::new(from, to);
        if self.board.is_legal(m) {
            self.board.update(m).expect("valid move");
            self.historic.push(self.board.to_string());
        }
        self.square_focused = None;
    }

    /// Draw the all the board side.
    pub fn draw_board(&self, ctx: &mut Context) -> GameResult {
        self.draw_empty_board(ctx)?;
        if self.theme.valid_moves_color.is_some() {
            self.draw_valid_moves(ctx)?;
        }
        self.draw_pinned_piece(ctx)?;
        self.draw_content_board(ctx)?;
        Ok(())
    }

    /// Draw the empty chess board (without pieces).
    fn draw_empty_board(&self, ctx: &mut Context) -> GameResult {
        for y in 0..BOARD_SIZE.1 as i16 {
            for x in 0..BOARD_SIZE.0 as i16 {
                let color_index = if (x % 2 == 1 && y % 2 == 1) || (x % 2 == 0 && y % 2 == 0) {
                    0
                } else {
                    1
                };
                let mesh = graphics::MeshBuilder::new()
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            (x * BOARD_CELL_PX_SIZE.0) as f32,
                            (y * BOARD_CELL_PX_SIZE.1) as f32,
                            BOARD_CELL_PX_SIZE.0 as f32,
                            BOARD_CELL_PX_SIZE.1 as f32,
                        ),
                        self.theme.board_color[color_index],
                    )?
                    .build(ctx)?;
                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }
        }
        Ok(())
    }

    /// Draw pieces.
    fn draw_content_board(&self, ctx: &mut Context) -> GameResult {
        // todo: draw_content_board()
        let mut path;
        let mut image;
        for square in ALL_SQUARES {
            if let Some((piece, color)) = self.board.on(square) {
                path = self.theme.piece_path[color.to_index()][piece.to_index()];
                image = graphics::Image::new(ctx, path).expect("Image load error");
                let (x, y) = square.to_screen();
                let dest_point = Vec2::new(x, y);
                let image_scale = Vec2::new(0.5, 0.5);
                let dp = graphics::DrawParam::new()
                    .dest(dest_point)
                    .scale(image_scale);
                graphics::draw(ctx, &image, dp)?;
            }
        }
        Ok(())
    }

    /// Draw all the possible destination of the selected piece.
    fn draw_valid_moves(&self, ctx: &mut Context) -> GameResult {
        if let Some(square) = self.square_focused {
            if let Some(valid_dest) = self.board.get_valid_moves(square) {
                for dest in valid_dest {
                    let (x, y) = dest.to_screen();
                    let mesh = graphics::MeshBuilder::new()
                        .rectangle(
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(
                                x,
                                y,
                                BOARD_CELL_PX_SIZE.0 as f32,
                                BOARD_CELL_PX_SIZE.1 as f32,
                            ),
                            self.theme.valid_moves_color.unwrap(),
                        )?
                        .build(ctx)?;
                    graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
                }
            }
        }
        Ok(())
    }

    /// Draw the [`Piece`] that are pinned (i.e. can't move).
    fn draw_pinned_piece(&self, ctx: &mut Context) -> GameResult {
        warn!("draw_pinned_piece(): NotImplementedYet");
        Ok(())
    }

    /// Draw a window with winner, score?, stats?
    pub fn draw_winner(&self, ctx: &mut Context, winner: String) -> GameResult {
        warn!("NotImplementedYet: draw_winner()");
        trace!("The winner is {}", winner);
        Ok(())
    }
}

impl event::EventHandler<GameError> for Chess {
    /// Update will happen on every frame before it is drawn.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // First we clear the screen and set the background color
        graphics::clear(ctx, self.theme.background_color);

        // Draw the board and his content
        self.draw_board(ctx)?;

        // If the game is over draw a popup to show the score
        match self.state() {
            GameState::Ongoing => { /* Do Nothing */ }
            GameState::Checkmates(color) => {
                let winner = match !color {
                    Color::White => "White".to_string(),
                    Color::Black => "Black".to_string(),
                };
                self.draw_winner(ctx, winner)?;
            }
            GameState::Stalemate => {
                self.draw_winner(ctx, "Stalemate".to_string())?;
            }
            GameState::DrawByRequest => {
                self.draw_winner(ctx, "Draw".to_string())?;
            }
            GameState::Resigns(color) => {
                let winner = match !color {
                    Color::White => "White".to_string(),
                    Color::Black => "Black".to_string(),
                };
                self.draw_winner(ctx, format!("{winner} wins by default"))?;
            }
        }

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
