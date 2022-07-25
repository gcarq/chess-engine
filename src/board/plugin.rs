use crate::board::components::PieceColor;
use crate::board::events::{
    CheckedPieceMoveEvent, PieceSelectionEvent, PlayedMoveEvent, UncheckedPieceMoveEvent,
};
use crate::board::systems::{
    handle_checked_move_events, handle_square_status_updates, handle_unchecked_move_events,
    record_played_moves, selection,
};
use crate::board::systems::{input, startup};
use crate::board::{CurrentPlayer, PlayedMoves};
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayedMoves>()
            .insert_resource(CurrentPlayer(PieceColor::White))
            .add_startup_system(startup::setup_board)
            .add_event::<PieceSelectionEvent>()
            .add_event::<UncheckedPieceMoveEvent>()
            .add_event::<CheckedPieceMoveEvent>()
            .add_event::<PlayedMoveEvent>()
            .add_system(input::left_click_piece_selection)
            .add_system(selection::handle_piece_selection_events)
            .add_system(handle_square_status_updates)
            .add_system(handle_unchecked_move_events)
            .add_system(handle_checked_move_events)
            .add_system(record_played_moves);
    }
}
