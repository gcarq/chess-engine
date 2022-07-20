use crate::board::events::{CheckedPieceMoveEvent, PieceSelectionEvent, UncheckedPieceMoveEvent};
use crate::board::systems::{
    handle_checked_move_events, handle_piece_selection, handle_square_updates,
    verify_unchecked_moves,
};
use crate::board::systems::{input, startup};
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup::setup_board)
            .add_event::<PieceSelectionEvent>()
            .add_event::<UncheckedPieceMoveEvent>()
            .add_event::<CheckedPieceMoveEvent>()
            .add_system(input::left_click_piece_selection)
            //.add_system(draw_selected_piece)
            .add_system(handle_piece_selection)
            .add_system(handle_square_updates)
            .add_system(verify_unchecked_moves)
            .add_system(handle_checked_move_events);
    }
}
