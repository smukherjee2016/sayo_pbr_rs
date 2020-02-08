use crate::common::*;
use crate::film::Film;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;
use crate::{SceneCamera, SceneConfig, SceneGeometries, Tile};
use crossbeam::crossbeam_channel::unbounded;
use scoped_pool::Pool;

pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBSDF,
    PathTracerNEE,
}

impl Integrator for BaseIntegrator {
    fn render(
        scene: &SceneConfig,
        samples_count: u32,
        bounces_count: u32,
        camera: &SceneCamera,
        geometries: &SceneGeometries,
        film: &Film,
    ) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = vec![];
        let cpus = num_cpus::get();
        info!("Trying with {} cpus", cpus);
        let pool = Pool::new(cpus);

        info!(
            "Beginning rendering with {} spp and {} bounces",
            samples_count, bounces_count
        );
        let (s, r) = unbounded();

        let pixel_numbers = 0..(film.height * film.width);
        pool.scoped(|scope| {
            for i in pixel_numbers.step_by(TILE_SIZE) {
                let sender = s.clone();
                scope.execute(move || match scene.integrator {
                    Integrators::DirectLighting => {
                        let tile: Tile = DirectLightingIntegrator::integrate(
                            i,
                            samples_count,
                            bounces_count,
                            &camera,
                            &geometries,
                            &film,
                        );
                        sender.send(tile).unwrap();
                    }
                    Integrators::PathTracerBSDF => {}
                    Integrators::PathTracerNEE => {}
                });
                //warn!("{:?}", tile.pixels);
            }
        });
        drop(s); //To avoid waiting for the initial s which does not do anything
        tiles.extend(r);
        info!("Finished running render()");
        tiles
    }
}
