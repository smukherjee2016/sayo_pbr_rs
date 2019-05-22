use crate::common::*;
use crate::geometry::Hitable;
use std::path::PathBuf;

pub struct TriangleMesh {
    //Same as tobj::Mesh
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>,
    pub indices: Vec<u32>,
    pub material_id: Option<usize>,
}

pub struct Triangle {
    //TODO Visit this implementation someday after leveling up
    //indices: Vec<u128>,
    //mesh: Arc<TriangleMesh>,
    pub positions: Vec<Point3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Point2>,
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
        let mut triangles: Vec<Triangle> = vec![];

        //Convert the indices to groups of 3
        let triangle_indices: Vec<Vec<u32>> = self.indices.chunks(3).map(|x| x.to_vec()).collect();
        for index in &triangle_indices {
            let triangle = Triangle {
                positions: vec![Point3::new(
                    *self.positions.get(index[0] as usize).unwrap() as fp,
                    *self.positions.get(index[1] as usize).unwrap() as fp,
                    *self.positions.get(index[2] as usize).unwrap() as fp,
                )],
                normals: vec![Point3::new(
                    *self.normals.get(index[0] as usize).unwrap() as fp,
                    *self.normals.get(index[1] as usize).unwrap() as fp,
                    *self.normals.get(index[2] as usize).unwrap() as fp,
                )],
                texcoords: vec![Point2::new(
                    *self.texcoords.get(index[0] as usize).unwrap() as fp,
                    *self.texcoords.get(index[1] as usize).unwrap() as fp,
                )],
            };
            triangles.push(triangle);
        }

        //info!("Triangle 0: Positions: {:?}", triangles.get(0).unwrap().texcoords);

        triangles
    }
}

impl Hitable for Triangle {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        let intersection_info = IntersectionInfo {
            t_intersection: 0.0,
            point_of_intersection: ZERO_VEC3,
            normal: ZERO_VEC3,
        };

        Some(intersection_info)
    }
}
