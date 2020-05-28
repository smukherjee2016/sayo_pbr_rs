use crate::accel::aabb::Boundable;
use crate::common::*;
use crate::film::Film;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;
use crate::utilities::threadpool::ThreadPool;
use crate::{SceneCamera, SceneConfig, SceneGeometries, Tile};
use crossbeam::crossbeam_channel::unbounded;
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;
use std::{thread, time};

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
    let mut ret: Vec<u32> = Vec::with_capacity(input.len());
    for element in &input {
        let mut x: Spectrum = *element * 16.0; // Hardcoded Exposure Adjustment
        x = Spectrum::from(0.0).max_component_wise(x - Spectrum::from(0.004));
        let ret_color: Spectrum = (x * (Spectrum::from(6.2) * x + Spectrum::from(0.5)))
            / (x * (Spectrum::from(6.2) * x + Spectrum::from(1.7)) + Spectrum::from(0.06));
        let gamma_corrected_color: Spectrum = Spectrum::new(
            ret_color.x.powf(2.20),
            ret_color.y.powf(2.20),
            ret_color.z.powf(2.20),
        );
        let final_color: u32 = from_u8_rgb(
            (gamma_corrected_color.x * 255.0) as u8,
            (gamma_corrected_color.y * 255.0) as u8,
            (gamma_corrected_color.z * 255.0) as u8,
        );
        ret.push(final_color);
    }
    ret
}

impl Integrator for BaseIntegrator {
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

        //Set up temp buffer and window
        // let mut frame_buffer: Vec<u32> = vec![0; (film.height * film.width) as usize];
        // info!("{}", frame_buffer.len());
        // let mut window = Window::new(
        //     "Renderer?",
        //     film.width as usize,
        //     film.height as usize,
        //     WindowOptions::default(),
        // )
        // .unwrap_or_else(|e| {
        //     panic!("{}", e);
        // });

        let pixel_numbers = 0..(film.height * film.width);
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
                        t_min,
                        t_max,
                    );
                    sender.send(tile).unwrap();
                }
                Integrators::PathTracerBSDF => {}
                Integrators::PathTracerNEE => {}
            });
            //warn!("{:?}", tile.pixels);
        }
        drop(s);
        // #[allow(clippy::never_loop)]
        // while window.is_open() && !window.is_key_down(Key::Escape) {
        //     let r2 = r;
        //     thread::sleep(time::Duration::from_millis(16));
        //     for finished_tile in r2 {
        //         let tile = finished_tile.clone();
        //         // Now that we have the tile from the renderer, push it into tiles for image
        //         // and push it to display buffer, tone mapping and showing to the screen
        //         //tiles.push(finished_tile.clone());
        //         frame_buffer.splice(
        //             finished_tile.start_index as usize
        //                 ..(finished_tile.start_index as usize + finished_tile.num_pixels),
        //             do_tonemapping(finished_tile.pixels),
        //         );
        //
        //         //info!(finished_tile.num_pixels);
        //         // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        //         window
        //             .update_with_buffer(&frame_buffer, film.width as usize, film.height as usize)
        //             .unwrap();
        //         tiles.push(tile);
        //     }
        //     break;
        // }
        tiles.extend(r);
        info!("Finished running render()");
        tiles
    }
}
