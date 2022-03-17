use ndarray::Array2;
use tev_client::TevClient;

use crate::accel::aabb::Boundable;
use crate::common::*;
use crate::film::Film;
use crate::{SceneCamera, SceneConfig};
use std::sync::{Arc, Mutex};

pub mod baseintegrator;
pub mod directlighting;

pub trait Integrator {
    #[allow(clippy::too_many_arguments)]
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
        tev_client: Arc<Mutex<TevClient>>,
    ) -> Array2<Spectrum>;
}
