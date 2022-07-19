use crate::board::events::{CheckedPieceMoveEvent, UncheckedPieceMoveEvent};
use crate::board::systems::{
    check_move_legality, draw_selected_piece, handle_checked_move_events, left_click_piece_release,
    left_click_piece_selection, setup_board,
};
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_board)
            .add_event::<UncheckedPieceMoveEvent>()
            .add_event::<CheckedPieceMoveEvent>()
            .add_system(left_click_piece_selection)
            .add_system(left_click_piece_release)
            .add_system(draw_selected_piece)
            .add_system(check_move_legality)
            .add_system(handle_checked_move_events);
    }
}
