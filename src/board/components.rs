use std::fmt;

use crate::board::utils;
use crate::constants::{
    SQUARE_COLOR_DARK_DEFAULT, SQUARE_COLOR_DARK_SELECTED, SQUARE_COLOR_LIGHT_DEFAULT,
    SQUARE_COLOR_LIGHT_SELECTED,
};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        assert!(x < 9);
        assert!(y < 9);
        Self { x, y }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            File::from_index(self.x),
            utils::rank_for_index(self.y)
        )
    }
}

#[derive(Component, Debug)]
pub struct Square;

pub enum SquareColor {
    Light,
    Dark,
}

impl SquareColor {
    pub fn default(&self) -> Color {
        match &self {
            SquareColor::Light => SQUARE_COLOR_LIGHT_DEFAULT,
            SquareColor::Dark => SQUARE_COLOR_DARK_DEFAULT,
        }
    }

    pub fn selected(&self) -> Color {
        match &self {
            SquareColor::Light => SQUARE_COLOR_LIGHT_SELECTED,
            SquareColor::Dark => SQUARE_COLOR_DARK_SELECTED,
        }
    }
}

/// A column of the chessboard.
/// A specific file can be named either using its position in algebraic notation,
/// a–h, or by using its position in descriptive notation.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn from_index(index: usize) -> Self {
        assert!(index < 8);
        match index {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        };
        write!(f, "{}", char)
    }
}

#[derive(Component)]
pub struct Board;

#[derive(Inspectable, Debug, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Inspectable, Debug, Eq, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Component, Inspectable, Debug, Copy, Clone)]
pub struct Piece {
    pub kind: PieceType,
    pub color: PieceColor,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(kind: PieceType, color: PieceColor) -> Self {
        Self {
            kind,
            color,
            has_moved: false,
        }
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

/// Used to distinguish between selected and non-selected pieces and squares
#[derive(Component)]
pub struct Selected;

/// Used to mark squares as possible targets for the current piece move
#[derive(Component)]
pub struct PossibleTarget;
