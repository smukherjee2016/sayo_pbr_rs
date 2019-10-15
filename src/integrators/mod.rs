use crate::{SceneConfig, Tile};

pub mod baseintegrator;
pub mod directlighting;

pub trait Integrator {
    fn render(scene: &SceneConfig, samples_count: u32, bounces_count: u32) -> Vec<Tile>;
}
