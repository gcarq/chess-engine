use crate::board::PlayedMoves;
use crate::constants::SIDE_PANEL_RIGHT_WIDTH;
use crate::gui::{utils, OccupiedScreenSpace};
use crate::{BoardCamera, OriginalCameraTransforms};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn render_ui(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    played_moves: Res<PlayedMoves>,
) {
    occupied_screen_space.left = 0.0;
    occupied_screen_space.top = 0.0;
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .default_width(SIDE_PANEL_RIGHT_WIDTH)
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            utils::build_played_moves_grid(ui, &played_moves.0);
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

pub fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransforms>,
    mut board_camera_query: Query<&mut Transform, With<BoardCamera>>,
) {
    let horizontal_offset = (occupied_screen_space.right - occupied_screen_space.left) / 2.0;
    let mut board_cam_tf = board_camera_query.get_single_mut().unwrap();
    board_cam_tf.translation.x =
        original_camera_transform.board_camera.translation.x + horizontal_offset;
}
