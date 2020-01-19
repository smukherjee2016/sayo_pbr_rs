use crate::common::*;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;
use crate::{SceneConfig, Tile};
use crossbeam::crossbeam_channel::unbounded;
use minifb::{Key, Window, WindowOptions};
use scoped_pool::Pool;
use std::borrow::Borrow;

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

        //Set up temp buffer and window
        let mut frame_buffer: Vec<u32> = vec![0; (film.height * film.width) as usize];
        let mut window = Window::new(
            "Renderer?",
            film.width as usize,
            film.height as usize,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let pixel_numbers = 0..(film.height * film.width);
        pool.scoped(|scope| {
            for i in pixel_numbers.step_by(TILE_SIZE) {
                let sender = s.clone();
                scope.execute(move || match scene.integrator {
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
                });
                //warn!("{:?}", tile.pixels);
            }
        });
        drop(s); //To avoid waiting for the initial s which does not do anything

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Receive tile from renderer without blocking, should allow other keystrokes to be recognized
            let finished_tile_result = r.clone().try_recv();
            match finished_tile_result {
                Ok(finished_tile) => {
                    // Now that we have the tile from the renderer, push it into tiles for image
                    // and push it to display buffer, tone mapping and showing to the screen
                    tiles.push(finished_tile.clone());
                    //dbg!(finished_tile.num_pixels);
                }
                Err(e) => {}
            }
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(&frame_buffer, film.width as usize, film.height as usize)
                .unwrap();
        }
        info!("Finished running render()");
        tiles
    }
}
