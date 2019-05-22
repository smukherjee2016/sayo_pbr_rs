use crate::common::*;
use crate::integrators::*;
use crate::SceneConfig;
use crate::camera::Camera;
use crate::film::Film;
use std::sync::Arc;

pub struct TestIntegrator;

impl Integrator for TestIntegrator {
    fn render(scene: &mut SceneConfig, samples_count: u32, bounces_count: u32) {
        let mut camera = &mut scene.camera;
        let mut film = &scene.film;

        for i in 0..(film.height * film.width) {
            let position_in_film = i;
            let x = position_in_film % film.width;
            let y = position_in_film / film.width;

            let pixel_value: Spectrum;
            for _j in 0..samples_count {
                for _k in 0..bounces_count {

                    //Core Integrator code goes here
                    let ray = camera.generate_camera_ray(x, y, Arc::from(film));
                    //info!("Ray info: {:?}", &ray);
                }
            }
        }
        info!("Finished running render()");
    }
}