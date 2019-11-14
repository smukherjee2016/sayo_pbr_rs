use crate::common::*;

pub mod aabb;
pub mod triangle;

pub trait Hitable {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo>;
}
