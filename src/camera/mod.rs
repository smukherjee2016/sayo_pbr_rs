pub mod pinholecamera;
use crate::common::*;
use crate::film::Film;

pub trait Camera {
    fn new(origin_: Point3, look_at: Point3, up_: Vec3) -> Self
    where
        Self: Sized;
    fn generate_camera_ray(&self, x: i32, y: i32, film: &Film) -> Ray;
}
