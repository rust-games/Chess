use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::{event, graphics, Context, GameError, GameResult};
use glam::vec2;
use log::{debug, info, trace, warn};

use crate::{
    Chess, GameState, Square, Theme, ALL_SQUARES, BOARD_CELL_PX_SIZE, BOARD_PX_SIZE, BOARD_SIZE,
    SCREEN_PX_SIZE, SIDE_SCREEN_PX_SIZE,
};

/// A wrapper of [`Chess`] for GUI.
#[derive(Default, Debug)]
pub struct ChessGui {
    chess: Chess,
    theme: Theme,
}

impl ChessGui {
    /// Create a new instance of ChessGui.
    pub fn new(chess: Chess, theme: Theme) -> Self {
        ChessGui { chess, theme }
    }

    /// Set the theme for the GUI.
    ///
    /// # Examples
    ///
    /// ```
    /// use chess::{ChessGui, THEME_SANDCASTLE};
    ///
    /// let mut game = ChessGui::default();
    /// game.set_theme(THEME_SANDCASTLE);
    /// ```
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Base function to call when a user click on the screen.
    fn click(&mut self, x: f32, y: f32) {
        if x < BOARD_PX_SIZE.0 {
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
        match self.chess.square_focused {
            Some(square_selected) => self.chess.play(square_selected, current_square),
            None => {
                if self
                    .chess
                    .board
                    .color_on_is(current_square, self.chess.board.side_to_move())
                {
                    self.chess.square_focused = Some(current_square);
                }
            }
        }
    }

    /// React when the user click on the side screen.
    ///
    /// It is the callers responsibility to ensure the coordinate is in the side.
    fn click_on_side(&self, x: f32, y: f32) {
        // todo
        info!("Click at: ({x},{y}) -> on the side screen")
    }

    /// Draw all of the board side.
    fn draw_board(&self, ctx: &mut Context) -> GameResult {
        self.draw_empty_board(ctx)?;
        self.draw_legal_moves(ctx)?;
        self.draw_pinned_piece(ctx)?;
        self.draw_content_board(ctx)?;
        Ok(())
    }

    /// Draw the empty chess board (without pieces).
    fn draw_empty_board(&self, ctx: &mut Context) -> GameResult {
        for y in 0..BOARD_SIZE.1 {
            for x in 0..BOARD_SIZE.0 {
                let color_index = if (x % 2 == 1 && y % 2 == 1) || (x % 2 == 0 && y % 2 == 0) {
                    0
                } else {
                    1
                };
                let mesh = graphics::MeshBuilder::new()
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            x as f32 * BOARD_CELL_PX_SIZE.0,
                            y as f32 * BOARD_CELL_PX_SIZE.1,
                            BOARD_CELL_PX_SIZE.0,
                            BOARD_CELL_PX_SIZE.1,
                        ),
                        self.theme.board_color[color_index],
                    )?
                    .build(ctx)?;
                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }
        }
        Ok(())
    }

    /// Draw pieces on the board.
    fn draw_content_board(&self, ctx: &mut Context) -> GameResult {
        let mut path;
        let mut image;
        for square in ALL_SQUARES {
            if let Some((piece, color)) = self.chess.board.on(square) {
                path = self.theme.piece_path[color.to_index()][piece.to_index()];
                image = graphics::Image::new(ctx, path).expect("Image load error");
                let (x, y) = square.to_screen();
                let dest_point = vec2(x, y);
                let image_scale = vec2(0.5, 0.5);
                let dp = graphics::DrawParam::new()
                    .dest(dest_point)
                    .scale(image_scale);
                graphics::draw(ctx, &image, dp)?;
            }
        }
        Ok(())
    }

    /// Draw all the possible destination of the selected piece.
    fn draw_legal_moves(&self, ctx: &mut Context) -> GameResult {
        if self.theme.valid_moves_color.is_some() {
            if let Some(square) = self.chess.square_focused {
                for dest in self.chess.board.get_legal_moves(square) {
                    let (x, y) = dest.to_screen();
                    let mesh = graphics::MeshBuilder::new()
                        .rectangle(
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(x, y, BOARD_CELL_PX_SIZE.0, BOARD_CELL_PX_SIZE.1),
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
        if self.theme.piece_pinned_path.is_some() {
            let mut path;
            let mut image;
            for square in self.chess.board.pinned() {
                path = self.theme.piece_pinned_path.unwrap();
                image = graphics::Image::new(ctx, path).expect("Image load error");
                let (x, y) = square.to_screen();
                let dest_point = vec2(x, y);
                // We set the scale at 1.0 because we want the same size
                // for the image and a Board_cell
                const SCALE: f32 = 1.0;
                let image_scale = vec2(
                    SCALE * (BOARD_CELL_PX_SIZE.0 / image.width() as f32),
                    SCALE * (BOARD_CELL_PX_SIZE.1 / image.height() as f32),
                );
                let dp = graphics::DrawParam::new()
                    .dest(dest_point)
                    .scale(image_scale);
                graphics::draw(ctx, &image, dp)?;
            }
        } else if self.theme.piece_pinned_color.is_some() {
            for piece in self.chess.board.pinned() {
                let (x, y) = piece.to_screen();
                let mesh = graphics::MeshBuilder::new()
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(x, y, BOARD_CELL_PX_SIZE.0, BOARD_CELL_PX_SIZE.1),
                        self.theme.piece_pinned_color.unwrap(),
                    )?
                    .build(ctx)?;
                graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
            }
        }
        Ok(())
    }

    /// Draw all the side screen.
    ///
    /// TODO: according to the state, print some features
    ///     - timer (print real timer)
    fn draw_side(&self, ctx: &mut Context) -> GameResult {
        // button White timer
        let bounds = graphics::Rect::new(20.0, 20.0, 115.0, 50.0);
        let color = graphics::Color::new(0.5, 0.5, 0.5, 1.0);
        self.draw_button(ctx, bounds, color, "White Timer\n00:00")?;

        // button Black timer
        let bounds = graphics::Rect::new(155.0, 20.0, 115.0, 50.0);
        let color = graphics::Color::new(0.5, 0.5, 0.5, 1.0);
        self.draw_button(ctx, bounds, color, "Black Timer\n00:00")?;

        // button theme
        let bounds = graphics::Rect::new(SIDE_SCREEN_PX_SIZE.0 - 70.0, 20.0, 50.0, 50.0);
        let color = graphics::Color::new(0.5, 0.5, 0.5, 1.0);
        self.draw_button(ctx, bounds, color, "T")?;

        // line
        let mesh = graphics::MeshBuilder::new()
            .line(
                &[vec2(BOARD_PX_SIZE.0, 90.0), vec2(SCREEN_PX_SIZE.0, 90.0)],
                1.0,
                graphics::Color::WHITE,
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // button winner
        let bounds = graphics::Rect::new(20.0, 110.0, 320.0, SIDE_SCREEN_PX_SIZE.1 - 250.0 - 110.0);
        let color = graphics::Color::from_rgb(200, 200, 200);
        self.draw_button(ctx, bounds, color, "Winner")?;

        // line
        let mesh = graphics::MeshBuilder::new()
            .line(
                &[
                    vec2(BOARD_PX_SIZE.0, SIDE_SCREEN_PX_SIZE.1 - 230.0),
                    vec2(SCREEN_PX_SIZE.0, SIDE_SCREEN_PX_SIZE.1 - 230.0),
                ],
                1.0,
                graphics::Color::WHITE,
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // button undo
        let bounds = graphics::Rect::new(20.0, SIDE_SCREEN_PX_SIZE.1 - 210.0, 150.0, 50.0);
        let color = (0.0, 0.0, 0.7);
        self.draw_button(ctx, bounds, color, "Undo")?;

        // button declare draw - conditional draw
        let bounds = graphics::Rect::new(190.0, SIDE_SCREEN_PX_SIZE.1 - 210.0, 150.0, 50.0);
        let color = (0.7, 0.0, 0.7);
        self.draw_button(ctx, bounds, color, "Declare Draw")?;

        // button offer draw
        let bounds = graphics::Rect::new(20.0, SIDE_SCREEN_PX_SIZE.1 - 140.0, 150.0, 50.0);
        let color = (0.7, 0.7, 0.0);
        self.draw_button(ctx, bounds, color, "Offer Draw")?;

        // button accept draw - conditional draw
        let bounds = graphics::Rect::new(190.0, SIDE_SCREEN_PX_SIZE.1 - 140.0, 150.0, 50.0);
        let color = (0.0, 0.7, 0.0);
        self.draw_button(ctx, bounds, color, "Accept Draw")?;

        // button reset
        let bounds = graphics::Rect::new(20.0, SIDE_SCREEN_PX_SIZE.1 - 70.0, 150.0, 50.0);
        let color = (0.0, 0.0, 0.7);
        self.draw_button(ctx, bounds, color, "Reset")?;

        // button resign
        let bounds = graphics::Rect::new(190.0, SIDE_SCREEN_PX_SIZE.1 - 70.0, 150.0, 50.0);
        let color = (0.7, 0.0, 0.0);
        self.draw_button(ctx, bounds, color, "Resign")?;

        Ok(())
    }

    /// Draw a button.
    ///
    /// > Add the BOARD_PX_SIZE.x to the bound for being in the side panel.
    fn draw_button<C>(
        &self,
        ctx: &mut Context,
        bounds: graphics::Rect,
        color: C,
        text: &str,
    ) -> GameResult
    where
        C: Into<graphics::Color>,
    {
        let bounds = graphics::Rect {
            x: BOARD_PX_SIZE.0 + bounds.x,
            ..bounds
        };
        let mesh = graphics::MeshBuilder::new()
            .rectangle(graphics::DrawMode::fill(), bounds, color.into())?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Set and draw the text in the popup
        let font = graphics::Font::new(ctx, self.theme.font_path)?;
        let text = graphics::Text::new((text, font, self.theme.font_scale));
        let dest_point = glam::Vec2::new(
            bounds.x + (bounds.w - text.width(ctx)) / 2.0,
            bounds.y + (bounds.h - text.height(ctx)) / 2.0,
        );
        graphics::draw(ctx, &text, (dest_point,))?;

        Ok(())
    }

    /// Draw a window with winner, score?, stats?
    fn draw_winner(&self, _ctx: &mut Context, game_state: GameState) -> GameResult {
        warn!("NotImplementedYet: draw_winner()");
        trace!("GameState: {:?}", game_state);
        Ok(())
    }
}

impl event::EventHandler<GameError> for ChessGui {
    /// Update will happen on every frame before it is drawn.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // First we clear the screen and set the background color
        graphics::clear(ctx, self.theme.background_color);

        // Draw according to the GameState
        match self.chess.state() {
            GameState::Ongoing => {
                self.draw_board(ctx)?;
                self.draw_side(ctx)?;
            }
            game_state => self.draw_winner(ctx, game_state)?,
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
        keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            // Z: Quit the game
            KeyCode::Escape => event::quit(ctx),
            // R: Reset the game (new chess game)
            KeyCode::R => self.chess.reset(),
            // CTRL+Z: Undo (i.e. go back one step in history)
            KeyCode::Z if keymod == KeyMods::CTRL => self.chess.undo(),
            _ => {}
        };
    }
}
