use crate::film::Film;
use crate::{SceneCamera, SceneConfig, SceneGeometries, Tile};
use std::sync::Arc;
use crate::accel::aabb::Boundable;
use crate::accel::bvh_node::BVHNode;

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
    ) -> Vec<Tile>;
}
