use crate::common::*;

mod triangle;

pub trait Geometry {
    fn check_intersection_and_return_closest_hit(ray : Ray) -> Option<IntersectionInfo>;
}