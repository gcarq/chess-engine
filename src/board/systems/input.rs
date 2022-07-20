use crate::board::components::Square;
use crate::board::events::{PieceSelectionEvent, UncheckedPieceMoveEvent};
use crate::board::{utils, SelectedPiece};
use crate::{some_or_return, Location, MainCamera};
use bevy::prelude::*;

/// This system handles `MouseButton::Left` input and fires required events
pub fn left_click_piece_selection(
    squares_q: Query<(Entity, &Children, &Location, &GlobalTransform), With<Square>>,
    cameras_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    selected_piece: Option<Res<SelectedPiece>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut piece_selections: EventWriter<PieceSelectionEvent>,
    mut unchecked_moves: EventWriter<UncheckedPieceMoveEvent>,
) {
    // only consider piece selection if left mouse button was just pressed
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));
    for (square_entity, square_children, square_location, square_transform) in squares_q.iter() {
        if !utils::intersects_square(&cursor, &square_transform.translation) {
            continue;
        }
        println!("clicked on {}", square_location);

        match selected_piece {
            Some(selected) => {
                // consider it a piece deselection if the same square was clicked again
                if square_entity == selected.square {
                    piece_selections.send(PieceSelectionEvent::Deselected(square_children[0]));
                    break;
                }
                unchecked_moves.send(UncheckedPieceMoveEvent::new(
                    selected.piece,
                    selected.square,
                    square_entity,
                ));
            }
            None => {
                if square_children.len() != 0 {
                    piece_selections.send(PieceSelectionEvent::Selected(square_children[0]));
                }
            }
        }

        break;
    }
}
