pub mod input;
pub mod startup;

use crate::board::components::{Location, Piece, Selected, Square};
use crate::board::events::{
    CheckedPieceMoveEvent, MoveTarget, PieceSelectionEvent, UncheckedPieceMoveEvent,
};
use crate::board::utils::square_color;
use crate::board::{utils, SelectedPiece};
use crate::constants::{BOARD_WIDTH, PIECE_Z_AXIS};
use crate::{ok_or_return, some_or_return, MainCamera};
use bevy::prelude::*;

/// Draws `Square` based on their components
pub fn handle_square_updates(
    mut selected_squares_q: Query<(&mut Sprite, &Location), (With<Square>, With<Selected>)>,
    mut normal_squares_q: Query<(&mut Sprite, &Location), (With<Square>, Without<Selected>)>,
) {
    for (mut sprite, location) in selected_squares_q.iter_mut() {
        sprite.color = square_color(location.x, location.y).selected();
    }

    for (mut sprite, location) in normal_squares_q.iter_mut() {
        sprite.color = square_color(location.x, location.y).default();
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
pub fn verify_unchecked_moves(
    pieces_q: Query<&Piece, Without<Selected>>,
    square_q: Query<&Children, With<Square>>,
    selected_q: Query<&Piece, With<Selected>>,
    mut unchecked_moves: EventReader<UncheckedPieceMoveEvent>,
    mut checked_moves: EventWriter<CheckedPieceMoveEvent>,
    mut piece_selections: EventWriter<PieceSelectionEvent>,
) {
    for event in unchecked_moves.iter() {
        // check if new square is blocked by a same color piece
        let selected_comp = ok_or_return!(selected_q.get(event.piece));
        let ns_children = ok_or_return!(square_q.get(event.target));
        if ns_children.len() > 0 {
            let piece_comp = ok_or_return!(pieces_q.get(ns_children[0]));
            if piece_comp.color == selected_comp.color {
                piece_selections.send(PieceSelectionEvent::Reselect(ns_children[0]));
                continue;
            }
        }

        checked_moves.send(CheckedPieceMoveEvent::legal(event));
    }
}

/// Handles `PieceSelectionEvent` events
pub fn handle_piece_selection(
    mut commands: Commands,
    selected_squares_q: Query<Entity, (With<Square>, With<Selected>)>,
    piece_q: Query<&Parent, With<Piece>>,
    selected_piece: Option<Res<SelectedPiece>>,
    mut selection_events: EventReader<PieceSelectionEvent>,
) {
    for event in selection_events.iter() {
        match event {
            PieceSelectionEvent::Selected(piece) => {
                assert!(selected_piece.is_none());
                println!("Handling piece selection for {:?}", piece);
                // remove `Selected` component from all squares to clear previous move
                selected_squares_q.for_each(|square| {
                    commands.entity(square).remove::<Selected>();
                });
                let square = ok_or_return!(piece_q.get(*piece)).0;
                commands.entity(square).insert(Selected);
                commands.entity(*piece).insert(Selected);
                commands.insert_resource(SelectedPiece::new(square, *piece));
            }
            PieceSelectionEvent::Deselected(piece) => {
                assert!(selected_piece.is_some());
                println!("Handling piece deselection for {:?}", piece);
                let square = ok_or_return!(piece_q.get(*piece)).0;
                commands.entity(square).remove::<Selected>();
                utils::deselect_piece(&mut commands, *piece);
            }
            PieceSelectionEvent::Reselect(piece) => {
                let selected = selected_piece.as_ref().expect("selected piece must be set");
                println!("Handling piece reselection for {:?}", piece);
                // remove `Selected` component from all squares to clear previous move
                selected_squares_q.for_each(|square| {
                    commands.entity(square).remove::<Selected>();
                });
                utils::deselect_piece(&mut commands, selected.piece);
                let square = ok_or_return!(piece_q.get(*piece)).0;
                commands.entity(square).insert(Selected);
                commands.entity(*piece).insert(Selected);
                commands.insert_resource(SelectedPiece::new(square, *piece));
            }
        }
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
            // if a legal move occurs we deselect the piece, but leave the source and target squares
            // as selected until a new move begins
            MoveTarget::Legal(target) => {
                println!("legal move");
                utils::switch_square(&mut commands, event.piece, event.source, target);
                selected_piece.has_moved = true;
                commands.entity(target).insert(Selected);
                utils::deselect_piece(&mut commands, event.piece);
                ok_or_return!(square_q.get(target))
            }
            // if a illegal move occurs we want to
            // deselect the piece and move it to the source square
            MoveTarget::Illegal => {
                println!("illegal move");
                utils::deselect_piece(&mut commands, event.piece);
                ok_or_return!(square_q.get(event.source))
            }
        };

        selected_tf.translation.z = PIECE_Z_AXIS;
        utils::adjust_to_square(&mut selected_tf, target_square);
    }
}
