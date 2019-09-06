pub use crate::integrators::directlighting;
use crate::integrators::Integrator;
use crate::SceneConfig;
use crate::integrators::directlighting::DirectLightingIntegrator;


pub struct BaseIntegrator;

pub enum Integrators {
    DirectLighting,
    PathTracerBSDF,
    PathTracerNEE,
}

impl Integrator for BaseIntegrator {
    fn render(scene: &mut SceneConfig, samples_count: u32, bounces_count: u32) {
        match scene.integrator {
            Integrators::DirectLighting => {
                DirectLightingIntegrator::render(scene, samples_count, bounces_count);
            }
            Integrators::PathTracerBSDF => {}
            Integrators::PathTracerNEE => {}
        }
    }
}
