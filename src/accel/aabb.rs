use crate::common::*;
use crate::geometry::Hitable;
use std::sync::Arc;

//Rectangular AABB, defined by two points of its diagonal
#[derive(Debug, Clone)]
pub struct AxisAlignedBoundingBox {
    pub min: Point3,
    pub max: Point3,
}

impl Default for AxisAlignedBoundingBox {
    fn default() -> Self {
        AxisAlignedBoundingBox {
            min: Point3::from(fp::MAX),
            max: Point3::from(fp::MIN),
        }
    }
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

    pub fn longest_axis(self) -> i32 {
        let max_x: fp = self.max.x - self.min.x;
        let max_y: fp = self.max.y - self.min.y;
        let max_z: fp = self.max.z - self.min.z;
        if max_y > max_x {
            if max_z > max_y {
                2
            } else {
                1
            }
        } else if max_z > max_x {
            2
        } else {
            0
        }
    }
}

impl Hitable for AxisAlignedBoundingBox {
    fn check_intersection_and_return_closest_hit(
        &self,
        ray: Ray,
        _t_min: fp,
        _t_max: fp,
    ) -> Option<IntersectionInfo> {
        //     let mut t_min = t_min;
        //     let mut t_max = t_max;
        //     for a in 0..3 {
        //         //let inv_d : fp = 1.0 / ray.d[a];
        //         let mut t_0: fp = (self.min[a] - ray.o[a]) * ray.inv_dir[a];
        //         let mut t_1: fp = (self.max[a] - ray.o[a]) * ray.inv_dir[a];
        //         if ray.inv_dir[a] < 0.0 {
        //             swap(&mut t_0, &mut t_1);
        //         }
        //
        //         t_min = if t_0 > t_min { t_0 } else { t_min };
        //         t_max = if t_1 < t_max { t_1 } else { t_max };
        //         if t_max < t_min {
        //             return None;
        //         }
        //     }
        //     let intersection_info = IntersectionInfo {
        //         t_intersection: 0.0,
        //         point_of_intersection: Point3::from(0.0),
        //         normal: Vec3::from(0.0),
        //         is_aabb: true,
        //     };
        //     Some(intersection_info)
        // }
        // fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        //  //https://tavianator.com/fast-branchless-raybounding-box-intersections-part-2-nans/
        // Ray-AABB intersection, 8c16t: 200ns max
        // let start = Instant::now();
        let mut t_1: fp = (self.min[0] - ray.o[0]) * ray.inv_dir[0];
        let mut t_2: fp = (self.max[0] - ray.o[0]) * ray.inv_dir[0];

        let mut t_min = fp::min(t_1, t_2);
        let mut t_max = fp::max(t_1, t_2);

        for i in 1..3 {
            t_1 = (self.min[i] - ray.o[i]) * ray.inv_dir[i];
            t_2 = (self.max[i] - ray.o[i]) * ray.inv_dir[i];

            //NaN handling version
            t_min = fp::max(t_min, fp::min(fp::min(t_1, t_2), t_max));
            t_max = fp::min(t_max, fp::max(fp::max(t_1, t_2), t_min));

            //Non-NAN handling version
            //t_min = fp::max(t_min, fp::min(t_1, t_2));
            //t_max = fp::min(t_max, fp::max(t_1, t_2));
        }
        //warn!("Time elapsed for AABB intersection: {:?}", start.elapsed());
        if t_max > fp::max(t_min, 0.0) {
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

pub fn surrounding_box_primitives(
    primitives_vector: Vec<Arc<dyn Boundable>>,
) -> AxisAlignedBoundingBox {
    let mut ret_aabb = primitives_vector.get(0).unwrap().get_bounding_box();
    for primitive in &primitives_vector {
        let bounding_box_primitive: AxisAlignedBoundingBox = primitive.get_bounding_box();
        ret_aabb = surrounding_box(&bounding_box_primitive, &ret_aabb);
    }
    ret_aabb
}
