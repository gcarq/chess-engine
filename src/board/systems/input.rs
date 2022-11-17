use crate::board::components::Square;
use crate::board::events::{PieceSelectionEvent, UncheckedPieceMoveEvent};
use crate::board::{utils, CurrentPlayer, SelectedPiece};
use crate::{some_or_return, BoardCamera, Location, Piece};
use bevy::prelude::*;

/// This system handles `MouseButton::Left` input and fires required events
pub fn left_click_piece_selection(
    pieces_q: Query<&Piece>,
    squares_q: Query<(Entity, &Children, &Location, &GlobalTransform), With<Square>>,
    cameras_q: Query<(&Camera, &GlobalTransform), With<BoardCamera>>,
    selected_piece: Option<Res<SelectedPiece>>,
    mouse_button_input: Res<Input<MouseButton>>,
    current_player: Res<CurrentPlayer>,
    windows: Res<Windows>,
    mut piece_selection_writer: EventWriter<PieceSelectionEvent>,
    mut moves_writer: EventWriter<UncheckedPieceMoveEvent>,
) {
    // only consider piece selection if left mouse button was just pressed
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor = some_or_return!(utils::translate_cursor_pos(cameras_q, windows));
    for (square_entity, square_children, square_location, square_transform) in squares_q.iter() {
        if !utils::intersects_square(&cursor, &square_transform.translation()) {
            continue;
        }
        println!("DEBUG: clicked on {}", square_location);
        let piece = utils::resolve_piece(square_children, &pieces_q);
        match selected_piece {
            Some(selected) => {
                // consider it a piece deselection if the same square was clicked again
                if square_entity == selected.square {
                    piece_selection_writer
                        .send(PieceSelectionEvent::Deselected(some_or_return!(piece).0));
                    break;
                }

                moves_writer.send(UncheckedPieceMoveEvent::new(
                    selected.clone(),
                    square_entity,
                ));
            }
            None => {
                let (entity, comp) = some_or_return!(piece);
                if current_player.0 != comp.color {
                    println!("INFO: It's {}'s turn", current_player.0);
                    break;
                }
                piece_selection_writer.send(PieceSelectionEvent::Selected(entity));
            }
        }

        break;
    }
}
