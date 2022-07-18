use crate::board::systems::{
    handle_piece_movement, piece_deselection, piece_selection, setup_board,
};
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board)
            .add_system(piece_selection)
            .add_system(piece_deselection)
            .add_system(handle_piece_movement);
    }
}
