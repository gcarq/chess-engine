use crate::board::components::{Selected, Square};
use crate::board::events::{CheckedPieceMoveEvent, MoveTarget, UncheckedPieceMoveEvent};
use crate::board::{utils, SelectedPiece};
use crate::{ok_or_return, some_or_return, MainCamera};
use bevy::prelude::*;

/// This system picks up a piece if `MouseButton::Left` has just been pressed on a square
pub fn left_click_piece_selection(
    mut commands: Commands,
    squares_q: Query<(Entity, &Children, &GlobalTransform), With<Square>>,
    cameras_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    // only consider piece selection if left mouse button was just pressed
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));
    for (square_entity, square_children, square_transform) in squares_q.iter() {
        // only consider squares that have pieces on
        if square_children.len() == 0 {
            continue;
        }

        // find current piece and set it as currently selected
        if utils::intersects_square(&cursor, &square_transform.translation) {
            assert_eq!(
                square_children.len(),
                1,
                "there are multiple pieces on the same square"
            );
            let selected_piece_entity = square_children[0];
            commands.entity(selected_piece_entity).insert(Selected);
            commands.entity(square_entity).insert(Selected);
            commands.insert_resource(SelectedPiece::new(square_entity, selected_piece_entity));
            break;
        }
    }
}

/// This system will trigger a `UncheckedPieceMoveEvent` if `MouseButton::Left` has just been released
pub fn left_click_piece_release(
    squares_q: Query<(Entity, &GlobalTransform), With<Square>>,
    cameras_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    selected_piece: Option<Res<SelectedPiece>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut unchecked_moves: EventWriter<UncheckedPieceMoveEvent>,
    mut checked_moves: EventWriter<CheckedPieceMoveEvent>,
) {
    // only consider piece deselection if left mouse button was just released
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }

    let selected = some_or_return!(selected_piece);
    let piece_entity = selected.piece;
    let os_entity = selected.square;

    // translate position and get all required data
    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));

    // if mouse was released on same square we don't send a move event
    let (os_entity, os_transform) = ok_or_return!(squares_q.get(os_entity));
    if utils::intersects_square(&cursor, &os_transform.translation) {
        checked_moves.send(CheckedPieceMoveEvent::new(
            piece_entity,
            os_entity,
            MoveTarget::None,
        ));
        return;
    }

    for (ns_entity, ns_transform) in squares_q.iter() {
        if utils::intersects_square(&cursor, &ns_transform.translation) {
            unchecked_moves.send(UncheckedPieceMoveEvent::new(
                piece_entity,
                os_entity,
                ns_entity,
            ));
            return;
        }
    }

    // if cursor doesnt intersect a valid square we assume it is out of bound
    checked_moves.send(CheckedPieceMoveEvent::new(
        piece_entity,
        os_entity,
        MoveTarget::OutOfBound,
    ));
}
