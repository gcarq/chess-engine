use bevy::prelude::*;

pub mod components;
mod events;
pub mod plugin;
mod systems;
mod utils;

/// Holds the currently selected piece to move it around
pub struct SelectedPiece(pub Entity);
