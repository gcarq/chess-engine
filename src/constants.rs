use bevy::prelude::Color;

pub const WINDOW_HEIGHT: f32 = 800.0;
pub const WINDOW_WIDTH: f32 = 800.0;

pub const BOARD_HEIGHT: f32 = 640.0;
pub const BOARD_WIDTH: f32 = BOARD_HEIGHT;
pub const BOARD_PADDING: f32 = (BOARD_WIDTH - SQUARE_SIZE * 8.0) / 2.0;
pub const BOARD_LEGEND_FONT_SIZE: f32 = 30.0;

pub const SQUARE_Z_AXIS: f32 = 10.0;
pub const SQUARE_SIZE: f32 = 80.0;

pub const SQUARE_COLOR_LIGHT_DEFAULT: Color = Color::rgb(0.87, 0.89, 0.90);
pub const SQUARE_COLOR_DARK_DEFAULT: Color = Color::rgb(0.55, 0.64, 0.68);

pub const SQUARE_COLOR_LIGHT_SELECTED: Color = Color::rgb(0.96, 0.96, 0.41);
pub const SQUARE_COLOR_DARK_SELECTED: Color = Color::rgb(0.73, 0.79, 0.16);

pub const SQUARE_COLOR_POSSIBLE_TARGET: Color = Color::CYAN;

pub const PIECE_Z_AXIS: f32 = 20.0;
pub const PIECE_THEME: &str = "merida";
