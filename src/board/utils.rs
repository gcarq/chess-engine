use crate::board::components::SquareColor;
use crate::{MainCamera, SQUARE_SIZE};
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

pub fn index_for_rank(rank: usize) -> usize {
    assert!(rank > 0 && rank < 9);
    rank - 1
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
