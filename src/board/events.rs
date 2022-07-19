use bevy::prelude::*;

pub struct UncheckedPieceMoveEvent {
    pub from: Entity,
    pub to: Entity,
    pub piece: Entity,
}

impl UncheckedPieceMoveEvent {
    pub fn new(from: Entity, to: Entity, piece: Entity) -> Self {
        Self { from, to, piece }
    }
}

pub struct CheckedPieceMoveEvent {
    pub from: Entity,
    pub to: Entity,
    pub piece: Entity,
    pub is_legal: bool,
}

impl CheckedPieceMoveEvent {
    pub fn from(event: &UncheckedPieceMoveEvent, is_legal: bool) -> Self {
        Self {
            from: event.from,
            to: event.to,
            piece: event.piece,
            is_legal,
        }
    }
}
