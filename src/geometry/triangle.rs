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

        //*self.positions.get(index[0] as usize).unwrap() as fp
        //Convert the indices to groups of 3
        for v in 0..self.indices.len() / 3 {
            let index_0_of_triangle = self.indices[3 * v] as usize;
            let index_1_of_triangle = self.indices[3 * v + 1] as usize;
            let index_2_of_triangle = self.indices[3 * v + 2] as usize;

            let triangle = Triangle {
                positions: vec![
                    Point3::new(
                        self.positions[3 * index_0_of_triangle] as fp,
                        self.positions[3 * index_0_of_triangle + 1] as fp,
                        self.positions[3 * index_0_of_triangle + 2] as fp,
                    ),
                    Point3::new(
                        self.positions[3 * index_1_of_triangle] as fp,
                        self.positions[3 * index_1_of_triangle + 1] as fp,
                        self.positions[3 * index_1_of_triangle + 2] as fp,
                    ),
                    Point3::new(
                        self.positions[3 * index_2_of_triangle] as fp,
                        self.positions[3 * index_2_of_triangle + 1] as fp,
                        self.positions[3 * index_2_of_triangle + 2] as fp,
                    ),
                ],
                normals: vec![
                    Vector3::new(
                        self.normals[3 * index_0_of_triangle] as fp,
                        self.normals[3 * index_0_of_triangle + 1] as fp,
                        self.normals[3 * index_0_of_triangle + 2] as fp,
                    ),
                    Vector3::new(
                        self.normals[3 * index_1_of_triangle] as fp,
                        self.normals[3 * index_1_of_triangle + 1] as fp,
                        self.normals[3 * index_1_of_triangle + 2] as fp,
                    ),
                    Vector3::new(
                        self.normals[3 * index_2_of_triangle] as fp,
                        self.normals[3 * index_2_of_triangle + 1] as fp,
                        self.normals[3 * index_2_of_triangle + 2] as fp,
                    ),
                ],
                texcoords: vec![
                    Point2::new(
                        self.texcoords[2 * index_0_of_triangle] as fp,
                        self.texcoords[2 * index_0_of_triangle + 1] as fp,
                    ),
                    Point2::new(
                        self.texcoords[2 * index_1_of_triangle] as fp,
                        self.texcoords[2 * index_1_of_triangle + 1] as fp,
                    ),
                    Point2::new(
                        self.texcoords[2 * index_2_of_triangle] as fp,
                        self.texcoords[2 * index_2_of_triangle + 1] as fp,
                    ),
                ],
            };
            /*
            warn!(" Positions of triangle {} : {} {} {}", v, index_0_of_triangle,
                  index_1_of_triangle, index_2_of_triangle);
            warn!("    v[{}] = ({}, {}, {})", v,  self.positions[3*index_0_of_triangle],
                  self.positions[3*index_0_of_triangle + 1], self.positions[3*index_0_of_triangle + 2]);
            warn!("    v[{}] = ({}, {}, {})", v,  self.positions[3*index_1_of_triangle],
                  self.positions[3*index_1_of_triangle + 1], self.positions[3*index_1_of_triangle + 2]);
            warn!("    v[{}] = ({}, {}, {})", v,  self.positions[3*index_2_of_triangle],
                  self.positions[3*index_2_of_triangle + 1], self.positions[3*index_2_of_triangle + 2]);
            */
            triangles.push(triangle);
        }

        //info!("Size of triangles: {}", triangles.len());
        triangles
    }
}

impl Hitable for Triangle {
    fn check_intersection_and_return_closest_hit(&self, ray: Ray) -> Option<IntersectionInfo> {
        let intersection_info = IntersectionInfo {
            t_intersection: 0.0,
            point_of_intersection: Default::default(),
            normal: Vector3::new(0.1, 0.4, 0.9),
        };

        Some(intersection_info)
    }
}
