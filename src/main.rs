use crate::board::components::{Location, Piece};
use crate::board::plugin::BoardPlugin;
use crate::constants::{SQUARE_SIZE, WINDOW_BACKGROUND_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::gui::plugin::GuiPlugin;
use crate::gui::OccupiedScreenSpace;
use crate::resources::ResourcePlugin;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_prototype_lyon::prelude::*;
use bevy_svg::prelude::*;

mod board;
mod constants;
mod gui;
pub mod macros;
mod resources;

/// Holds the original`Transform` for `BoardCamera` entity
/// to offset the camera after drawing UI elements
pub struct OriginalCameraTransforms {
    pub board_camera: Transform,
    pub text_camera: Transform,
}

#[derive(Component)]
pub struct BoardCamera;
#[derive(Component)]
pub struct TextCamera;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Location>()
                .register_inspectable::<Piece>();
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "chess-engine".to_string(),
            height: WINDOW_HEIGHT,
            width: WINDOW_WIDTH,
            resizable: true,
            ..default()
        })
        .init_resource::<OccupiedScreenSpace>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(SvgPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(BoardPlugin)
        .add_startup_system(setup_basics)
        .run();
}

fn setup_basics(mut commands: Commands) {
    let board_camera = OrthographicCameraBundle::new_2d();
    let text_camera = UiCameraBundle::default();
    // Optimal power saving and present mode settings for desktop apps.
    commands.insert_resource(WinitSettings::game());
    commands.insert_resource(ClearColor(WINDOW_BACKGROUND_COLOR));
    commands.insert_resource(OriginalCameraTransforms {
        board_camera: board_camera.transform,
        text_camera: text_camera.transform,
    });
    commands.spawn_bundle(board_camera).insert(BoardCamera);
    commands.spawn_bundle(text_camera).insert(TextCamera);
}
