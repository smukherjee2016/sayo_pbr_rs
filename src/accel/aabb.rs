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
        //https://tavianator.com/fast-branchless-raybounding-box-intersections-part-2-nans/
        //TODO Unrolled the whole loop since indexing not supported yet for Vector3. Will this lead to issues?
        let mut t1: fp = (self.min.x - ray.o.x) * ray.inv_dir.x;
        let mut t2: fp = (self.max.x - ray.o.x) * ray.inv_dir.x;

        let mut tmin: fp = fp::min(t1, t2);
        let mut tmax: fp = fp::max(t1, t2);

        t1 = (self.min.y - ray.o.y) * ray.inv_dir.y;
        t2 = (self.max.y - ray.o.y) * ray.inv_dir.y;

        tmin = fp::max(tmin, fp::min(fp::min(t1, t2), tmax));
        tmax = fp::min(tmax, fp::max(fp::max(t1, t2), tmin));

        t1 = (self.min.z - ray.o.z) * ray.inv_dir.z;
        t2 = (self.max.z - ray.o.z) * ray.inv_dir.z;

        tmin = fp::max(tmin, fp::min(fp::min(t1, t2), tmax));
        tmax = fp::min(tmax, fp::max(fp::max(t1, t2), tmin));

        if tmax >= fp::max(tmin, 0.0) {
            let intersection_info = IntersectionInfo {
                t_intersection: 0.0,
                point_of_intersection: Point3::from(0.0),
                normal: Vec3::from(0.0),
                is_aabb: true,
            };
            return Some(intersection_info);
        }
        None
    }
}

//Objects that can show an axis-aligned bounding box around themselves
//Anything that's boundable must be hitable as well
pub trait Boundable: Hitable {
    fn get_bounding_box(t0: fp, t1: fp) -> AxisAlignedBoundingBox;
}
