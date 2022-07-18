use bevy::prelude::*;

pub struct MovePieceEvent {
    pub from: Entity,
    pub to: Entity,
    pub piece: Entity,
}

impl MovePieceEvent {
    pub fn new(from: Entity, to: Entity, piece: Entity) -> Self {
        Self { from, to, piece }
    }
}
