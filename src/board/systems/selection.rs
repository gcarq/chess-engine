use crate::board::components::{Piece, PossibleTarget, Selected, Square};
use crate::board::events::PieceSelectionEvent;
use crate::board::{utils, SelectedPiece};
use crate::{ok_or_return, Location};
use bevy::prelude::*;

fn mark_possible_moves(
    commands: &mut Commands,
    moves: &[Location],
    query: &Query<(Entity, &Location), With<Square>>,
) {
    for (entity, location) in query.iter() {
        if moves.contains(location) {
            commands.entity(entity).insert(PossibleTarget);
        }
    }
}

/// Handles `PieceSelectionEvent` events
pub fn handle_piece_selection_events(
    world: &World,
    mut commands: Commands,
    selected_squares_q: Query<Entity, (With<Square>, With<Selected>)>,
    possible_target_squares_q: Query<Entity, (With<Square>, With<PossibleTarget>)>,
    squares_q: Query<(Entity, &Location), With<Square>>,
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
                let selected =
                    SelectedPiece::new(square, *piece, world).expect("unable to select piece");

                // mark squares and piece
                mark_possible_moves(&mut commands, &selected.legal_moves, &squares_q);
                commands.entity(square).insert(Selected);
                commands.entity(*piece).insert(Selected);
                commands.insert_resource(selected);
            }
            PieceSelectionEvent::Deselected(piece) => {
                assert!(selected_piece.is_some());
                println!("Handling piece deselection for {:?}", piece);

                // remove `PossibleTarget` component to reset squares to default color
                possible_target_squares_q.for_each(|entity| {
                    commands.entity(entity).remove::<PossibleTarget>();
                });

                // deselect current square and piece
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

                // deselect entities
                possible_target_squares_q.for_each(|entity| {
                    commands.entity(entity).remove::<PossibleTarget>();
                });
                utils::deselect_piece(&mut commands, selected.piece);

                let square = ok_or_return!(piece_q.get(*piece)).0;
                let selected =
                    SelectedPiece::new(square, *piece, world).expect("unable to select piece");

                // mark squares and piece
                mark_possible_moves(&mut commands, &selected.legal_moves, &squares_q);
                commands.entity(square).insert(Selected);
                commands.entity(*piece).insert(Selected);
                commands.insert_resource(selected);
            }
        }
    }
}
