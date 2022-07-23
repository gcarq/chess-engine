use crate::board::components::{PieceColor, Selected, SquareColor};
use crate::board::events::CheckedPieceMoveEvent;
use crate::board::SelectedPiece;
use crate::{Location, MainCamera, Piece, SQUARE_SIZE};
use bevy::prelude::*;

/// Translates the current cursor position to world coordinates
pub fn translate_cursor_pos(
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
) -> Option<Vec2> {
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = cameras.single();
    let window = windows.get_primary().unwrap();
    let screen_pos = window.cursor_position()?;

    // get the size of the window
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    Some(world_pos.truncate())
}

/// Checks if the given position intersects the square of the given piece
pub fn intersects_square(pos: &Vec2, square: &Vec3) -> bool {
    let center_offset = center_offset();
    let in_x = square.x - center_offset <= pos.x && pos.x <= square.x + center_offset;
    let in_y = square.y - center_offset <= pos.y && pos.y <= square.y + center_offset;
    in_x && in_y
}

/// Returns the correct Rank for the given vector index
pub fn rank_for_index(index: usize) -> usize {
    assert!(index < 8);
    index + 1
}

/// Returns the color for the given indicates
pub fn square_color(x: usize, y: usize) -> SquareColor {
    if (x + y + 1) % 2 == 0 {
        SquareColor::Light
    } else {
        SquareColor::Dark
    }
}

/// Returns the center offset for a `Square`
pub fn center_offset() -> f32 {
    SQUARE_SIZE / 2.0
}

/// Deselects the current piece
pub fn deselect_piece(commands: &mut Commands, piece: Entity) {
    commands.entity(piece).remove::<Selected>();
    commands.remove_resource::<SelectedPiece>();
}

/// Removes piece entity from source square and adds it to the target square as a child,
/// also replaces `Location` component on entity and sets `has_moved` to reflect this move properly
pub fn update_entity_for_move(
    commands: &mut Commands,
    event: &CheckedPieceMoveEvent,
    target: Entity,
    loc_comp: Location,
) {
    // Update square children
    commands
        .entity(event.selected.square)
        .remove_children(&[event.selected.piece]);
    commands.entity(target).add_child(event.selected.piece);

    // Update `Location` component for piece entity
    commands.entity(event.selected.piece).remove::<Location>();
    commands.entity(event.selected.piece).insert(loc_comp);

    // Update `Piece` component for piece entity
    let mut piece_comp = event.selected.piece_comp;
    piece_comp.has_moved = true;
    commands.entity(event.selected.piece).remove::<Piece>();
    commands.entity(event.selected.piece).insert(piece_comp);
}

/// Adjusts the given piece `GlobalTransform` to square `GlobalTransform`
pub fn adjust_to_square(piece: &mut GlobalTransform, square: &GlobalTransform) {
    let center_offset = center_offset();
    piece.translation.x = square.translation.x - center_offset;
    piece.translation.y = square.translation.y + center_offset;
}

/// Returns the first entity from the list that has the `Piece` component
pub fn resolve_piece(entities: &[Entity], pieces_q: &Query<&Piece>) -> Option<(Entity, Piece)> {
    for entity in entities {
        if let Ok(comp) = pieces_q.get(*entity) {
            return Some((*entity, *comp));
        }
    }
    None
}

/// Returns a new vector of `Location` created by the given location and offsets
pub fn translate_from_offsets(location: &Location, offsets: Vec<Vec<isize>>) -> Vec<Location> {
    offsets
        .into_iter()
        .filter_map(|off| location.translate(off[0], off[1]))
        .collect()
}

/// Returns the `Location` of all same color pieces
pub fn same_color_pieces(color: &PieceColor, pieces: &Query<(&Piece, &Location)>) -> Vec<Location> {
    pieces
        .iter()
        .filter_map(|(piece, location)| match color == &piece.color {
            true => Some(location),
            false => None,
        })
        .cloned()
        .collect()
}
