use crate::SceneConfig;

pub mod baseintegrator;

pub trait Integrator {
    fn render(scene: &mut SceneConfig, samples_count: u32, bounces_count: u32);
}
