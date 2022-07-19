use bevy::prelude::*;

pub mod components;
mod events;
pub mod plugin;
mod systems;
mod utils;

/// Holds the currently selected piece and square where it sits on
pub struct SelectedPiece {
    pub square: Entity,
    pub piece: Entity,
}

impl SelectedPiece {
    pub fn new(square: Entity, piece: Entity) -> Self {
        Self { square, piece }
    }
}
