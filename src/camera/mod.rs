pub mod pinholecamera;
use crate::common::*;
use crate::film::Film;
use std::rc::Rc;
use std::fmt::{Debug, Formatter, Error};
use crate::camera::pinholecamera::PinholeCamera;

pub trait Camera {
    fn generate_camera_ray(&mut self, x : i32, y : i32, film : Rc<Film>) -> Ray;
    fn new(origin_ : Point3, look_at: Point3, up_ : Vec3) -> Self where Self: Sized;
}

