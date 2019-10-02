pub use crate::integrators::directlighting;
use crate::integrators::Integrator;
use crate::SceneConfig;
use crate::integrators::directlighting::DirectLightingIntegrator;
use crate::common::*;

pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBSDF,
    PathTracerNEE,
}

impl Integrator for BaseIntegrator {
    fn render(scene: &mut SceneConfig, samples_count: u32, bounces_count: u32) {
        let camera = &scene.camera;
        let mut film = &mut scene.film.borrow_mut();
        info!(
            "Beginning rendering with {} spp and {} bounces",
            samples_count, bounces_count
        );

        for i in 0..(film.height * film.width) {
            match scene.integrator {
                Integrators::DirectLighting => {

                    DirectLightingIntegrator::integrate(film.clone(), i, samples_count, bounces_count);
                }
                Integrators::PathTracerBSDF => {}
                Integrators::PathTracerNEE => {}
            }
        }
        info!("Finished running render()");
    }
}
