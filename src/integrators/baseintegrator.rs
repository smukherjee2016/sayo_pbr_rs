use crate::common::*;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;
use crate::{SceneConfig, Tile};
use crossbeam::crossbeam_channel::unbounded;
use std::borrow::Borrow;
use scoped_pool::Pool;

pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBSDF,
    PathTracerNEE,
}

impl Integrator for BaseIntegrator {
    fn render(scene: &SceneConfig, samples_count: u32, bounces_count: u32) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = vec![];
        let cpus = num_cpus::get();
        info!("Trying with {} cpus", cpus);
        let pool = Pool::new(cpus);

        let film = scene.film.borrow();
        info!(
            "Beginning rendering with {} spp and {} bounces",
            samples_count, bounces_count
        );
        let (s, r) = unbounded();

        let pixel_numbers = 0..(film.height * film.width);
        pool.scoped(|scope| {
            for i in pixel_numbers.step_by(TILE_SIZE) {
                let sender = s.clone();
                scope.execute(move ||
                 match scene.integrator {
                    Integrators::DirectLighting => {
                        let tile: Tile = DirectLightingIntegrator::integrate(
                            scene,
                            i,
                            samples_count,
                            bounces_count,
                        );
                        sender.send(tile).unwrap();
                    }
                    Integrators::PathTracerBSDF => {}
                    Integrators::PathTracerNEE => {}
                }
                );

                tiles.push(r.recv().unwrap());
                //warn!("{:?}", tile.pixels);
            }
        });
        info!("Finished running render()");
        tiles
    }
}
