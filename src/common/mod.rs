pub use crate::utilities::mathutils::*;
pub use log::{info, trace, warn};

pub const EPSILON: fp = 1e-5;
pub const TILE_SIZE: usize = 256;

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Point3,
    pub d: Vec3,
    pub t: fp,
    pub tmax: fp,
    pub inv_dir: Vec3,
}

#[derive(Debug, Default)]
pub struct IntersectionInfo {
    pub t_intersection: fp,
    pub point_of_intersection: Point3,
    pub normal: Vec3,
    pub is_aabb: bool,
}

impl Ray {
    pub fn new(origin_: Point3, direction: Vec3, t_: fp, tmax_: fp) -> Ray {
        let inv_dir: Vec3 = Vec3 {
            x: 1.0 / direction.x,
            y: 1.0 / direction.y,
            z: 1.0 / direction.z,
        };
        Ray {
            o: origin_,
            d: direction,
            t: t_,
            tmax: tmax_,
            inv_dir,
        }
    }
}
