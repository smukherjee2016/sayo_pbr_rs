use crate::common::*;
use crate::geometry::Hitable;

//Rectangular AABB, defined by two points of its diagonal
pub struct AxisAlignedBoundingBox {
    min: Point3,
    max: Point3,
}

impl AxisAlignedBoundingBox {
    pub fn new_aabb(&mut self, _min: Point3, _max: Point3) {
        self.min = _min;
        self.max = _max;
    }
}

impl Hitable for AxisAlignedBoundingBox {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        unimplemented!()
    }
}

//Objects that can show an axis-aligned bounding box around themselves
//Anything that's boundable must be hitable as well
pub trait Boundable: Hitable {
    fn get_bounding_box(t0: fp, t1: fp) -> AxisAlignedBoundingBox;
}
