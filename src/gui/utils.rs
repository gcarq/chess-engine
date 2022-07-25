use crate::board::PlayedMove;
use bevy_egui::egui::{Grid, RichText, Ui};

/// Takes a slice of all played moves and groups them by move number
fn group_played_moves(played_moves: &[PlayedMove]) -> Vec<Vec<PlayedMove>> {
    played_moves.chunks(2).map(|chunk| chunk.to_vec()).collect()
}

pub fn build_played_moves_grid(ui: &mut Ui, played_moves: &[PlayedMove]) {
    Grid::new("played_moves").show(ui, |ui| {
        let grouped_moves = group_played_moves(played_moves);
        for (move_nr, moves) in grouped_moves.iter().enumerate() {
            ui.label(
                RichText::new(format!("{}.", move_nr + 1))
                    .strong()
                    .size(18.0),
            );
            for played_move in moves {
                ui.label(RichText::new(played_move.notation()).size(16.0));
            }
            ui.end_row();
        }
    });
}
