use crate::common::*;
use crate::accel::aabb::Boundable;

pub mod triangle;

pub trait Hitable : Send + Sync {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo>;
}
