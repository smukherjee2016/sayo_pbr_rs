use crate::common::*;
use crate::geometry::Geometry;
use std::path::PathBuf;

pub struct TriangleMesh {
    triangles: Vec<Triangle>,
}

struct Triangle {}

impl TriangleMesh {
    pub fn new(mesh_name_and_path: PathBuf) -> TriangleMesh {
        let mesh = tobj::load_obj(mesh_name_and_path.as_path());
        assert!(mesh.is_ok());
        let (models, materials) = mesh.unwrap();
        info!("Number of models: {}", models.len());

        TriangleMesh{ triangles: vec![] }
    }
}

impl Geometry for TriangleMesh {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        let intersection_info = IntersectionInfo {
            t_intersection: 0.0,
            point_of_intersection: ZERO_VEC3,
            normal: ZERO_VEC3,
        };

        Some(intersection_info)
    }
}
