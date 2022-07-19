use crate::board::components::{Board, File, Location, Piece, PieceColor, PieceType, Square};
use crate::board::utils;
use crate::constants::{
    BOARD_HEIGHT, BOARD_LEGEND_FONT_SIZE, BOARD_PADDING, BOARD_WIDTH, PIECE_Z_AXIS, SQUARE_Z_AXIS,
};
use crate::resources::{DefaultFont, PieceTheme};
use crate::{SQUARE_SIZE, WINDOW_HEIGHT};
use bevy::prelude::*;
use bevy_svg::prelude::*;

/// Sets up the board, all squares and the default position for pieces
pub fn setup_board(mut commands: Commands, font: Res<DefaultFont>, piece_theme: Res<PieceTheme>) {
    let board_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::RED, // TODO: remove me
            custom_size: Some(Vec2::new(BOARD_WIDTH, BOARD_HEIGHT)),
            ..default()
        },
        ..default()
    };
    draw_vertical_legend(&mut commands, &font);
    draw_horizontal_legend(&mut commands, &font);
    commands
        .spawn_bundle(board_bundle)
        .with_children(|parent| {
            draw_squares(parent, &piece_theme);
        })
        .insert(Board);
}

/// Draws the file notation as horizontal legend
fn draw_horizontal_legend(commands: &mut Commands, font: &Res<DefaultFont>) {
    let center_offset = utils::center_offset();
    let parent_bundle = NodeBundle {
        style: Style {
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            position: Rect {
                bottom: Val::Px((WINDOW_HEIGHT - BOARD_HEIGHT) / 4.0),
                left: Val::Px(center_offset),
                ..default()
            },
            flex_direction: FlexDirection::Row,
            size: Size::new(Val::Px(BOARD_WIDTH), Val::Px(center_offset)),
            ..default()
        },
        visibility: Visibility { is_visible: false },
        ..default()
    };

    let text_style = TextStyle {
        font: font.0.clone(),
        font_size: BOARD_LEGEND_FONT_SIZE,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(parent_bundle)
        .with_children(|parent| {
            for y in 0..8 {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        format!("{}", File::from_index(y)),
                        text_style.clone(),
                        TextAlignment::default(),
                    ),
                    ..default()
                });
            }
        });
}

/// Draws the rank notation as vertical legend
fn draw_vertical_legend(commands: &mut Commands, font: &Res<DefaultFont>) {
    let center_offset = utils::center_offset();
    let parent_bundle = NodeBundle {
        style: Style {
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            position: Rect {
                bottom: Val::Px(SQUARE_SIZE),
                left: Val::Px((WINDOW_HEIGHT - BOARD_HEIGHT) / 4.0),
                ..default()
            },
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Px(center_offset), Val::Px(BOARD_HEIGHT)),
            ..default()
        },
        visibility: Visibility { is_visible: false },
        ..default()
    };

    let text_style = TextStyle {
        font: font.0.clone(),
        font_size: BOARD_LEGEND_FONT_SIZE,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(parent_bundle)
        .with_children(|parent| {
            for x in 0..8 {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        format!("{}", utils::rank_for_index(x)),
                        text_style.clone(),
                        TextAlignment::default(),
                    ),
                    ..default()
                });
            }
        });
}

/// Draws up all squares as children for the given `board`
fn draw_squares(board: &mut ChildBuilder, piece_theme: &Res<PieceTheme>) {
    let board_offset = BOARD_WIDTH / 2.0 - utils::center_offset() + BOARD_PADDING;
    // iterate over files
    for x in 0..8 {
        // iterate over ranks
        for y in 0..8 {
            let transform = Transform::from_xyz(
                x as f32 * SQUARE_SIZE - board_offset,
                y as f32 * SQUARE_SIZE - board_offset,
                SQUARE_Z_AXIS,
            );
            let square_bundle = SpriteBundle {
                sprite: Sprite {
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

    let center_offset = utils::center_offset();
    let transform = Transform {
        translation: Vec3::new(center_offset * -1.0, center_offset, PIECE_Z_AXIS),
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
