use crate::constants::PIECE_THEME;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;
use bevy_svg::prelude::Svg;
use std::fmt;
use std::path::Path;

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

pub struct PieceTheme {
    pub vectors: HashMap<String, Handle<Svg>>,
}

impl FromWorld for PieceTheme {
    /// Loads the vector graphics for the current `PIECE_THEME`
    fn from_world(world: &mut World) -> Self {
        let mut vectors = HashMap::with_capacity(12);
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        // TODO: use load_folder()
        let path = Path::new("piece").join(PIECE_THEME);
        for color in ['b', 'w'] {
            for piece in ['B', 'K', 'N', 'P', 'Q', 'R'] {
                let asset_id = format!("{}{}", color, piece);
                let svg_handle = asset_server.load(path.join(format!("{}.svg", asset_id)));
                vectors.insert(asset_id, svg_handle);
            }
        }

        Self { vectors }
    }
}
