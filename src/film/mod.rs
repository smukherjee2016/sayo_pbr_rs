use crate::common::*;
use cgmath::Vector3;

#[derive(Debug, Default, Clone)]
pub struct Film {
    pub image : Vec<Color>,
    pub height: i32,
    pub width : i32,
    pub fov : fp,
    pub distance_to_film: fp,
    pub aspect_ratio: fp
}

impl Film {
    pub fn new(&mut self, width : i32, height : i32, fovdegrees : fp) {

        self.distance_to_film = 1.0;
        self.aspect_ratio = width as fp / height as fp;
        self.fov = fovdegrees * std::f64::consts::PI / 360.0;

        self.width = width;
        self.height = height;
        self.image.resize((width * height) as usize, Vector3::from((0.0, 0.3, 0.7)));
    }

}