use crate::board::components::{
    Board, File, Location, Piece, PieceColor, PieceType, SelectedPiece, Square,
};
use crate::board::events::MovePieceEvent;
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

/// This system picks up a piece if `MouseButton::Left` has just been pressed on a square
pub fn left_click_piece_selection(
    mut commands: Commands,
    squares_q: Query<(&Children, &GlobalTransform), With<Square>>,
    cameras_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // only consider piece selection if left mouse button was just pressed
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));
    for (children, transform) in squares_q.iter() {
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
            break;
        }
    }
}

/// This system will trigger a `MovePieceEvent` if `MouseButton::Left` has just been released
pub fn left_click_piece_release(
    mut pieces_q: Query<&Parent, (Without<Square>, Without<MainCamera>)>,
    squares_q: Query<(Entity, &GlobalTransform), With<Square>>,
    cameras_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    selected_piece: Option<Res<SelectedPiece>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut event_writer: EventWriter<MovePieceEvent>,
) {
    // only consider piece deselection if left mouse button was just released
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    let piece = some_or_return!(selected_piece).0;

    // translate position and check if cursor is on a valid square
    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));
    for (new_square, square_transform) in squares_q.iter() {
        if utils::intersects_square(&cursor, &square_transform.translation) {
            let old_square = ok_or_return!(pieces_q.get_mut(piece));
            event_writer.send(MovePieceEvent::new(old_square.0, new_square, piece));
            break;
        }
    }
}

/// Draws `SelectedPiece` at the cursor position
pub fn draw_selected_piece(
    mut pieces_q: Query<&mut GlobalTransform, Without<MainCamera>>,
    cameras_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    selected_piece: Option<Res<SelectedPiece>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // only consider piece movements if mouse button is pressed
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let piece = some_or_return!(selected_piece).0;
    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));
    let mut transform = ok_or_return!(pieces_q.get_mut(piece));

    // stick piece to cursor and clamp it to board size
    let board_offset = BOARD_WIDTH / 2.0;
    let center_offset = utils::center_offset();
    let left_bound = (board_offset + center_offset) * -1.0;
    let right_bound = board_offset - center_offset;
    transform.translation.x = (cursor.x - center_offset).clamp(left_bound, right_bound);
    transform.translation.y = (cursor.y + center_offset).clamp(left_bound, right_bound);

    // increase z axis so that selected piece is always in foreground
    transform.translation.z = PIECE_Z_AXIS * 2.0;
}

/// Handles `MovePieceEvent` and makes all checks necessary if this is a legal move
pub fn handle_move_piece_events(
    mut commands: Commands,
    mut pieces_q: Query<(&mut GlobalTransform, &mut Piece), Without<Square>>,
    squares_q: Query<(&GlobalTransform, &Location), With<Square>>,
    mut events: EventReader<MovePieceEvent>,
) {
    for event in events.iter() {
        // check if square is blocked by same color piece

        // switch parent square to place piece
        commands.entity(event.from).remove_children(&[event.piece]);
        commands.entity(event.to).add_child(event.piece);

        let (mut piece_transform, mut piece_comp) = ok_or_return!(pieces_q.get_mut(event.piece));
        let (ns_transform, ns_loc) = ok_or_return!(squares_q.get(event.to));

        // adjust piece transform to match new square
        let center_offset = utils::center_offset();
        piece_transform.translation.x = ns_transform.translation.x - center_offset;
        piece_transform.translation.y = ns_transform.translation.y + center_offset;
        piece_transform.translation.z = PIECE_Z_AXIS;
        piece_comp.has_moved = true;
        commands.remove_resource::<SelectedPiece>();
        println!("{}{}", *piece_comp, ns_loc);
    }
}