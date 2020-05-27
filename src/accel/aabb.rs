use crate::common::*;
use crate::geometry::Hitable;
use std::rc::Rc;
use std::sync::Arc;

//Rectangular AABB, defined by two points of its diagonal
#[derive(Default, Debug, Clone)]
pub struct AxisAlignedBoundingBox {
    pub min: Point3,
    pub max: Point3,
}

impl AxisAlignedBoundingBox {
    pub fn new_aabb(_min: Point3, _max: Point3) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            min: _min,
            max: _max,
        }
    }

    pub fn area_aabb(self) -> f64 {
        let extent = self.max - self.min;
        2.0 * (extent.x * extent.y + extent.y * extent.z + extent.z * extent.x)
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
    fn get_bounding_box(&self) -> AxisAlignedBoundingBox;
}

impl Boundable for AxisAlignedBoundingBox {
    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.clone()
    }
}

// Return surrounding box of two AABB's
pub fn surrounding_box(
    a: &AxisAlignedBoundingBox,
    b: &AxisAlignedBoundingBox,
) -> AxisAlignedBoundingBox {
    let small: Vector3 = Vector3::new(
        fp::min(a.min.x, b.min.x),
        fp::min(a.min.y, b.min.y),
        fp::min(a.min.z, b.min.z),
    );

    let big: Vector3 = Vector3::new(
        fp::max(a.max.x, b.max.x),
        fp::max(a.max.y, b.max.y),
        fp::max(a.max.z, b.max.z),
    );

    AxisAlignedBoundingBox {
        min: small,
        max: big,
    }
}

pub fn surrounding_box_primitives(primitives_vector : Vec<Arc<dyn Boundable>>) -> AxisAlignedBoundingBox {
    let mut ret_aabb = AxisAlignedBoundingBox::default();
    for primitive in &primitives_vector {
        let bounding_box_primitive : AxisAlignedBoundingBox = primitive.get_bounding_box();
        ret_aabb = surrounding_box(&bounding_box_primitive, &ret_aabb);
    }
    ret_aabb
}
