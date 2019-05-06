use crate::common::*;
use crate::geometry::Geometry;
use std::path::PathBuf;

pub struct TriangleMesh {
    triangles : Vec<Triangle>
}

struct Triangle {

}

impl TriangleMesh {
    pub fn new(mesh : PathBuf) -> TriangleMesh {
        TriangleMesh{ triangles: vec![] }
    }
}

impl Geometry for TriangleMesh {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        let intersection_info = IntersectionInfo {
            t_intersection: 0.0,
            point_of_intersection: ZERO_VEC3,
            normal: ZERO_VEC3
        };

        Some(intersection_info)
    }
}