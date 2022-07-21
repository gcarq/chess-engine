use crate::board::SelectedPiece;
use bevy::prelude::*;

pub struct UncheckedPieceMoveEvent {
    pub selected: SelectedPiece,
    pub target: Entity,
}

impl UncheckedPieceMoveEvent {
    pub fn new(selected: SelectedPiece, target: Entity) -> Self {
        Self { selected, target }
    }
}

pub enum MoveTarget {
    Legal(Entity),
    Illegal,
}

pub struct CheckedPieceMoveEvent {
    pub selected: SelectedPiece,
    pub target: MoveTarget,
}

impl CheckedPieceMoveEvent {
    pub fn new(selected: SelectedPiece, target: MoveTarget) -> Self {
        Self { selected, target }
    }

    pub fn legal(event: &UncheckedPieceMoveEvent) -> Self {
        Self::new(event.selected.clone(), MoveTarget::Legal(event.target))
    }

    pub fn illegal(event: &UncheckedPieceMoveEvent) -> Self {
        Self::new(event.selected.clone(), MoveTarget::Illegal)
    }
}

pub enum PieceSelectionEvent {
    Selected(Entity),
    Deselected(Entity),
    Reselect(Entity),
}
