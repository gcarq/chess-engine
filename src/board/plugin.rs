use crate::board::events::{CheckedPieceMoveEvent, PieceSelectionEvent, UncheckedPieceMoveEvent};
use crate::board::systems::{
    handle_checked_move_events, handle_square_status_updates, handle_unchecked_move_events,
    selection,
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
            .add_system(selection::handle_piece_selection_events)
            .add_system(handle_square_status_updates)
            .add_system(handle_unchecked_move_events)
            .add_system(handle_checked_move_events);
    }
}
