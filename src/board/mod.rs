use std::fmt;

use crate::board::pieces::{Piece, PieceColor, PieceType};
use crate::constants::{
    BOARD_HEIGHT, BOARD_PADDING, BOARD_WIDTH, SQUARE_COLOR_DARK, SQUARE_COLOR_LIGHT,
};
use crate::{PieceTheme, SQUARE_SIZE};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_svg::prelude::Svg2dBundle;

pub mod pieces;

#[derive(Component, Inspectable, Copy, Clone)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        assert!(x < 9);
        assert!(y < 9);
        Self { x, y }
    }

    pub fn from_notation(file: File, rank: usize) -> Self {
        Self::new(file.to_index(), rank - 1)
    }
}

#[derive(Component)]
pub struct Square;

pub enum SquareColor {
    Light,
    Dark,
}

impl SquareColor {
    pub fn color(&self) -> Color {
        match &self {
            SquareColor::Light => SQUARE_COLOR_LIGHT,
            SquareColor::Dark => SQUARE_COLOR_DARK,
        }
    }
}

/// A column of the chessboard.
/// A specific file can be named either using its position in algebraic notation,
/// a–h, or by using its position in descriptive notation.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn to_index(&self) -> usize {
        match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        };
        write!(f, "{}", char)
    }
}

#[derive(Component)]
pub struct Board;

/// Returns the correct File for the given vector index
pub fn file_for_index(index: usize) -> File {
    assert!(index < 8);
    match index {
        0 => File::A,
        1 => File::B,
        2 => File::C,
        3 => File::D,
        4 => File::E,
        5 => File::F,
        6 => File::G,
        7 => File::H,
        _ => unimplemented!(),
    }
}

/// Returns the correct Rank for the given vector index
pub fn rank_for_index(index: usize) -> usize {
    assert!(index < 8);
    index + 1
}

/// Returns the color for the given indicates
pub fn square_color(x: usize, y: usize) -> SquareColor {
    if (x + y + 1) % 2 == 0 {
        SquareColor::Light
    } else {
        SquareColor::Dark
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board);
    }
}

/// Sets up the board, all squares and the default position for pieces
fn setup_board(mut commands: Commands, piece_theme: Res<PieceTheme>) {
    let board_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::RED, // TODO: remove me
            custom_size: Some(Vec2::new(BOARD_WIDTH, BOARD_HEIGHT)),
            ..default()
        },
        ..default()
    };
    commands.spawn_bundle(board_bundle).with_children(|parent| {
        setup_squares(parent, &piece_theme);
    });
}

/// Sets up all squares as children for the given `board`
fn setup_squares(board: &mut ChildBuilder, piece_theme: &Res<PieceTheme>) {
    let center_offset = BOARD_WIDTH / 2.0 - SQUARE_SIZE / 2.0 + BOARD_PADDING;
    for x in 0..8 {
        for y in 0..8 {
            let transform = Transform::from_xyz(
                x as f32 * SQUARE_SIZE - center_offset,
                y as f32 * SQUARE_SIZE - center_offset,
                10.0,
            );
            let square_bundle = SpriteBundle {
                sprite: Sprite {
                    color: square_color(x, y).color(),
                    custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                    ..default()
                },
                transform,
                ..default()
            };
            let location = Location::new(x, y);
            board
                .spawn_bundle(square_bundle)
                .insert(Square)
                .insert(location)
                .with_children(|parent| {
                    place_piece(parent, location, piece_theme);
                });
        }
    }
}

/// Places a `Piece` on the given position as a direct child of the given `parent`.
fn place_piece(square: &mut ChildBuilder, location: Location, piece_theme: &Res<PieceTheme>) {
    let piece = match location {
        // first rank
        Location { x: 0, y: 0 } => Piece::new(PieceType::Rook, PieceColor::White),
        Location { x: 1, y: 0 } => Piece::new(PieceType::Knight, PieceColor::White),
        Location { x: 2, y: 0 } => Piece::new(PieceType::Bishop, PieceColor::White),
        Location { x: 3, y: 0 } => Piece::new(PieceType::Queen, PieceColor::White),
        Location { x: 4, y: 0 } => Piece::new(PieceType::King, PieceColor::White),
        Location { x: 5, y: 0 } => Piece::new(PieceType::Bishop, PieceColor::White),
        Location { x: 6, y: 0 } => Piece::new(PieceType::Knight, PieceColor::White),
        Location { x: 7, y: 0 } => Piece::new(PieceType::Rook, PieceColor::White),
        // second rank
        Location { y: 1, .. } => Piece::new(PieceType::Pawn, PieceColor::White),
        // seventh rank
        Location { y: 6, .. } => Piece::new(PieceType::Pawn, PieceColor::Black),
        // eight rank
        Location { x: 0, y: 7 } => Piece::new(PieceType::Rook, PieceColor::Black),
        Location { x: 1, y: 7 } => Piece::new(PieceType::Knight, PieceColor::Black),
        Location { x: 2, y: 7 } => Piece::new(PieceType::Bishop, PieceColor::Black),
        Location { x: 3, y: 7 } => Piece::new(PieceType::Queen, PieceColor::Black),
        Location { x: 4, y: 7 } => Piece::new(PieceType::King, PieceColor::Black),
        Location { x: 5, y: 7 } => Piece::new(PieceType::Bishop, PieceColor::Black),
        Location { x: 6, y: 7 } => Piece::new(PieceType::Knight, PieceColor::Black),
        Location { x: 7, y: 7 } => Piece::new(PieceType::Rook, PieceColor::Black),
        Location { .. } => return,
    };

    let svg = piece_theme
        .vectors
        .get(&piece.resource_name())
        .cloned()
        .unwrap();

    let center_offset = SQUARE_SIZE / 2.0;
    let transform = Transform {
        translation: Vec3::new(center_offset * -1.0, center_offset, 20.0),
        scale: Vec3::new(1.6, 1.6, 0.0),
        ..default()
    };
    let piece_bundle = Svg2dBundle {
        svg,
        transform,
        ..default()
    };
    square
        .spawn_bundle(piece_bundle)
        .insert(piece)
        .insert(location);
}
