use crate::common::*;

pub mod triangle;
pub mod aabb;

pub trait Hitable {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo>;
}
