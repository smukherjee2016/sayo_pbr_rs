mod pinholecamera;
use crate::common::*;

pub trait Camera {
    fn generate_camera_ray() -> Ray;
}