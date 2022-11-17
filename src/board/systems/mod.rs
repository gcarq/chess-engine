pub mod input;
pub mod selection;
pub mod startup;

use crate::board::components::{Location, Piece, PossibleTarget, Selected, Square};
use crate::board::events::{
    CheckedPieceMoveEvent, MoveTarget, PieceSelectionEvent, PlayedMoveEvent,
    UncheckedPieceMoveEvent,
};
use crate::board::utils::square_color;
use crate::board::{utils, CurrentPlayer, PlayedMove, PlayedMoves};
use crate::constants::PIECE_Z_AXIS;
use crate::ok_or_return;
use bevy::prelude::*;

/// Draws `Square` based on their components
pub fn handle_square_status_updates(
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

/// Handles `UncheckedPieceMoveEvent` and checks if this is a legal move
pub fn handle_unchecked_move_events(
    pieces_q: Query<&Piece>,
    square_q: Query<(&Children, &Location), With<Square>>,
    mut unchecked_moves: EventReader<UncheckedPieceMoveEvent>,
    mut checked_moves: EventWriter<CheckedPieceMoveEvent>,
    mut piece_selections: EventWriter<PieceSelectionEvent>,
) {
    for event in unchecked_moves.iter() {
        let (ns_children, ns_location) = ok_or_return!(square_q.get(event.target));

        // check if new square is occupied by a same color piece
        if let Some((piece_entity, piece_comp)) = utils::resolve_piece(ns_children, &pieces_q) {
            if piece_comp.color == event.selected.piece_comp.color {
                piece_selections.send(PieceSelectionEvent::Reselect(piece_entity));
                continue;
            }
        }

        // check if target square makes up a legal move
        if event.selected.possible_targets.contains(ns_location) {
            checked_moves.send(CheckedPieceMoveEvent::legal(event));
        } else {
            checked_moves.send(CheckedPieceMoveEvent::illegal(event));
        }
    }
}

/// Handles `CheckedPieceMoveEvent`
pub fn handle_checked_move_events(
    mut commands: Commands,
    location_q: Query<&Location>,
    square_q: Query<&GlobalTransform, With<Square>>,
    possible_targets_q: Query<Entity, With<PossibleTarget>>,
    mut selected_q: Query<&mut GlobalTransform, (With<Selected>, Without<Square>)>,
    mut current_player: ResMut<CurrentPlayer>,
    mut checked_moves_reader: EventReader<CheckedPieceMoveEvent>,
    mut played_moves_writer: EventWriter<PlayedMoveEvent>,
) {
    for event in checked_moves_reader.iter() {
        let mut selected_tf = ok_or_return!(selected_q.get_mut(event.selected.piece));

        let target_square = match event.target {
            // if a legal move occurs we deselect the piece, but leave the source and target squares
            // as selected until a new move begins
            MoveTarget::Legal(target) => {
                let loc_comp = *location_q.get(target).unwrap();
                utils::update_entity_for_move(&mut commands, event, target, loc_comp);

                // trigger event that this move has been played
                played_moves_writer.send(PlayedMoveEvent(PlayedMove::from_event(
                    event, target, loc_comp,
                )));

                current_player.switch();

                commands.entity(target).insert(Selected);
                utils::deselect_piece(&mut commands, event.selected.piece);
                square_q.get(target).unwrap()
            }
            // if a illegal move occurs we want to
            // deselect the piece and move it to the source square
            MoveTarget::Illegal => {
                println!("illegal move");
                utils::deselect_piece(&mut commands, event.selected.piece);
                square_q.get(event.selected.square).unwrap()
            }
        };

        possible_targets_q.for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });

        selected_tf.translation_mut().z = PIECE_Z_AXIS;
        utils::adjust_to_square(&mut selected_tf, target_square);
    }
}

/// Handles `PlayedMoveEvent` to display them
pub fn record_played_moves(
    mut played_moves: ResMut<PlayedMoves>,
    mut moves_reader: EventReader<PlayedMoveEvent>,
) {
    for event in moves_reader.iter() {
        played_moves.0.push(event.0);
        println!("DEBUG: {}", event.0.notation());
    }
}
