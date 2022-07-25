use crate::board::components::{Piece, PossibleTarget, Selected, Square};
use crate::board::events::PieceSelectionEvent;
use crate::board::{utils, SelectedPiece};
use crate::constants::{
    PIECE_Z_AXIS, POSSIBLE_TARGET_FILL_COLOR, POSSIBLE_TARGET_OUTLINE_COLOR,
    POSSIBLE_TARGET_OUTLINE_WIDTH, POSSIBLE_TARGET_RADIUS, SQUARE_Z_AXIS,
};
use crate::{ok_or_return, Location};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/// Visually marks the given locations on the board
fn mark_possible_targets(
    commands: &mut Commands,
    targets: &[Location],
    query: &Query<(Entity, &Location), With<Square>>,
) {
    let shape = RegularPolygon {
        sides: 36,
        feature: POSSIBLE_TARGET_RADIUS,
        ..default()
    };
    let draw_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(POSSIBLE_TARGET_FILL_COLOR),
        outline_mode: StrokeMode::new(POSSIBLE_TARGET_OUTLINE_COLOR, POSSIBLE_TARGET_OUTLINE_WIDTH),
    };
    let transform = Transform::from_xyz(0.0, 0.0, SQUARE_Z_AXIS + PIECE_Z_AXIS);

    for (entity, location) in query.iter() {
        if targets.contains(location) {
            commands.entity(entity).with_children(|square| {
                square
                    .spawn_bundle(GeometryBuilder::build_as(&shape, draw_mode, transform))
                    .insert(PossibleTarget);
            });
        }
    }
}

/// Handles `PieceSelectionEvent` events
pub fn handle_piece_selection_events(
    world: &World,
    mut commands: Commands,
    selected_squares_q: Query<Entity, (With<Square>, With<Selected>)>,
    possible_targets_q: Query<Entity, With<PossibleTarget>>,
    squares_q: Query<(Entity, &Location), With<Square>>,
    piece_locations_q: Query<(&Piece, &Location)>,
    piece_q: Query<&Parent, With<Piece>>,
    selected_piece: Option<Res<SelectedPiece>>,
    mut selection_event_reader: EventReader<PieceSelectionEvent>,
) {
    for event in selection_event_reader.iter() {
        match event {
            PieceSelectionEvent::Selected(piece) => {
                assert!(selected_piece.is_none());
                println!("Handling piece selection for {:?}", piece);

                // remove `Selected` component from all squares to clear previous move
                selected_squares_q.for_each(|square| {
                    commands.entity(square).remove::<Selected>();
                });

                let square = ok_or_return!(piece_q.get(*piece)).0;
                let selected = SelectedPiece::new(square, *piece, world, &piece_locations_q)
                    .expect("unable to select piece");

                // mark squares and piece
                mark_possible_targets(&mut commands, &selected.possible_targets, &squares_q);
                commands.entity(square).insert(Selected);
                commands.entity(*piece).insert(Selected);
                commands.insert_resource(selected);
            }
            PieceSelectionEvent::Deselected(piece) => {
                assert!(selected_piece.is_some());
                println!("Handling piece deselection for {:?}", piece);

                // remove `PossibleTarget` component to reset squares to default color
                possible_targets_q.for_each(|entity| {
                    commands.entity(entity).despawn_recursive();
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
                possible_targets_q.for_each(|entity| {
                    commands.entity(entity).despawn_recursive();
                });
                utils::deselect_piece(&mut commands, selected.piece);

                let square = ok_or_return!(piece_q.get(*piece)).0;
                let selected = SelectedPiece::new(square, *piece, world, &piece_locations_q)
                    .expect("unable to select piece");

                // mark squares and piece
                mark_possible_targets(&mut commands, &selected.possible_targets, &squares_q);
                commands.entity(square).insert(Selected);
                commands.entity(*piece).insert(Selected);
                commands.insert_resource(selected);
            }
        }
    }
}
