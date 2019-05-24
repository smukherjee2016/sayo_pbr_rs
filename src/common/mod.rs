pub use cgmath::prelude::*;
pub use f64 as fp;
pub use log::{info, trace, warn};

pub type Spectrum = cgmath::Vector3<fp>;
pub type Point3 = cgmath::Vector3<fp>;
pub type Point2 = cgmath::Vector2<fp>;
pub type Vec3 = cgmath::Vector3<fp>;
pub type Mat4 = cgmath::Matrix4<fp>;
pub const EPSILON: fp = 1e-5;
pub const ZERO_VEC3: cgmath::Vector3<fp> = cgmath::Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const ZERO_VEC4: cgmath::Vector4<fp> = cgmath::Vector4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

pub const ZERO_MAT4: cgmath::Matrix4<fp> = cgmath::Matrix4 {
    x: ZERO_VEC4,
    y: ZERO_VEC4,
    z: ZERO_VEC4,
    w: ZERO_VEC4,
};

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Point3,
    pub d: Vec3,
    pub t: fp,
    pub tmax: fp,
}

#[derive(Debug)]
pub struct IntersectionInfo {
    pub t_intersection: fp,
    pub point_of_intersection: Point3,
    pub normal: Vec3,
}

impl Ray {
    pub fn new(origin_: Point3, direction_: Vec3, t_: fp, tmax_: fp) -> Ray {
        Ray {
            o: origin_,
            d: direction_,
            t: t_,
            tmax: tmax_,
        }
    }
}
