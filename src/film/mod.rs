use crate::common::*;

#[derive(Debug, Default, Clone)]
pub struct Film {
    pub height: i32,
    pub width: i32,
    pub fov: fp,
    pub distance_to_film: fp,
    pub aspect_ratio: fp,
}

impl Film {
    pub fn new_film(&mut self, width: i32, height: i32, fov_degrees: fp) {
        self.distance_to_film = 1.0;
        self.aspect_ratio = fp::from(width) / fp::from(height);
        self.fov = fov_degrees * std::f64::consts::PI / 180.0;

        self.width = width;
        self.height = height;
    }
}
