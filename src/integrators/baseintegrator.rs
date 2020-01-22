use crate::common::*;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;
use crate::{SceneConfig, Tile};
use crossbeam::crossbeam_channel::unbounded;
use minifb::{Key, Window, WindowOptions};
use scoped_pool::Pool;
use std::borrow::{Borrow, BorrowMut};
use threadpool::ThreadPool;
use std::sync::Arc;

pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBSDF,
    PathTracerNEE,
}

//TODO: Move Window class and this method to its separate file
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn do_tonemapping(input: Vec<Spectrum>) -> Vec<u32> {
    let ret : Vec<u32> = vec![from_u8_rgb(0, 127, 255); input.len()];
    ret
}

impl Integrator for BaseIntegrator {
    fn render(scene: Arc<SceneConfig>, samples_count: u32, bounces_count: u32) {
        let mut tiles: Vec<Tile> = vec![];
        let cpus = num_cpus::get() - 1;
        info!("Trying with {} cpus", cpus);
        let pool = ThreadPool::new(cpus);
        let mut film = scene.film.clone();

        info!(
            "Beginning rendering with {} spp and {} bounces",
            samples_count, bounces_count
        );
        let (s, r) = unbounded();
        let r2 = r.clone();

        //Set up temp buffer and window
        let mut frame_buffer: Vec<u32> = vec![0; (film.height * film.width) as usize];
        dbg!(frame_buffer.len());
//        let mut window = Window::new(
//            "Renderer?",
//            film.width as usize,
//            film.height as usize,
//            WindowOptions::default(),
//        )
//        .unwrap_or_else(|e| {
//            panic!("{}", e);
//        });

        let pixel_numbers = 0..(film.height * film.width);
        //pool.scoped(|scope| {
            for i in pixel_numbers.step_by(TILE_SIZE) {
                let sender = s.clone();
                let cloned_scene = Arc::clone(&scene);
                pool.execute(move || match cloned_scene.integrator {
                    Integrators::DirectLighting => {
                        let tile: Tile = DirectLightingIntegrator::integrate(
                            cloned_scene,
                            i,
                            samples_count,
                            bounces_count,
                        );
                        dbg!("Did some tile work: {}", tile.start_index);
                        sender.send(tile).unwrap();
                    }
                    Integrators::PathTracerBSDF => {}
                    Integrators::PathTracerNEE => {}
                });
                //warn!("{:?}", tile.pixels);
            }
       // });
        drop(s); //To avoid waiting for the initial s which does not do anything

//        while window.is_open() && !window.is_key_down(Key::Escape) && !r.is_empty() {
//            //tiles.extend(r2.clone());
//            // Receive tile from renderer without blocking, should allow other keystrokes to be recognized
//            let finished_tile_result = r.try_recv();
//            match finished_tile_result {
//                Ok(finished_tile) => {
//                    // Now that we have the tile from the renderer, push it into tiles for image
//                    // and push it to display buffer, tone mapping and showing to the screen
//                    //tiles.push(finished_tile.clone());
//                    frame_buffer.splice(
//                        finished_tile.start_index as usize
//                            ..(finished_tile.start_index as usize + finished_tile.num_pixels),
//                        do_tonemapping(finished_tile.pixels),
//                    );
//
//                    //dbg!(finished_tile.num_pixels);
//                    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
//                    window
//                        .update_with_buffer(&frame_buffer, film.width as usize, film.height as usize)
//                        .unwrap();
//                }
//                Err(_) => {}
//            }
//
//        }
        tiles.extend(r);
        info!("Finished running render()");
        let film_mut = film.borrow_mut();
        for tile in tiles.clone() {
            //warn!("{}", tile.start_index);
            film_mut.write_tile(tile);
        }

    }
}
