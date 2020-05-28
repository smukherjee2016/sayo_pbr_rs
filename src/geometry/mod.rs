use crate::common::*;

pub mod triangle;

pub trait Hitable: Send + Sync {
    fn check_intersection_and_return_closest_hit(
        &self,
        ray: Ray,
        t_min: fp,
        t_max: fp,
    ) -> Option<IntersectionInfo>;
}
