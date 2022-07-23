use crate::gui::systems::{render_ui, update_camera_transform_system};
use bevy::prelude::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_ui)
            .add_system(update_camera_transform_system);
    }
}
