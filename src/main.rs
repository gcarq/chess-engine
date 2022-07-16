use crate::board::pieces::Piece;
use crate::board::{BoardPlugin, Location};
use crate::constants::{SQUARE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_svg::prelude::*;

mod board;
mod constants;

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
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(SvgPlugin)
        .add_startup_system(setup_basics)
        .run();
}

fn setup_basics(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
