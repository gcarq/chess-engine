pub mod plugin;
mod systems;
mod utils;

#[derive(Default, Debug)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
