use std::fmt;

use crate::board::utils;
use crate::constants::{SQUARE_COLOR_DARK, SQUARE_COLOR_LIGHT};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Copy, Clone, Debug)]
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

    pub fn from_notation(file: File, rank: usize) -> Self {
        Self::new(file.to_index(), utils::index_for_rank(rank))
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
    pub fn color(&self) -> Color {
        match &self {
            SquareColor::Light => SQUARE_COLOR_LIGHT,
            SquareColor::Dark => SQUARE_COLOR_DARK,
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

    pub fn to_index(&self) -> usize {
        match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
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

#[derive(Inspectable, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Inspectable, Debug)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Component, Inspectable, Debug)]
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

/// Holds the currently selected piece to move it around
pub struct SelectedPiece(pub Entity);