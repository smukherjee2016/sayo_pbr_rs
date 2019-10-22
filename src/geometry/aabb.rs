use crate::common::*;
use crate::geometry::Hitable;

pub struct AABB {


}

impl Hitable for AABB {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        unimplemented!()
    }
}