use crate::common::*;
use crate::geometry::Hitable;
use std::path::PathBuf;

#[derive(Clone)]
pub struct TriangleMesh {
    //Same as tobj::Mesh
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>,
    pub indices: Vec<u32>,
    pub material_id: Option<usize>,
}

pub struct Triangle<'a> {
    indices: Vec<u128>,
    mesh: &'a TriangleMesh,
}

impl TriangleMesh {
    pub fn new(mesh_name_and_path: PathBuf) -> Vec<TriangleMesh> {
        //Load in the .obj file. It might have multiple models(meshes) in it
        let obj_mesh = tobj::load_obj(mesh_name_and_path.as_path());
        assert!(obj_mesh.is_ok());
        let (models, materials) = obj_mesh.unwrap();
        let mut meshes: Vec<TriangleMesh> = Vec::new();
        for model in models {
            let mesh = TriangleMesh {
                positions: model.mesh.positions,
                normals: model.mesh.normals,
                texcoords: model.mesh.texcoords,
                indices: model.mesh.indices,
                material_id: model.mesh.material_id,
            };
            meshes.push(mesh);
        }

        meshes
    }

    pub fn get_triangles_from_mesh(&self) -> Vec<Triangle> {
        vec![Triangle {
            indices: vec![],
            // mesh: Arc::new(*self)
            mesh: &self
        }]
    }
}


impl<'a> Hitable for Triangle<'a> {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        let intersection_info = IntersectionInfo {
            t_intersection: 0.0,
            point_of_intersection: ZERO_VEC3,
            normal: ZERO_VEC3,
        };

        Some(intersection_info)
    }
}
