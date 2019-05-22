use crate::common::*;
use crate::SceneConfig;
use crate::camera::*;
use crate::film::Film;
use std::sync::Arc;

pub mod testintegrator;

pub trait Integrator {
    fn render(scene : &mut SceneConfig, samples_count: u32, bounces_count: u32);
}