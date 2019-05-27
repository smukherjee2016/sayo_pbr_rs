pub use log::{info, trace, warn};
pub use crate::utilities::mathutils::*;

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
