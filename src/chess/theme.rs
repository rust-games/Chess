//! Describe theme available in the game

use ggez::graphics::Color;

pub const DEFAULT_THEME: Theme = Theme {
    board_color: [
        Color::new(0.4375, 0.3984, 0.4648, 1.0),
        Color::new(0.7969, 0.7148, 0.6797, 1.0),
    ],
    piece_path: PieceRolePath {
        white: PiecePath {
            pawn: "/images/pieces/white_pawn.png",
            knight: "/images/pieces/white_knight.png",
            bishop: "/images/pieces/white_bishop.png",
            rook: "/images/pieces/white_rook.png",
            queen: "/images/pieces/white_queen.png",
            king: "/images/pieces/white_king.png",
        },
        black: PiecePath {
            pawn: "/images/pieces/black_pawn.png",
            knight: "/images/pieces/black_knight.png",
            bishop: "/images/pieces/black_bishop.png",
            rook: "/images/pieces/black_rook.png",
            queen: "/images/pieces/black_queen.png",
            king: "/images/pieces/black_king.png",
        },
    },
    valid_moves_color: Some(Color::RED),
    background_color: Color::new(0.5, 0.5, 0.5, 1.0),
    font_path: "/fonts/LiberationMono-Regular.ttf",
    font_scale: 15.0,
};

pub const CORAL_THEME: Theme = Theme {
    board_color: [
        Color::new(112.0 / 256.0, 162.0 / 256.0, 163.0 / 256.0, 1.0),
        Color::new(177.0 / 256.0, 228.0 / 256.0, 185.0 / 256.0, 1.0),
    ],
    ..DEFAULT_THEME
};

pub const DUST_THEME: Theme = DEFAULT_THEME;

pub const MARINE_THEME: Theme = Theme {
    board_color: [
        Color::new(111.0 / 256.0, 115.0 / 256.0, 210.0 / 256.0, 1.0),
        Color::new(157.0 / 256.0, 172.0 / 256.0, 255.0 / 256.0, 1.0),
    ],
    ..DEFAULT_THEME
};

pub const WHEAT_THEME: Theme = Theme {
    board_color: [
        Color::new(187.0 / 256.0, 190.0 / 256.0, 100.0 / 256.0, 1.0),
        Color::new(234.0 / 256.0, 240.0 / 256.0, 206.0 / 256.0, 1.0),
    ],
    ..DEFAULT_THEME
};

pub const EMERALD_THEME: Theme = Theme {
    board_color: [
        Color::new(111.0 / 256.0, 143.0 / 256.0, 114.0 / 256.0, 1.0),
        Color::new(173.0 / 256.0, 189.0 / 256.0, 143.0 / 256.0, 1.0),
    ],
    ..DEFAULT_THEME
};

pub const SANDCASTLE_THEME: Theme = Theme {
    board_color: [
        Color::new(184.0 / 256.0, 139.0 / 256.0, 74.0 / 256.0, 1.0),
        Color::new(227.0 / 256.0, 193.0 / 256.0, 111.0 / 256.0, 1.0),
    ],
    ..DEFAULT_THEME
};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    // [dark, light]
    pub board_color: [Color; 2],
    pub piece_path: PieceRolePath,
    pub valid_moves_color: Option<Color>,
    pub background_color: Color,

    // font of texts (from resources/)
    // don't forget to start with "/"
    // example: "/fonts/font.ttf"
    pub font_path: &'static str,
    pub font_scale: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct PieceRolePath {
    pub white: PiecePath,
    pub black: PiecePath,
}

#[derive(Debug, Clone, Copy)]
pub struct PiecePath {
    pub pawn: &'static str,
    pub knight: &'static str,
    pub bishop: &'static str,
    pub rook: &'static str,
    pub queen: &'static str,
    pub king: &'static str,
}
