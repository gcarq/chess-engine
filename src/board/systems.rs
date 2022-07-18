use crate::board::components::{
    File, Location, Piece, PieceColor, PieceType, SelectedPiece, Square,
};
use crate::board::utils;
use crate::constants::{
    BOARD_HEIGHT, BOARD_LEGEND_FONT_SIZE, BOARD_PADDING, BOARD_WIDTH, PIECE_Z_AXIS, SQUARE_Z_AXIS,
};
use crate::resources::{DefaultFont, PieceTheme};
use crate::{ok_or_return, some_or_return, MainCamera, SQUARE_SIZE, WINDOW_HEIGHT};
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
    commands.spawn_bundle(board_bundle).with_children(|parent| {
        draw_squares(parent, &piece_theme);
    });
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
                    color: utils::square_color(x, y).color(),
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

/// This system is responsible for piece selection
pub fn piece_selection(
    mut commands: Commands,
    squares: Query<(&Children, &Location, &GlobalTransform), With<Square>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // only consider piece selection if left mouse button was just pressed
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor = some_or_return!(utils::translate_cursor_pos(cameras, windows));
    for (children, location, transform) in squares.iter() {
        // only consider squares that have pieces on
        if children.len() == 0 {
            continue;
        }
        // find current piece and set it as currently selected
        if utils::intersects_square(&cursor, &transform.translation) {
            assert_eq!(
                children.len(),
                1,
                "there are multiple pieces on the same square"
            );
            commands.insert_resource(SelectedPiece(children[0]));
            println!("selected piece at {}", location);
            break;
        }
    }
}

/// This system is responsible for tracking the current selected piece
pub fn piece_deselection(
    mut commands: Commands,
    selected_piece: Option<Res<SelectedPiece>>,
    mut pieces: Query<
        (&Parent, &mut GlobalTransform),
        (With<Piece>, Without<Square>, Without<MainCamera>),
    >,
    squares: Query<(Entity, &Location, &GlobalTransform), With<Square>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // only consider piece deselection if left mouse button was just released
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    let piece = some_or_return!(selected_piece);
    let (old_square, mut piece_transform) = ok_or_return!(pieces.get_mut(piece.0));
    // restore original z axis value
    piece_transform.translation.z = PIECE_Z_AXIS;
    // translate position and check if cursor is on a valid square
    let cursor = some_or_return!(utils::translate_cursor_pos(cameras, windows));
    for (new_square, location, square_transform) in squares.iter() {
        if utils::intersects_square(&cursor, &square_transform.translation) {
            // switch parent square to place piece
            commands.entity(old_square.0).remove_children(&[piece.0]);
            commands.entity(new_square).add_child(piece.0);
            // adjust transform of piece
            let center_offset = utils::center_offset();
            piece_transform.translation.x = square_transform.translation.x - center_offset;
            piece_transform.translation.y = square_transform.translation.y + center_offset;
            commands.remove_resource::<SelectedPiece>();
            println!("placed piece at {}", location);
            break;
        }
    }
}

/// This system is responsible for tracking the current selected piece
pub fn handle_piece_movement(
    selected_piece: Option<Res<SelectedPiece>>,
    mut pieces: Query<&mut GlobalTransform, Without<MainCamera>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // only consider piece movements if mouse button is pressed
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let piece = some_or_return!(selected_piece);
    let cursor = some_or_return!(utils::translate_cursor_pos(cameras, windows));
    println!("cursor: {:?}", cursor);
    let mut transform = ok_or_return!(pieces.get_mut(piece.0));
    let center_offset = utils::center_offset();
    // stick piece to cursor and clamp it to board size
    let board_offset = BOARD_WIDTH / 2.0;
    let left_bound = (board_offset + center_offset) * -1.0;
    let right_bound = board_offset - center_offset;
    transform.translation.x = (cursor.x - center_offset).clamp(left_bound, right_bound);
    transform.translation.y = (cursor.y + center_offset).clamp(left_bound, right_bound);

    // increase z axis so that selected piece is always in foreground
    transform.translation.z = PIECE_Z_AXIS * 2.0;
}
