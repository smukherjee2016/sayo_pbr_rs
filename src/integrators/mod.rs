use crate::{SceneConfig, Tile};
use std::sync::Arc;

pub mod baseintegrator;
pub mod directlighting;

pub trait Integrator {
    fn render(scene: Arc<SceneConfig>, samples_count: u32, bounces_count: u32);
}
