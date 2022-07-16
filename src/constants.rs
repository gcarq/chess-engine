use bevy::prelude::Color;

pub const WINDOW_HEIGHT: f32 = 700.0;
pub const WINDOW_WIDTH: f32 = 700.0;

pub const BOARD_HEIGHT: f32 = 640.0;
pub const BOARD_WIDTH: f32 = BOARD_HEIGHT;
pub const BOARD_PADDING: f32 = (BOARD_WIDTH - SQUARE_SIZE * 8.0) / 2.0;

pub const SQUARE_SIZE: f32 = 80.0;
pub const SQUARE_COLOR_LIGHT: Color = Color::rgb(0.87, 0.89, 0.90);
pub const SQUARE_COLOR_DARK: Color = Color::rgb(0.55, 0.64, 0.68);
