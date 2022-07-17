use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use std::fmt;

#[derive(Inspectable)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Inspectable)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Component, Inspectable)]
pub struct Piece {
    pub kind: PieceType,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(kind: PieceType, color: PieceColor) -> Self {
        Self { kind, color }
    }

    pub fn notation(&self) -> char {
        match self.kind {
            PieceType::King => 'K',
            PieceType::Queen => 'Q',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Pawn => 'P',
        }
    }

    /// Returns the identifier for the `PieceTheme` resource
    pub fn resource_name(&self) -> String {
        let color = match self.color {
            PieceColor::White => 'w',
            PieceColor::Black => 'b',
        };
        format!("{}{}", color, self.notation())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.notation())
    }
}
