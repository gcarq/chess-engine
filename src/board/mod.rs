use crate::board::components::{PieceColor, PieceType};
use crate::{Location, Piece};
use bevy::prelude::*;

pub mod components;
mod events;
pub mod plugin;
mod systems;
mod utils;

/// Holds the currently selected piece square where it sits on and all legal moves it can take
#[derive(Clone)]
pub struct SelectedPiece {
    pub square: Entity,
    pub piece: Entity,
    pub piece_comp: Piece,
    pub location_comp: Location,
    pub legal_moves: Vec<Location>,
}

impl SelectedPiece {
    pub fn new(square: Entity, piece: Entity, world: &World) -> Option<Self> {
        let piece_comp = *world.get_entity(piece)?.get::<Piece>()?;
        let location_comp = *world.get_entity(square)?.get::<Location>()?;
        let legal_moves = SelectedPiece::legal_moves(&piece_comp, &location_comp);
        Some(Self {
            square,
            piece,
            piece_comp,
            location_comp,
            legal_moves,
        })
    }

    /// Returns all possible squares the given `Piece` can move from the given `Location`.
    /// TODO: Currently it is not checked whether the returned locations make up a legal move.
    fn legal_moves(piece: &Piece, location: &Location) -> Vec<Location> {
        let mut squares = Vec::new();
        match piece.kind {
            PieceType::King => {
                // TODO:
            }
            PieceType::Queen => {
                // TODO:
            }
            PieceType::Rook => {
                for x in 0..8 {
                    if x != location.x {
                        squares.push(Location::new(x, location.y));
                    }
                }
                for y in 0..8 {
                    if y != location.y {
                        squares.push(Location::new(location.x, y));
                    }
                }
            }
            PieceType::Bishop => {
                // TODO:
            }
            PieceType::Knight => {
                // TODO:
            }
            PieceType::Pawn => {
                match piece.color {
                    PieceColor::White => {
                        // TODO: handle promotion
                        if location.y < 7 {
                            squares.push(Location::new(location.x, location.y + 1));
                        }
                        if !piece.has_moved {
                            squares.push(Location::new(location.x, location.y + 2));
                        }
                    }
                    PieceColor::Black => {
                        // TODO: handle promotion
                        if location.y > 0 {
                            squares.push(Location::new(location.x, location.y - 1));
                        }
                        if !piece.has_moved {
                            squares.push(Location::new(location.x, location.y - 2));
                        }
                    }
                }
            }
        }
        squares
    }
}
