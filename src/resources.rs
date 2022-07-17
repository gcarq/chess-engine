use crate::constants::PIECE_THEME;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_svg::prelude::*;
use std::path::Path;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceTheme>()
            .init_resource::<DefaultFont>();
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

pub struct DefaultFont(pub Handle<Font>);

impl FromWorld for DefaultFont {
    fn from_world(world: &mut World) -> Self {
        let font_handle = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load("FiraSans-Regular.ttf");
        Self(font_handle)
    }
}
