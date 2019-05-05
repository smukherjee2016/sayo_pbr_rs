pub use cgmath::prelude::*;
pub use f64 as fp;
pub type Color = cgmath::Vector3<fp>;
pub type Point3 = cgmath::Vector3<fp>;
pub type Vec3 = cgmath::Vector3<fp>;
pub const EPSILON : fp = 1e-5;
pub const ZERO_VECTOR: cgmath::Vector3<fp> = cgmath::Vector3{x: 0.0, y: 0.0, z :0.0};

pub struct Ray {
    o : Point3,
    d: Vec3,
    t : fp,
    tmax : fp
}

impl Ray {
    pub fn new(origin_ : Point3, direction_ : Vec3, t_ : fp, tmax_ : fp) -> Ray{
        Ray {
            o: origin_,
            d: direction_,
            t: t_,
            tmax: tmax_
        }
    }
}