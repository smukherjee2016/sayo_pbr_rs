use crate::accel::aabb::Boundable;
use crate::common::*;
use crate::film::Film;
pub use crate::integrators::directlighting;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::integrators::Integrator;

use crate::{SceneCamera, SceneConfig};

use ndarray::parallel::prelude::*;
use ndarray::Array2;

use std::sync::Arc;

pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBsdf,
    PathTracerNee,
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
    ) -> Array2<Spectrum> {
        let scene_data: Vec<Spectrum> = vec![
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            (film.height * film.width) as usize
        ];
        let mut frame_buffer2 =
            Array2::from_shape_vec((film.height as usize, film.width as usize), scene_data)
                .unwrap();
        let tiles: Vec<_> = frame_buffer2
            .exact_chunks_mut([16, 16])
            .into_iter()
            .collect();

        tiles
            .into_par_iter()
            .enumerate()
            .for_each(|(i, tile)| match scene.integrator {
                Integrators::DirectLighting => {
                    let camera = camera.clone();
                    let geometries = geometries.clone();
                    let film = film.clone();
                    DirectLightingIntegrator::integrate(
                        tile,
                        i as i32,
                        samples_count,
                        bounces_count,
                        camera,
                        geometries,
                        film,
                        t_min,
                        t_max,
                    );
                }
                Integrators::PathTracerBsdf => {}
                Integrators::PathTracerNee => {}
            });
        //info!("frame_buffer2: {:?}", frame_buffer2);

        frame_buffer2
    }
}
