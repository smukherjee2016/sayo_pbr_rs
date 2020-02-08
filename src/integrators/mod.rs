use crate::film::Film;
use crate::{SceneCamera, SceneConfig, SceneGeometries, Tile};

pub mod baseintegrator;
pub mod directlighting;

pub trait Integrator {
    fn render(
        scene: &SceneConfig,
        samples_count: u32,
        bounces_count: u32,
        camera: &SceneCamera,
        geometries: &SceneGeometries,
        film: &Film,
    ) -> Vec<Tile>;
}
