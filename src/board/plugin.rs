use crate::board::events::MovePieceEvent;
use crate::board::systems::{
    draw_selected_piece, handle_move_piece_events, left_click_piece_release,
    left_click_piece_selection, setup_board,
};
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board)
            .add_event::<MovePieceEvent>()
            .add_system(left_click_piece_selection)
            .add_system(left_click_piece_release)
            .add_system(draw_selected_piece)
            .add_system(handle_move_piece_events);
    }
}
