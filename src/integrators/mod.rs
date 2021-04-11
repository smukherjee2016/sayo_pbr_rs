use crate::accel::aabb::Boundable;
use crate::common::*;
use crate::film::Film;
use crate::{SceneCamera, SceneConfig, Tile};
use std::sync::Arc;

pub mod baseintegrator;
pub mod directlighting;

pub trait Integrator {
    fn render(
        scene: Arc<SceneConfig>,
        samples_count: u32,
        bounces_count: u32,
        camera: Arc<SceneCamera>,
        geometries: Arc<dyn Boundable>,
        //geometries: Arc<SceneGeometries>,
        film: Arc<Film>,
        t_min: fp,
        t_max: fp,
    ) -> Vec<Tile>;
}
