use crate::common::*;
use cgmath::Vector3;

#[derive(Debug, Default)]
pub struct Film {
    pub image : Vec<Color>,
    pub height: i32,
    pub width : i32,
    pub fov : fp
}

impl Film {
    pub fn new(&mut self, width : i32, height : i32) {
        self.width = width;
        self.height = height;
        self.image.resize((width * height) as usize, Vector3::from((0.0, 0.3, 0.7)));
    }

}