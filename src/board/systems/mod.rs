pub mod input;
pub mod startup;

use crate::board::components::{LegalSquare, Location, Piece, Selected, Square};
use crate::board::events::{CheckedPieceMoveEvent, MoveTarget, UncheckedPieceMoveEvent};
use crate::board::utils::square_color;
use crate::board::{utils, SelectedPiece};
use crate::constants::{BOARD_WIDTH, PIECE_Z_AXIS};
use crate::{ok_or_return, some_or_return, MainCamera};
use bevy::prelude::*;

/// Draws `Square`
pub fn handle_square_updates(
    mut legal_squares_q: Query<&mut Sprite, (With<Square>, With<LegalSquare>)>,
    mut normal_squares_q: Query<(&mut Sprite, &Location), (With<Square>, Without<LegalSquare>)>,
) {
    for mut sprite in legal_squares_q.iter_mut() {
        sprite.color = Color::rgba(0.5, 0.0, 0.0, 0.1);
    }

    for (mut sprite, location) in normal_squares_q.iter_mut() {
        sprite.color = square_color(location.x, location.y).raw();
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

    let piece_entity = some_or_return!(selected_piece).piece;
    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));
    let mut piece_transform = ok_or_return!(pieces_q.get_mut(piece_entity));

    // stick piece to cursor and clamp it to board size
    let board_offset = BOARD_WIDTH / 2.0;
    let center_offset = utils::center_offset();
    piece_transform.translation.x = (cursor.x - center_offset).clamp(
        (board_offset + center_offset) * -1.0,
        board_offset - center_offset,
    );
    piece_transform.translation.y = (cursor.y + center_offset).clamp(
        (board_offset - center_offset) * -1.0,
        board_offset + center_offset,
    );

    // increase z axis so that selected piece is always in foreground
    piece_transform.translation.z = PIECE_Z_AXIS * 2.0;
}

/// Handles `UncheckedPieceMoveEvent` and checks if this is a legal move
pub fn check_move_legality(
    pieces_q: Query<&Piece, Without<Selected>>,
    square_q: Query<&Children, With<Square>>,
    selected_q: Query<&Piece, With<Selected>>,
    mut unchecked_moves: EventReader<UncheckedPieceMoveEvent>,
    mut checked_moves: EventWriter<CheckedPieceMoveEvent>,
) {
    for event in unchecked_moves.iter() {
        // check if new square is blocked by a same color piece
        let selected_comp = ok_or_return!(selected_q.get(event.piece));
        let ns_children = ok_or_return!(square_q.get(event.target));
        if ns_children.len() > 0 {
            let piece_comp = ok_or_return!(pieces_q.get(ns_children[0]));
            if piece_comp.color == selected_comp.color {
                checked_moves.send(CheckedPieceMoveEvent::illegal(event));
                continue;
            }
        }

        checked_moves.send(CheckedPieceMoveEvent::legal(event));
    }
}

/// Handles `CheckedPieceMoveEvent`
pub fn handle_checked_move_events(
    mut commands: Commands,
    square_q: Query<&GlobalTransform, With<Square>>,
    mut selected_q: Query<(&mut GlobalTransform, &mut Piece), (With<Selected>, Without<Square>)>,
    mut events: EventReader<CheckedPieceMoveEvent>,
) {
    for event in events.iter() {
        let (mut selected_tf, mut selected_piece) = ok_or_return!(selected_q.get_mut(event.piece));

        let target_square = match event.target {
            MoveTarget::Legal(target) => {
                // switch parent square to place piece
                commands
                    .entity(event.source)
                    .remove_children(&[event.piece]);
                commands.entity(target).add_child(event.piece);
                selected_piece.has_moved = true;
                ok_or_return!(square_q.get(target))
            }
            MoveTarget::Illegal => {
                println!("illegal move");
                ok_or_return!(square_q.get(event.source))
            }
            MoveTarget::OutOfBound => {
                println!("out of bound move");
                ok_or_return!(square_q.get(event.source))
            }
            MoveTarget::None => {
                ok_or_return!(square_q.get(event.source))
            }
        };

        selected_tf.translation.z = PIECE_Z_AXIS;
        utils::adjust_to_square(&mut selected_tf, target_square);
        utils::deselect_piece(&mut commands, event.piece);
    }
}
