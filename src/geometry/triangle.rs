use crate::common::*;
use crate::geometry::Geometry;

struct TriangleMesh {
    triangles : Vec<Triangle>
}

struct Triangle {

}

impl Geometry for Triangle {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        let intersection_info = IntersectionInfo {
            t_intersection: 0.0,
            point_of_intersection: ZERO_VECTOR,
            normal: ZERO_VECTOR
        };

        Some(intersection_info)
    }
}