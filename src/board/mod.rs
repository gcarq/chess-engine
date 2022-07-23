use crate::board::components::{PieceColor, PieceType};
use crate::{Location, Piece};
use bevy::prelude::*;
use itertools::Itertools;

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
    pub possible_targets: Vec<Location>,
}

impl SelectedPiece {
    pub fn new(
        square: Entity,
        piece: Entity,
        world: &World,
        pieces: &Query<(&Piece, &Location)>,
    ) -> Option<Self> {
        let piece_comp = *world.get_entity(piece)?.get::<Piece>()?;
        let location_comp = *world.get_entity(square)?.get::<Location>()?;

        let same_color_pieces = utils::same_color_pieces(&piece_comp.color, pieces);
        let possible_targets =
            SelectedPiece::legal_moves(&piece_comp, &location_comp, &same_color_pieces);
        Some(Self {
            square,
            piece,
            piece_comp,
            location_comp,
            possible_targets,
        })
    }

    /// Returns all possible squares the given `Piece` can move from the given `Location`.
    /// It is not checked whether the returned locations make up a legal move.
    fn legal_moves(
        piece: &Piece,
        location: &Location,
        same_color_pieces: &[Location],
    ) -> Vec<Location> {
        match piece.kind {
            PieceType::King => SelectedPiece::king_moves(location),
            PieceType::Queen => SelectedPiece::rook_moves(location, same_color_pieces)
                .into_iter()
                .chain(SelectedPiece::bishop_moves(location, same_color_pieces).into_iter())
                .collect(),
            PieceType::Rook => SelectedPiece::rook_moves(location, same_color_pieces),
            PieceType::Bishop => SelectedPiece::bishop_moves(location, same_color_pieces),
            PieceType::Knight => SelectedPiece::knight_moves(location),
            PieceType::Pawn => SelectedPiece::pawn_moves(location, piece),
        }
        .into_iter()
        .filter(|loc| !same_color_pieces.contains(loc))
        .collect()
    }

    /// Returns all possible king moves
    fn king_moves(location: &Location) -> Vec<Location> {
        let mut offsets = vec![-1, 0, 1].into_iter().permutations(2).collect_vec();
        offsets.extend(vec![vec![-1, -1], vec![1, 1]]);
        utils::translate_from_offsets(location, offsets)
    }

    /// Returns all possible knight moves
    fn knight_moves(location: &Location) -> Vec<Location> {
        let offsets = vec![
            vec![-2, -1],
            vec![-2, 1],
            vec![-1, -2],
            vec![-1, 2],
            vec![1, -2],
            vec![1, 2],
            vec![2, -1],
            vec![2, 1],
        ];
        utils::translate_from_offsets(location, offsets)
    }

    /// Returns all possible pawn moves
    /// TODO: handle piece promotion, captures and en passant
    fn pawn_moves(location: &Location, piece: &Piece) -> Vec<Location> {
        let mut squares = Vec::new();
        match piece.color {
            PieceColor::White => {
                if location.y < 7 {
                    squares.push(Location::new(location.x, location.y + 1));
                }
                if !piece.has_moved {
                    squares.push(Location::new(location.x, location.y + 2));
                }
            }
            PieceColor::Black => {
                if location.y > 0 {
                    squares.push(Location::new(location.x, location.y - 1));
                }
                if !piece.has_moved {
                    squares.push(Location::new(location.x, location.y - 2));
                }
            }
        }
        squares
    }

    /// Returns all possible Rook moves
    fn rook_moves(location: &Location, same_color_pieces: &[Location]) -> Vec<Location> {
        let x_ranges: Vec<Vec<usize>> = vec![
            (0..location.x).rev().collect(),
            (location.x + 1..8).collect(),
        ];
        let y_ranges: Vec<Vec<usize>> = vec![
            (0..location.y).rev().collect(),
            (location.y + 1..8).collect(),
        ];

        let x_locations = x_ranges.into_iter().flat_map(|x_range| {
            x_range
                .into_iter()
                .map(|x| Location::new(x, location.y))
                .take_while(|loc| !same_color_pieces.contains(loc))
        });

        let y_locations = y_ranges.into_iter().flat_map(|y_range| {
            y_range
                .into_iter()
                .map(|y| Location::new(location.x, y))
                .take_while(|loc| !same_color_pieces.contains(loc))
        });
        x_locations.chain(y_locations).collect()
    }

    /// Returns all possible bishop moves
    fn bishop_moves(location: &Location, same_color_pieces: &[Location]) -> Vec<Location> {
        let offset_modifiers = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
        offset_modifiers
            .into_iter()
            .flat_map(|(x_mod, y_mod)| {
                (1..7)
                    .map_while(move |offset| location.translate(offset * x_mod, offset * y_mod))
                    .take_while(|loc| !same_color_pieces.contains(loc))
            })
            .collect()
    }
}
