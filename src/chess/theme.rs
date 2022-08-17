//! Describe theme available in the game

use ggez::graphics::Color;

pub const DEFAULT_THEME: Theme = Theme {
    valid_moves_color: Some(Color::RED),
    background_color: Color::GREEN,
    grid_color: Color::WHITE,
    font_path: "/fonts/LiberationMono-Regular.ttf",
    font_scale: 15.0,
};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub valid_moves_color: Option<Color>,
    pub background_color: Color,
    pub grid_color: Color,

    // font of texts (from resources/)
    // don't forget to start with "/"
    // example: "/fonts/font.ttf"
    pub font_path: &'static str,
    pub font_scale: f32,
}
