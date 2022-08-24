use std::fmt;

use ggez::{graphics, Context, GameResult};

use crate::{ChessGui, THEME_DUST};

/// Indicate how align the text.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub enum Align {
    Left,
    Right,
    #[default]
    Center,
}

/// A struct of button for interact with the GUI.
#[derive(Copy, Clone)]
pub struct Button {
    /// The id is not unique, it's just a name to identify it.
    pub id: &'static str,
    rect: graphics::Rect,
    color: graphics::Color,
    text: &'static str,
    align: Align,
    func: Option<fn(&mut ChessGui)>,
}

pub static FONT_PATH: &str = THEME_DUST.font_path;
pub static FONT_SCALE: f32 = THEME_DUST.font_scale;

impl Button {
    /// Create a new [`Button`].
    pub fn new(
        id: &'static str,
        rect: graphics::Rect,
        color: graphics::Color,
        text: &'static str,
        align: Align,
        func: Option<fn(&mut ChessGui)>,
    ) -> Self {
        Button {
            id,
            rect,
            color,
            text,
            align,
            func,
        }
    }

    /// Verify if a coordinate is in the button.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.rect.contains([x, y])
    }

    /// Draw the button in the [`Context`].
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        self.draw_rect(ctx)?;
        self.draw_text(ctx)?;
        Ok(())
    }

    /// Draw the button without text.
    fn draw_rect(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::MeshBuilder::new()
            .rectangle(graphics::DrawMode::fill(), self.rect, self.color)?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }

    /// Draw the text of the button.
    fn draw_text(&self, ctx: &mut Context) -> GameResult {
        let font = graphics::Font::new(ctx, FONT_PATH)?;
        let text = graphics::Text::new((self.text.clone(), font, FONT_SCALE));
        let dest_point = match self.align {
            Align::Left => [self.rect.x, self.rect.y],
            Align::Right => [
                self.rect.x + self.rect.w - text.width(ctx),
                self.rect.y + self.rect.h - text.height(ctx),
            ],
            Align::Center => [
                self.rect.x + (self.rect.w - text.width(ctx)) / 2.0,
                self.rect.y + (self.rect.h - text.height(ctx)) / 2.0,
            ],
        };
        graphics::draw(ctx, &text, (dest_point,))?;
        Ok(())
    }

    /// Call the func when the button is clicked.
    pub fn clicked(&self, chess_gui: &mut ChessGui) {
        if let Some(func) = self.func {
            func(chess_gui);
        }
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl fmt::Debug for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
