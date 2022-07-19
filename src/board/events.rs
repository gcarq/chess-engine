use bevy::prelude::*;

pub struct UncheckedPieceMoveEvent {
    pub piece: Entity,
    pub source: Entity,
    pub target: Entity,
}

impl UncheckedPieceMoveEvent {
    pub fn new(piece: Entity, source: Entity, target: Entity) -> Self {
        Self {
            piece,
            source,
            target,
        }
    }
}

pub enum MoveTarget {
    Legal(Entity),
    Illegal,
    OutOfBound,
    None,
}

pub struct CheckedPieceMoveEvent {
    pub piece: Entity,
    pub source: Entity,
    pub target: MoveTarget,
}

impl CheckedPieceMoveEvent {
    pub fn new(piece: Entity, source: Entity, target: MoveTarget) -> Self {
        Self {
            piece,
            source,
            target,
        }
    }

    pub fn legal(event: &UncheckedPieceMoveEvent) -> Self {
        Self::new(event.piece, event.source, MoveTarget::Legal(event.target))
    }

    pub fn illegal(event: &UncheckedPieceMoveEvent) -> Self {
        Self::new(event.piece, event.source, MoveTarget::Illegal)
    }
}
