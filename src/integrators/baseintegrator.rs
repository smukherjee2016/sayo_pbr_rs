use crate::common::*;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;
use crate::SceneConfig;
use std::borrow::{Borrow, BorrowMut};

pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBSDF,
    PathTracerNEE,
}

#[derive(Default)]
pub struct Tile {
    pub start_index: i32,
    pub num_pixels: usize,
    pub pixels: Vec<Spectrum>,
}

impl Integrator for BaseIntegrator {
    fn render(scene: &mut SceneConfig, samples_count: u32, bounces_count: u32) {
        let mut tiles: Vec<Tile> = vec![];
        let film = scene.film.borrow();
        info!(
            "Beginning rendering with {} spp and {} bounces",
            samples_count, bounces_count
        );
        let pixel_numbers = 0..(film.height * film.width);
        for i in pixel_numbers.step_by(TILE_SIZE) {
            let mut tile: Tile = Tile::default();
            match scene.integrator {
                Integrators::DirectLighting => {
                    tile =
                        DirectLightingIntegrator::integrate(scene, i, samples_count, bounces_count);
                }
                Integrators::PathTracerBSDF => {}
                Integrators::PathTracerNEE => {}
            }
            //warn!("{:?}", tile.pixels);
            tiles.push(tile);
        }
        let film_mut = scene.film.borrow_mut();
        for tile in tiles {
            //warn!("{}", tile.start_index);
            film_mut.write_tile(tile);
        }

        info!("Finished running render()");
    }
}
