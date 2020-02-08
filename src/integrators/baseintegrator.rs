use crate::common::*;
use crate::film::Film;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;
use crate::{SceneCamera, SceneConfig, SceneGeometries, Tile};
use crossbeam::crossbeam_channel::unbounded;
use std::cell::{RefCell, Ref};
use crate::utilities::threadpool::ThreadPool;
use std::sync::Arc;
use std::borrow::Borrow;

pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBSDF,
    PathTracerNEE,
}

impl Integrator for BaseIntegrator {
    fn render(
        scene: Arc<SceneConfig>,
        samples_count: u32,
        bounces_count: u32,
        camera: Arc<SceneCamera>,
        geometries: Arc<SceneGeometries>,
        film: Arc<RefCell<Film>>,
    ) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = vec![];
        let cpus = num_cpus::get();
        info!("Trying with {} cpus", cpus);
        let pool = ThreadPool::new(cpus);

        info!(
            "Beginning rendering with {} spp and {} bounces",
            samples_count, bounces_count
        );
        let (s, r) = unbounded();
        let borrowed_film : Ref<Film>  = film.as_ref().borrow();
        let pixel_numbers = 0..(borrowed_film.height * borrowed_film.width);
        for i in pixel_numbers.step_by(TILE_SIZE) {
            let sender = s.clone();
            let camera = camera.clone();
            let geometries = geometries.clone();
            let scene = scene.clone();
            let film = film.clone();
            pool.execute(move || match scene.integrator {
                Integrators::DirectLighting => {
                    let tile: Tile = DirectLightingIntegrator::integrate(
                        i,
                        samples_count,
                        bounces_count,
                        camera,
                        geometries,
                        film,
                    );
                    sender.send(tile).unwrap();
                }
                Integrators::PathTracerBSDF => {}
                Integrators::PathTracerNEE => {}
            });
            //warn!("{:?}", tile.pixels);
        }
        drop(s); //To avoid waiting for the initial s which does not do anything
        tiles.extend(r);
        info!("Finished running render()");
        tiles
    }
}
